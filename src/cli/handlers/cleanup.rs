use anyhow::Result;
use clap::Args;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;

use crate::{
    config::AppConfig,
    database::operations::OperationHistoryManager,
    output::{print_section_header, print_subsection_header},
};

/// Arguments for the cleanup command
#[derive(Debug, Args)]
pub struct CleanupArgs {
    /// Keep operation files for this many days
    #[arg(long, default_value = "30")]
    pub keep_days: u32,

    /// Keep this many most recent operation files
    #[arg(long, default_value = "10")]
    pub keep_count: usize,

    /// Preview cleanup changes (dry-run)
    #[arg(short, long)]
    pub preview: bool,

    /// Show detailed output
    #[arg(short, long)]
    pub verbose: bool,
}

/// Result of cleanup operation
#[derive(Debug)]
pub struct CleanupResult {
    pub operations_deleted: usize,
    pub operation_files_deleted: usize,
    pub cache_entries_cleared: usize,
    pub database_size_before: u64,
    pub database_size_after: u64,
    pub cleanup_duration: std::time::Duration,
}

/// Handle the cleanup command
pub async fn handle_cleanup(args: CleanupArgs) -> Result<()> {
    print_section_header("Database Cleanup and Maintenance");

    let operation_start = std::time::Instant::now();

    // Load configuration
    let config = AppConfig::load()?;
    println!("✓ Configuration loaded");

    // Create database connection
    let db_path = PathBuf::from(&config.database.path);
    let conn = rusqlite::Connection::open(&db_path)?;
    let operation_manager = OperationHistoryManager::new(conn);

    // Get database size before cleanup
    let database_size_before = std::fs::metadata(&db_path)?.len();

    if args.preview {
        println!("🔍 Preview mode - no changes will be made");
    }

    // Get operation statistics before cleanup
    let stats_before = operation_manager.get_operation_stats()?;
    println!("📊 Current database statistics:");
    println!("  • Total operations: {}", stats_before.total_operations);
    println!(
        "  • Total files processed: {}",
        stats_before.total_files_processed
    );
    println!(
        "  • Database size: {:.2} MB",
        database_size_before as f64 / 1024.0 / 1024.0
    );

    // Clean up old operations
    let cleanup_result = cleanup_old_operations(&operation_manager, &args).await?;

    // Optimize database
    if !args.preview {
        optimize_database(&db_path)?;
    }

    // Get database size after cleanup
    let database_size_after = std::fs::metadata(&db_path)?.len();

    // Get operation statistics after cleanup
    let stats_after = operation_manager.get_operation_stats()?;

    // Display results
    display_cleanup_results(
        &cleanup_result,
        &stats_before,
        &stats_after,
        database_size_before,
        database_size_after,
        operation_start.elapsed(),
        &args,
    )?;

    Ok(())
}

/// Clean up old operations from the database
async fn cleanup_old_operations(
    operation_manager: &OperationHistoryManager,
    args: &CleanupArgs,
) -> Result<CleanupResult> {
    print_subsection_header("Cleaning Up Old Operations");

    let mut operations_deleted = 0;
    let operation_files_deleted = 0;

    if args.preview {
        // In preview mode, just show what would be deleted
        let recent_operations = operation_manager.get_recent_operations(args.keep_count)?;
        let total_operations = operation_manager.get_operation_stats()?.total_operations;

        if total_operations > args.keep_count {
            let would_delete = total_operations - args.keep_count;
            println!(
                "📋 Would delete {} old operations (keeping {} most recent)",
                would_delete, args.keep_count
            );

            if args.verbose {
                println!("📋 Would keep these recent operations:");
                for op in recent_operations.iter().take(5) {
                    println!(
                        "  • {} ({}) - {} files",
                        op.operation_id,
                        op.created_at.format("%Y-%m-%d %H:%M"),
                        op.total_files
                    );
                }
                if recent_operations.len() > 5 {
                    println!("  • ... and {} more", recent_operations.len() - 5);
                }
            }
        } else {
            println!(
                "📋 No operations to delete (only {} operations exist)",
                total_operations
            );
        }
    } else {
        // Actually perform the cleanup
        let progress_bar = if !args.verbose {
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Cleaning up old operations...");
            Some(pb)
        } else {
            None
        };

        // Delete operations keeping only the most recent N operations
        operations_deleted = operation_manager.cleanup_old_operations_by_count(args.keep_count)?;

        if let Some(pb) = progress_bar {
            pb.finish_with_message("✓ Old operations cleaned up");
        }

        if args.verbose {
            println!("🗑️  Deleted {} old operations", operations_deleted);
        }
    }

    Ok(CleanupResult {
        operations_deleted,
        operation_files_deleted,
        cache_entries_cleared: 0, // TODO: Implement cache cleanup
        database_size_before: 0,
        database_size_after: 0,
        cleanup_duration: std::time::Duration::from_secs(0),
    })
}

/// Optimize the database for better performance
fn optimize_database(db_path: &PathBuf) -> Result<()> {
    print_subsection_header("Database Optimization");

    let conn = rusqlite::Connection::open(db_path)?;

    // Run VACUUM to reclaim space and optimize the database
    conn.execute("VACUUM", [])?;
    println!("✓ Database vacuumed");

    // Update statistics for better query planning
    conn.execute("ANALYZE", [])?;
    println!("✓ Database statistics updated");

    // Rebuild indexes for better performance
    conn.execute("REINDEX", [])?;
    println!("✓ Database indexes rebuilt");

    Ok(())
}

/// Display cleanup results
fn display_cleanup_results(
    cleanup_result: &CleanupResult,
    stats_before: &crate::database::operations::OperationStats,
    stats_after: &crate::database::operations::OperationStats,
    database_size_before: u64,
    database_size_after: u64,
    duration: std::time::Duration,
    args: &CleanupArgs,
) -> Result<()> {
    print_subsection_header("Cleanup Results");

    if args.preview {
        println!("🔍 Preview completed in {:.2?}", duration);
        return Ok(());
    }

    let size_saved = database_size_before.saturating_sub(database_size_after);
    let size_saved_mb = size_saved as f64 / 1024.0 / 1024.0;

    println!("✅ Cleanup completed in {:.2?}", duration);
    println!();
    println!("📊 Cleanup Summary:");
    println!(
        "  • Operations deleted: {}",
        cleanup_result.operations_deleted
    );
    println!(
        "  • Operation files deleted: {}",
        cleanup_result.operation_files_deleted
    );
    println!(
        "  • Cache entries cleared: {}",
        cleanup_result.cache_entries_cleared
    );
    println!("  • Database size reduction: {:.2} MB", size_saved_mb);
    println!();
    println!("📈 Database Statistics:");
    println!(
        "  • Operations before: {} → after: {}",
        stats_before.total_operations, stats_after.total_operations
    );
    println!(
        "  • Files processed before: {} → after: {}",
        stats_before.total_files_processed, stats_after.total_files_processed
    );
    println!(
        "  • Database size: {:.2} MB → {:.2} MB",
        database_size_before as f64 / 1024.0 / 1024.0,
        database_size_after as f64 / 1024.0 / 1024.0
    );

    if size_saved > 0 {
        println!();
        println!(
            "🎉 Successfully freed {:.2} MB of disk space!",
            size_saved_mb
        );
    }

    Ok(())
}
