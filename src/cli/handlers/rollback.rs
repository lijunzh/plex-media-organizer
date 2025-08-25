//! Rollback command handler for undoing previous organization operations

use anyhow::Result;
use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;

use crate::{
    config::AppConfig,
    database::operations::OperationHistoryManager,
    output::{print_section_header, print_subsection_header},
};
use serde::{Deserialize, Serialize};

#[derive(Args, Debug)]
pub struct RollbackArgs {
    /// Operation file to rollback
    #[arg(value_name = "OPERATION_FILE")]
    operation_file: PathBuf,

    /// Preview rollback changes (dry-run)
    #[arg(short, long)]
    preview: bool,

    /// Show detailed output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RollbackResult {
    pub operation_file: PathBuf,
    pub total_files: usize,
    pub rolled_back_files: Vec<RolledBackFile>,
    pub failed_files: Vec<FailedRollback>,
    pub rollback_stats: RollbackStats,
    pub operation_duration: std::time::Duration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RolledBackFile {
    pub original_path: PathBuf,
    pub restored_path: PathBuf,
    pub operation_type: OperationType,
    pub file_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FailedRollback {
    pub path: PathBuf,
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RollbackStats {
    pub successful_rollbacks: usize,
    pub failed_rollbacks: usize,
    pub total_size_restored: u64,
    pub operation_duration: std::time::Duration,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OperationType {
    Move,
    Rename,
    Copy,
}

/// Handle rollback command
pub async fn handle_rollback(args: RollbackArgs) -> Result<()> {
    let _rollback_start = std::time::Instant::now();

    print_section_header("🔄 Media File Rollback");
    println!("📁 Operation file: {}", args.operation_file.display());
    println!("🧪 Preview mode: {}", args.preview);
    println!("📝 Verbose output: {}", args.verbose);

    // Load configuration
    let config = AppConfig::load()?;
    println!("✓ Configuration loaded");

    // Validate operation file exists
    if !args.operation_file.exists() {
        anyhow::bail!(
            "Operation file not found: {}",
            args.operation_file.display()
        );
    }

    // Load operation data from database or file
    // Load operation data from database using operation ID
    let operation_data =
        load_operation_from_database(&args.operation_file.to_string_lossy(), &config)?;
    println!("✓ Operation data loaded from database");
    println!(
        "📊 Original operation: {} files organized",
        operation_data.organized_files.len()
    );

    // Perform rollback
    let rollback_result = perform_rollback(&args, &operation_data).await?;

    // Display results
    display_rollback_results(&rollback_result, &args);

    // Save rollback log if not in preview
    if !args.preview {
        save_rollback_log(&rollback_result).await?;
        println!("✓ Rollback log saved");
    }

    println!("✅ Rollback operation completed!");
    Ok(())
}

/// Load operation data from database
fn load_operation_from_database(
    operation_id: &str,
    config: &crate::config::AppConfig,
) -> Result<crate::cli::handlers::organize::OrganizationResult> {
    // Create database connection
    let db_path = config.database.path.clone();
    let conn = rusqlite::Connection::open(&db_path)?;

    // Get operation manager
    let operation_manager = OperationHistoryManager::new(conn);

    // Get operation record
    let operation_record = operation_manager
        .get_operation(operation_id)?
        .ok_or_else(|| anyhow::anyhow!("Operation not found: {}", operation_id))?;

    // Get operation files
    let operation_files = operation_manager.get_operation_files(operation_id)?;

    // Convert to OrganizationResult
    let organized_files = operation_files
        .into_iter()
        .map(
            |file_record| crate::cli::handlers::organize::OrganizedFile {
                original_path: file_record.original_path.clone(),
                new_path: file_record.new_path.clone(),
                media_file: crate::types::MediaFile {
                    id: format!("file_{}", uuid::Uuid::new_v4()),
                    file_path: file_record.original_path.clone(),
                    file_name: file_record
                        .original_path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                    file_size: file_record.file_size,
                    media_type: crate::types::MediaType::Video,
                    content_hash: format!(
                        "{:x}",
                        md5::compute(file_record.original_path.to_string_lossy().as_bytes())
                    ),
                    last_modified: chrono::Utc::now(),
                    metadata: crate::types::MediaMetadata {
                        title: None,
                        original_title: None,
                        year: None,
                        language: vec![],
                        quality: None,
                        source: None,
                        duration: None,
                        resolution: None,
                        codec: None,
                        audio_tracks: vec![],
                        subtitle_tracks: vec![],
                    },
                },
                operation_type: match file_record.operation_type.as_str() {
                    "Move" => crate::cli::handlers::organize::OperationType::Move,
                    "Rename" => crate::cli::handlers::organize::OperationType::Rename,
                    "Copy" => crate::cli::handlers::organize::OperationType::Copy,
                    _ => crate::cli::handlers::organize::OperationType::Move,
                },
            },
        )
        .collect();

    Ok(crate::cli::handlers::organize::OrganizationResult {
        total_files: operation_record.total_files,
        organized_files,
        skipped_files: vec![],
        failed_files: vec![],
        organization_stats: crate::cli::handlers::organize::OrganizationStats {
            successful_organizations: operation_record.successful_files,
            skipped_files: operation_record.skipped_files,
            failed_files: operation_record.failed_files,
            total_size_moved: operation_record.total_size_moved.unwrap_or(0),
            average_confidence: operation_record.average_confidence.unwrap_or(0.0),
            operation_duration: std::time::Duration::from_secs(1), // Default duration
        },
        operation_duration: std::time::Duration::from_secs(1), // Default duration
    })
}

/// Perform the rollback operation
async fn perform_rollback(
    args: &RollbackArgs,
    operation_data: &crate::cli::handlers::organize::OrganizationResult,
) -> Result<RollbackResult> {
    let mut rolled_back_files = Vec::new();
    let mut failed_files = Vec::new();
    let mut total_size_restored = 0u64;

    // Create progress bar
    let progress_bar = if !args.verbose {
        let pb = ProgressBar::new(operation_data.organized_files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
        Some(pb)
    } else {
        None
    };

    for organized_file in &operation_data.organized_files {
        match rollback_single_file(organized_file, args.preview).await {
            Ok(Some(rolled_back_file)) => {
                total_size_restored += rolled_back_file.file_size;
                rolled_back_files.push(rolled_back_file);
            }
            Ok(None) => {
                // File was already in correct location
            }
            Err(e) => {
                failed_files.push(FailedRollback {
                    path: organized_file.original_path.clone(),
                    error: e.to_string(),
                });
            }
        }

        if let Some(ref pb) = progress_bar {
            pb.inc(1);
        }
    }

    if let Some(pb) = progress_bar {
        pb.finish_with_message("Rollback completed");
    }

    let operation_duration = std::time::Instant::now().elapsed();

    let stats = RollbackStats {
        successful_rollbacks: rolled_back_files.len(),
        failed_rollbacks: failed_files.len(),
        total_size_restored,
        operation_duration,
    };

    Ok(RollbackResult {
        operation_file: args.operation_file.clone(),
        total_files: operation_data.organized_files.len(),
        rolled_back_files,
        failed_files,
        rollback_stats: stats,
        operation_duration,
    })
}

/// Rollback a single file
async fn rollback_single_file(
    organized_file: &crate::cli::handlers::organize::OrganizedFile,
    preview: bool,
) -> Result<Option<RolledBackFile>> {
    // Check if file exists at new location
    if !organized_file.new_path.exists() {
        return Err(anyhow::anyhow!(
            "File not found at new location: {}",
            organized_file.new_path.display()
        ));
    }

    // Check if original location is available
    if organized_file.original_path.exists() {
        return Err(anyhow::anyhow!(
            "Original file still exists: {}",
            organized_file.original_path.display()
        ));
    }

    // Get file metadata
    let metadata = std::fs::metadata(&organized_file.new_path)?;
    let file_size = metadata.len();

    // Determine operation type for rollback
    let rollback_operation_type = match organized_file.operation_type {
        crate::cli::handlers::organize::OperationType::Move => OperationType::Move,
        crate::cli::handlers::organize::OperationType::Rename => OperationType::Rename,
        crate::cli::handlers::organize::OperationType::Copy => OperationType::Copy,
    };

    // Create parent directory if it doesn't exist
    if let Some(parent) = organized_file.original_path.parent() {
        if !parent.exists() && !preview {
            std::fs::create_dir_all(parent)?;
        }
    }

    // Perform the rollback operation
    if !preview {
        match rollback_operation_type {
            OperationType::Move | OperationType::Rename => {
                std::fs::rename(&organized_file.new_path, &organized_file.original_path)?;
            }
            OperationType::Copy => {
                std::fs::copy(&organized_file.new_path, &organized_file.original_path)?;
            }
        }
    }

    Ok(Some(RolledBackFile {
        original_path: organized_file.original_path.clone(),
        restored_path: organized_file.new_path.clone(),
        operation_type: rollback_operation_type,
        file_size,
    }))
}

/// Display rollback results
fn display_rollback_results(result: &RollbackResult, args: &RollbackArgs) {
    print_section_header("📋 Rollback Results");

    println!("📊 Overall Statistics:");
    println!("  Total files processed: {}", result.total_files);
    println!(
        "  Successfully rolled back: {}",
        result.rollback_stats.successful_rollbacks
    );
    println!(
        "  Failed rollbacks: {}",
        result.rollback_stats.failed_rollbacks
    );
    println!(
        "  Success rate: {:.1}%",
        if result.total_files > 0 {
            (result.rollback_stats.successful_rollbacks as f64 / result.total_files as f64) * 100.0
        } else {
            0.0
        }
    );
    println!(
        "  Total size restored: {:.1} MB",
        result.rollback_stats.total_size_restored as f64 / 1024.0 / 1024.0
    );
    println!(
        "  Operation duration: {:?}",
        result.rollback_stats.operation_duration
    );

    if args.verbose && !result.rolled_back_files.is_empty() {
        print_subsection_header("📋 Rolled Back Files");
        for file in &result.rolled_back_files {
            println!(
                "  ✅ {} -> {}",
                file.restored_path.display(),
                file.original_path.display()
            );
        }
    }

    if !result.failed_files.is_empty() {
        print_subsection_header("❌ Failed Rollbacks");
        for file in &result.failed_files {
            println!("  ❌ {}: {}", file.path.display(), file.error);
        }
    }

    if args.preview {
        print_subsection_header("📋 PREVIEW SUMMARY");
        println!("This is a PREVIEW of what would happen:");
        println!(
            "• {} files would be rolled back",
            result.rollback_stats.successful_rollbacks
        );
        println!(
            "• {} files would fail to rollback",
            result.rollback_stats.failed_rollbacks
        );
        println!(
            "• Operation type: {}",
            if result.rollback_stats.successful_rollbacks > 0 {
                "RESTORE files to original locations"
            } else {
                "No operations"
            }
        );
        println!();
        println!("💡 To actually perform these operations, run without --preview");
        println!("⚠️  WARNING: This will move files back to their original locations!");
    }
}

/// Save rollback log
async fn save_rollback_log(result: &RollbackResult) -> Result<()> {
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let log_filename = format!("rollback_{}.json", timestamp);

    let log_content = serde_json::to_string_pretty(result)?;
    std::fs::write(&log_filename, log_content)?;

    println!("📝 Rollback log saved: {}", log_filename);
    Ok(())
}
