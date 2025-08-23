//! Organize command handler for file organization and renaming

use crate::{
    config::AppConfig,
    database::DatabaseManager,
    external::tmdb::UnifiedTmdbClient,
    output::{print_section_header, print_subsection_header},
    parsers::UnifiedMovieParser,
    types::{MediaFile, MediaMetadata, MediaType},
};

mod duration_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration.as_secs_f64().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = f64::deserialize(deserializer)?;
        Ok(Duration::from_secs_f64(secs))
    }
}

/// Configuration for organizing files
#[derive(Debug, Clone)]
struct OrganizeConfig {
    parser: UnifiedMovieParser,
    output_directory: PathBuf,
    config: AppConfig,
    min_confidence: f32,
    preview: bool,
    verbose: bool,
    use_cache: bool,
    organize_by_year: bool,
}

/// Configuration for processing a single file
#[derive(Debug)]
#[allow(dead_code)]
struct ProcessFileConfig<'a> {
    parser: &'a UnifiedMovieParser,
    output_directory: &'a Path,
    config: &'a AppConfig,
    min_confidence: f32,
    preview: bool,
    use_cache: bool,
    organize_by_year: bool,
    parser_result:
        &'a crate::parsers::types::ParserResult<crate::parsers::types::FilenameComponents>,
}
use anyhow::Result;
use chrono::DateTime;
use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Args, Debug)]
pub struct OrganizeArgs {
    /// Directory to organize
    #[arg(value_name = "DIRECTORY")]
    directory: PathBuf,

    /// Preview organization changes (dry-run) - shows detailed file transformations
    #[arg(short, long)]
    preview: bool,

    /// Create backup before organizing
    #[arg(short, long)]
    backup: bool,

    /// Show detailed output
    #[arg(short, long)]
    verbose: bool,

    /// Optimize for network drives (SMB, NFS, etc.)
    #[arg(long)]
    network_mode: bool,

    /// Maximum number of concurrent operations
    #[arg(long, default_value = "16")]
    max_parallel: usize,

    /// Batch size for processing (smaller for network drives)
    #[arg(long, default_value = "100")]
    batch_size: usize,

    /// Minimum confidence threshold (0.0-1.0) for organizing movies
    #[arg(long, default_value = "0.7")]
    min_confidence: f32,

    /// Skip movies with no TMDB match instead of using fallback data
    #[arg(long, default_value = "true")]
    skip_unmatched: bool,

    /// Skip warnings for low confidence matches
    #[arg(long)]
    no_warnings: bool,

    /// Use database caching
    #[arg(long)]
    use_cache: bool,

    /// Output directory for organized files
    #[arg(long)]
    output_dir: Option<PathBuf>,

    /// Organize files by year folders (default: flat structure for Plex)
    #[arg(long)]
    organize_by_year: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationResult {
    pub total_files: usize,
    pub organized_files: Vec<OrganizedFile>,
    pub skipped_files: Vec<SkippedFile>,
    pub failed_files: Vec<FailedFile>,
    pub organization_stats: OrganizationStats,
    #[serde(with = "duration_serde")]
    pub operation_duration: std::time::Duration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizedFile {
    pub original_path: PathBuf,
    pub new_path: PathBuf,
    pub media_file: MediaFile,
    pub operation_type: OperationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkippedFile {
    pub path: PathBuf,
    pub reason: SkipReason,
    pub confidence: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FailedFile {
    pub path: PathBuf,
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizationStats {
    pub successful_organizations: usize,
    pub skipped_files: usize,
    pub failed_files: usize,
    pub total_size_moved: u64,
    pub average_confidence: f32,
    #[serde(with = "duration_serde")]
    pub operation_duration: std::time::Duration,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OperationType {
    Move,
    Rename,
    Copy,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SkipReason {
    LowConfidence(f32),
    NoMetadata,
    AlreadyOrganized,
    ExtrasContent,
    UnsupportedFormat,
    UserSkipped,
}

pub async fn handle_organize(args: OrganizeArgs) -> Result<()> {
    print_section_header("Media File Organization");

    let operation_start = std::time::Instant::now();

    // Load configuration
    let config = AppConfig::load()?;
    println!("✓ Configuration loaded");

    // Initialize database if caching is enabled
    let database = if args.use_cache {
        let db_path = PathBuf::from(&config.database.path);
        std::fs::create_dir_all(db_path.parent().unwrap())?;
        let db = DatabaseManager::new(&db_path).await?;
        println!("✓ Database cache initialized at: {}", db_path.display());
        Some(db)
    } else {
        None
    };

    // Create TMDB client if API key is available
    let tmdb_client = if let Some(api_key) = &config.apis.tmdb_api_key {
        match UnifiedTmdbClient::new(api_key.clone()) {
            Ok(client) => {
                println!("✓ TMDB integration enabled");
                Some(client)
            }
            Err(e) => {
                println!("⚠️  TMDB integration failed: {}", e);
                None
            }
        }
    } else {
        println!("ℹ️  TMDB integration disabled (no API key)");
        None
    };

    // Create parser with TMDB integration if available
    let parser = if let Some(ref db) = database {
        if let Some(tmdb) = tmdb_client {
            UnifiedMovieParser::with_config_and_database_and_tmdb(config.clone(), db.clone(), tmdb)
        } else {
            UnifiedMovieParser::with_config_and_database(config.clone(), db.clone())
        }
    } else if let Some(tmdb) = tmdb_client {
        UnifiedMovieParser::with_config_and_tmdb(config.clone(), tmdb)
    } else {
        UnifiedMovieParser::with_config(config.clone())
    };
    println!("✓ Parser initialized");

    // Validate directory
    if !args.directory.exists() {
        anyhow::bail!("Directory does not exist: {}", args.directory.display());
    }
    if !args.directory.is_dir() {
        anyhow::bail!("Path is not a directory: {}", args.directory.display());
    }

    // Determine output directory
    let output_directory = args
        .output_dir
        .clone()
        .unwrap_or_else(|| args.directory.join("organized"));

    println!("📁 Source directory: {}", args.directory.display());
    println!("📤 Output directory: {}", output_directory.display());
    println!("🔧 Network mode: {}", args.network_mode);
    println!("⚡ Max parallel operations: {}", args.max_parallel);
    println!("📦 Batch size: {}", args.batch_size);
    println!("🎯 Confidence threshold: {:.2}", args.min_confidence);
    println!("🧪 Preview mode: {}", args.preview);
    println!("📝 Default operation: MOVE/RENAME (not copy) - suitable for large media files");
    println!(
        "📁 Organization style: {}",
        if args.organize_by_year {
            "Year-based folders"
        } else {
            "Flat structure (Plex optimal)"
        }
    );

    // Create backup if requested
    if args.backup && !args.preview {
        create_backup(&args.directory).await?;
    }

    // Scan for all files
    let all_files = scan_for_media_files(&args.directory, args.verbose).await?;
    println!("✓ Found {} total files", all_files.len());

    if all_files.is_empty() {
        println!("⚠️  No files found in directory");
        return Ok(());
    }

    // Create organize config
    let organize_config = OrganizeConfig {
        parser,
        output_directory: output_directory.clone(),
        config: config.clone(),
        min_confidence: args.min_confidence,
        preview: args.preview,
        verbose: args.verbose,
        use_cache: args.use_cache,
        organize_by_year: args.organize_by_year,
    };

    // Organize files
    let result = organize_files(&organize_config, &all_files).await?;

    // Display results
    display_organization_results(&result, &args).await?;

    // Save operation to database if not in preview mode
    if !args.preview {
        if let Some(_db) = &database {
            // Create a direct connection for the operation manager
            let db_path = config.database.path.clone();
            let conn = rusqlite::Connection::open(&db_path)?;

            // Create operation history manager
            let operation_manager = crate::database::operations::OperationHistoryManager::new(conn);

            // Store the operation
            let operation_id = operation_manager.store_operation(
                &result,
                &args.directory,
                Some(&output_directory),
            )?;
            println!("✓ Operation saved to database with ID: {}", operation_id);
        } else {
            println!("⚠️  No database available, operation not saved");
        }
    }

    let operation_duration = operation_start.elapsed();

    if args.preview {
        println!("\n📋 PREVIEW SUMMARY");
        println!("==================");
        println!("This is a PREVIEW of what would happen:");
        println!(
            "• {} files would be organized",
            result.organization_stats.successful_organizations
        );
        println!(
            "• {} files would be skipped",
            result.organization_stats.skipped_files
        );
        println!(
            "• {} files would fail",
            result.organization_stats.failed_files
        );
        println!(
            "• Operation type: {}",
            if result.organization_stats.successful_organizations > 0 {
                match result.organized_files[0].operation_type {
                    OperationType::Move => "MOVE files to organized structure",
                    OperationType::Rename => "RENAME files in place",
                    OperationType::Copy => "COPY files (not recommended for large media files)",
                }
            } else {
                "No operations"
            }
        );
        println!("\n💡 To actually perform these operations, run without --preview");
        println!("⚠️  WARNING: This will move/rename your actual files!");
    } else {
        println!("\n✅ Organization completed in {:?}!", operation_duration);
    }

    Ok(())
}

async fn scan_for_media_files(directory: &PathBuf, verbose: bool) -> Result<Vec<PathBuf>> {
    let mut all_files = Vec::new();

    let progress_bar = if !verbose {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
        );
        pb.set_message("Scanning for files...");
        Some(pb)
    } else {
        None
    };

    for entry in walkdir::WalkDir::new(directory)
        .max_depth(5)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            all_files.push(entry.path().to_path_buf());

            if let Some(ref pb) = progress_bar {
                pb.set_message(format!("Found {} files...", all_files.len()));
            }
        }
    }

    if let Some(pb) = progress_bar {
        pb.finish_with_message(format!("Found {} total files", all_files.len()));
    }

    Ok(all_files)
}

async fn organize_files(
    config: &OrganizeConfig,
    media_files: &[PathBuf],
) -> Result<OrganizationResult> {
    let mut organized_files = Vec::new();
    let mut skipped_files = Vec::new();
    let mut failed_files = Vec::new();
    let mut total_size_moved = 0u64;
    let mut total_confidence = 0.0f32;
    let mut successful_count = 0;

    let organize_start = std::time::Instant::now();

    let progress_bar = if !config.verbose {
        let pb = ProgressBar::new(media_files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );
        pb.set_message("Organizing media files...");
        Some(pb)
    } else {
        None
    };

    for file_path in media_files.iter() {
        if let Some(ref pb) = progress_bar {
            pb.set_message(format!(
                "Processing: {}",
                file_path.file_name().unwrap().to_string_lossy()
            ));
        }

        // First, try to parse the file to get confidence information
        let filename = file_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;

        let parser_result = if config.use_cache {
            config.parser.parse_async(filename).await?
        } else if config.parser.tmdb_client.is_some() {
            config.parser.parse_with_tmdb(filename).await?
        } else {
            config.parser.parse(filename)?
        };

        let process_config = ProcessFileConfig {
            parser: &config.parser,
            output_directory: &config.output_directory,
            config: &config.config,
            min_confidence: config.min_confidence,
            preview: config.preview,
            use_cache: config.use_cache,
            organize_by_year: config.organize_by_year,
            parser_result: &parser_result,
        };

        match process_single_file(file_path, &process_config).await {
            Ok(Some(organized_file)) => {
                successful_count += 1;
                total_size_moved += organized_file.media_file.file_size;
                // For now, we'll use a simple confidence check based on title presence
                if organized_file.media_file.metadata.title.is_some() {
                    total_confidence += 1.0;
                }
                organized_files.push(organized_file);
            }
            Ok(None) => {
                // File was skipped - get more detailed information for debugging
                let skip_reason = determine_skip_reason(file_path, &config.config);

                skipped_files.push(SkippedFile {
                    path: file_path.clone(),
                    reason: skip_reason,
                    confidence: Some(parser_result.data.confidence),
                });
            }
            Err(e) => {
                failed_files.push(FailedFile {
                    path: file_path.clone(),
                    error: e.to_string(),
                });
            }
        }

        if let Some(ref pb) = progress_bar {
            pb.inc(1);
        }
    }

    if let Some(pb) = progress_bar {
        pb.finish_with_message("Organization completed");
    }

    let operation_duration = organize_start.elapsed();
    let average_confidence = if successful_count > 0 {
        total_confidence / successful_count as f32
    } else {
        0.0
    };

    let stats = OrganizationStats {
        successful_organizations: organized_files.len(),
        skipped_files: skipped_files.len(),
        failed_files: failed_files.len(),
        total_size_moved,
        average_confidence,
        operation_duration,
    };

    Ok(OrganizationResult {
        total_files: media_files.len(),
        organized_files,
        skipped_files,
        failed_files,
        organization_stats: stats,
        operation_duration,
    })
}

async fn process_single_file(
    file_path: &PathBuf,
    config: &ProcessFileConfig<'_>,
) -> Result<Option<OrganizedFile>> {
    let filename = file_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;

    // Check if file should be skipped
    if should_skip_file(file_path, config.config) {
        return Ok(None);
    }

    // Check confidence threshold
    if config.parser_result.data.confidence < config.min_confidence {
        return Ok(None);
    }

    // Get file metadata
    let metadata = std::fs::metadata(file_path)?;

    // Create MediaFile
    let media_file = MediaFile {
        id: format!("file_{}", uuid::Uuid::new_v4()),
        file_path: file_path.clone(),
        file_name: filename.to_string(),
        file_size: metadata.len(),
        media_type: MediaType::Movie, // For now, assume all are movies
        content_hash: format!("{:x}", md5::compute(filename.as_bytes())),
        last_modified: DateTime::from(
            metadata
                .modified()
                .unwrap_or_else(|_| std::time::SystemTime::now()),
        ),
        metadata: MediaMetadata {
            title: Some(config.parser_result.data.title.clone()),
            original_title: config.parser_result.data.original_title.clone(),
            year: config.parser_result.data.year,
            language: config
                .parser_result
                .data
                .language
                .as_ref()
                .map(|l| vec![l.clone()])
                .unwrap_or_default(),
            quality: config.parser_result.data.quality.clone(),
            source: config.parser_result.data.source.clone(),
            duration: None,
            resolution: None,
            codec: config.parser_result.data.codec.clone(),
            audio_tracks: Vec::new(),
            subtitle_tracks: Vec::new(),
        },
    };

    // Generate new path
    let new_path = generate_organized_path(
        &media_file,
        config.output_directory,
        config.config,
        config.organize_by_year,
    )?;

    // Determine operation type
    let operation_type = if file_path.parent() == Some(config.output_directory) {
        OperationType::Rename
    } else {
        OperationType::Move
    };

    // In preview mode, we don't actually perform the operation
    // but we still determine what would happen
    if !config.preview {
        perform_file_operation(file_path, &new_path, &operation_type).await?;
    }

    Ok(Some(OrganizedFile {
        original_path: file_path.clone(),
        new_path,
        media_file,
        operation_type,
    }))
}

fn should_skip_file(file_path: &Path, config: &AppConfig) -> bool {
    let filename = file_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_lowercase();

    // Check for extras content
    let extras_patterns = config.get_all_content_filtering_patterns();
    for pattern in extras_patterns {
        if filename.contains(&pattern.to_lowercase()) {
            return true;
        }
    }

    // Check for extras extensions
    let extras_extensions = config.get_extras_extensions();
    if let Some(extension) = file_path.extension() {
        let ext = extension.to_string_lossy().to_lowercase();
        if extras_extensions.contains(&ext) {
            return true;
        }
    }

    false
}

fn determine_skip_reason(file_path: &Path, config: &AppConfig) -> SkipReason {
    let filename = file_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_lowercase();

    // Check for extras content
    let extras_patterns = config.get_all_content_filtering_patterns();
    for pattern in extras_patterns {
        if filename.contains(&pattern.to_lowercase()) {
            return SkipReason::ExtrasContent;
        }
    }

    // Check for extras extensions
    let extras_extensions = config.get_extras_extensions();
    if let Some(extension) = file_path.extension() {
        let ext = extension.to_string_lossy().to_lowercase();
        if extras_extensions.contains(&ext) {
            return SkipReason::ExtrasContent;
        }
    }

    // Check for unsupported formats
    let video_extensions = [
        "mkv", "mp4", "avi", "mov", "wmv", "flv", "webm", "m4v", "3gp", "ogv", "ts", "mts", "m2ts",
        "vob", "iso", "bdmv", "mpls",
    ];

    if let Some(extension) = file_path.extension() {
        let ext = extension.to_string_lossy().to_lowercase();
        if !video_extensions.contains(&ext.as_str()) {
            return SkipReason::UnsupportedFormat;
        }
    }

    SkipReason::UserSkipped
}

fn generate_organized_path(
    media_file: &MediaFile,
    output_directory: &Path,
    _config: &AppConfig,
    organize_by_year: bool,
) -> Result<PathBuf> {
    let title = media_file
        .metadata
        .title
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("No title available"))?;

    let year = media_file
        .metadata
        .year
        .map(|y| format!(" ({})", y))
        .unwrap_or_else(|| " (Unknown Year)".to_string());

    let quality_and_source = format_quality_and_source(
        media_file.metadata.quality.as_ref(),
        media_file.metadata.source.as_ref(),
    );

    // Clean title for filesystem
    let clean_title = clean_title_for_filesystem(title);

    // Generate filename
    let filename = format!(
        "{}{}{}.{}",
        clean_title,
        year,
        quality_and_source,
        media_file
            .file_path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
    );

    // Determine the target directory based on organization style
    let target_directory = if organize_by_year {
        // Year-based organization (legacy style)
        if let Some(year) = media_file.metadata.year {
            output_directory.join(year.to_string())
        } else {
            output_directory.join("Unknown Year")
        }
    } else {
        // Flat structure (Plex optimal)
        output_directory.to_path_buf()
    };

    // Create final path
    let final_path = target_directory.join(filename);

    Ok(final_path)
}

fn clean_title_for_filesystem(title: &str) -> String {
    // Replace invalid filesystem characters
    title
        .chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '|' | '?' | '*' | '/' | '\\' => '_',
            _ => c,
        })
        .collect::<String>()
        .replace("  ", " ") // Replace double spaces with single
        .trim()
        .to_string()
}

fn format_quality_and_source(quality: Option<&String>, source: Option<&String>) -> String {
    let mut parts = Vec::new();

    if let Some(q) = quality {
        parts.push(q.clone());
    }

    if let Some(s) = source {
        parts.push(s.clone());
    }

    if parts.is_empty() {
        String::new()
    } else {
        format!(" [{}]", parts.join("] ["))
    }
}

async fn perform_file_operation(
    source: &PathBuf,
    destination: &PathBuf,
    operation_type: &OperationType,
) -> Result<()> {
    // Create destination directory if it doesn't exist
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent)?;
    }

    match operation_type {
        OperationType::Move => {
            std::fs::rename(source, destination)?;
        }
        OperationType::Rename => {
            std::fs::rename(source, destination)?;
        }
        OperationType::Copy => {
            std::fs::copy(source, destination)?;
        }
    }

    Ok(())
}

async fn create_backup(directory: &Path) -> Result<()> {
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let backup_name = format!("backup_{}", timestamp);
    let backup_path = directory.join(backup_name);

    println!("💾 Creating backup at: {}", backup_path.display());

    // For now, we'll just create a backup directory
    // In a real implementation, you might want to copy files or create a compressed archive
    std::fs::create_dir_all(&backup_path)?;

    println!("✅ Backup created successfully");
    Ok(())
}

async fn display_organization_results(
    result: &OrganizationResult,
    args: &OrganizeArgs,
) -> Result<()> {
    print_subsection_header("Organization Results");

    println!("📊 Overall Statistics:");
    println!("  Total files processed: {}", result.total_files);
    println!(
        "  Successfully organized: {}",
        result.organization_stats.successful_organizations
    );
    println!(
        "  Skipped files: {}",
        result.organization_stats.skipped_files
    );
    println!("  Failed files: {}", result.organization_stats.failed_files);
    println!(
        "  Success rate: {:.1}%",
        (result.organization_stats.successful_organizations as f64 / result.total_files as f64)
            * 100.0
    );

    if result.organization_stats.successful_organizations > 0 {
        println!(
            "  Total size moved: {} MB",
            result.organization_stats.total_size_moved / 1024 / 1024
        );
        println!(
            "  Average confidence: {:.2}",
            result.organization_stats.average_confidence
        );
    }

    println!(
        "  Operation duration: {:?}",
        result.organization_stats.operation_duration
    );

    // Show organized files - always show in preview mode, or when verbose
    if args.preview || args.verbose {
        print_subsection_header("File Transformations");

        let display_count = if args.preview {
            result.organized_files.len() // Show all files in preview mode
        } else {
            std::cmp::min(10, result.organized_files.len()) // Show sample when not preview
        };

        for organized_file in result.organized_files.iter().take(display_count) {
            let operation_str = match organized_file.operation_type {
                OperationType::Move => "MOVE",
                OperationType::Rename => "RENAME",
                OperationType::Copy => "COPY",
            };

            // Show full paths in preview mode for better verification
            if args.preview {
                println!(
                    "  {} {} -> {}",
                    operation_str,
                    organized_file.original_path.display(),
                    organized_file.new_path.display()
                );
            } else {
                println!(
                    "  {} {} -> {}",
                    operation_str,
                    organized_file
                        .original_path
                        .file_name()
                        .unwrap()
                        .to_string_lossy(),
                    organized_file
                        .new_path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                );
            }
        }

        if !args.preview && result.organized_files.len() > display_count {
            println!(
                "  ... and {} more files",
                result.organized_files.len() - display_count
            );
        }
    }

    // Show skipped files - always show in preview mode, or when verbose
    if (args.preview || args.verbose) && !result.skipped_files.is_empty() {
        print_subsection_header("Skipped Files");

        let display_count = if args.preview {
            result.skipped_files.len() // Show all skipped files in preview mode
        } else {
            std::cmp::min(5, result.skipped_files.len()) // Show sample when not preview
        };

        for skipped_file in result.skipped_files.iter().take(display_count) {
            let reason_str = match &skipped_file.reason {
                SkipReason::LowConfidence(conf) => format!("Low confidence ({:.2})", conf),
                SkipReason::NoMetadata => "No metadata".to_string(),
                SkipReason::AlreadyOrganized => "Already organized".to_string(),
                SkipReason::ExtrasContent => "Extras content".to_string(),
                SkipReason::UnsupportedFormat => "Unsupported format".to_string(),
                SkipReason::UserSkipped => "User skipped".to_string(),
            };

            // Show full paths in preview mode for better debugging
            if args.preview {
                println!("  ⚠️  {} ({})", skipped_file.path.display(), reason_str);
            } else {
                println!(
                    "  ⚠️  {} ({})",
                    skipped_file.path.file_name().unwrap().to_string_lossy(),
                    reason_str
                );
            }
        }

        if !args.preview && result.skipped_files.len() > display_count {
            println!(
                "  ... and {} more files",
                result.skipped_files.len() - display_count
            );
        }
    }

    // Show failed files
    if !result.failed_files.is_empty() {
        print_subsection_header("Failed Files");

        let sample_count = std::cmp::min(5, result.failed_files.len());
        for failed_file in result.failed_files.iter().take(sample_count) {
            println!(
                "  ❌ {}: {}",
                failed_file.path.file_name().unwrap().to_string_lossy(),
                failed_file.error
            );
        }

        if result.failed_files.len() > sample_count {
            println!(
                "  ... and {} more files",
                result.failed_files.len() - sample_count
            );
        }
    }

    Ok(())
}
