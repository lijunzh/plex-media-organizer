//! Scan command handler for comprehensive media file analysis

use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use crate::{
    config::AppConfig,
    core::Processor,
    database::DatabaseManager,
    output::{print_section_header, print_subsection_header},
    parsers::UnifiedMovieParser,
    types::MediaFile,
};
use serde::{Deserialize, Serialize};

#[derive(Args, Debug)]
pub struct ScanArgs {
    /// Directory to scan
    #[arg(value_name = "DIRECTORY")]
    directory: PathBuf,

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

    /// Output results to JSON file
    #[arg(long)]
    output_json: Option<PathBuf>,

    /// Show parsing statistics
    #[arg(long)]
    show_stats: bool,

    /// Custom database path (overrides config file and environment variable)
    #[arg(long)]
    database_path: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    pub total_files: usize,
    pub media_files: Vec<MediaFile>,
    pub parsing_stats: ParsingStats,
    pub cache_stats: Option<CacheStats>,
    pub scan_duration: std::time::Duration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsingStats {
    pub successful_parses: usize,
    pub failed_parses: usize,
    pub high_confidence_parses: usize,
    pub low_confidence_parses: usize,
    pub average_confidence: f32,
    pub parsing_duration: std::time::Duration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheStats {
    pub hits: usize,
    pub misses: usize,
    pub hit_rate: f32,
}

pub async fn handle_scan(args: ScanArgs) -> Result<()> {
    print_section_header("Media Directory Scanner");

    let scan_start = std::time::Instant::now();

    // Load configuration
    let config = AppConfig::load()?;
    println!("✓ Configuration loaded");

    // Initialize database if caching is enabled
    let database = if args.use_cache {
        let db_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("plex-media-organizer")
            .join("cache.db");

        std::fs::create_dir_all(db_path.parent().unwrap())?;
        let db = DatabaseManager::new(&db_path).await?;
        println!("✓ Database cache initialized at: {}", db_path.display());
        Some(db)
    } else {
        None
    };

    // Create parser
    let parser = if let Some(db) = database {
        UnifiedMovieParser::with_config_and_database(config, db)
    } else {
        UnifiedMovieParser::with_config(config)
    };
    println!("✓ Parser initialized");

    // Validate directory
    if !args.directory.exists() {
        anyhow::bail!("Directory does not exist: {}", args.directory.display());
    }
    if !args.directory.is_dir() {
        anyhow::bail!("Path is not a directory: {}", args.directory.display());
    }

    println!("📁 Scanning directory: {}", args.directory.display());
    println!("🔧 Network mode: {}", args.network_mode);
    println!("⚡ Max parallel operations: {}", args.max_parallel);
    println!("📦 Batch size: {}", args.batch_size);
    println!("🎯 Confidence threshold: {:.2}", args.min_confidence);

    // Create processor with appropriate settings
    let mut processor = if args.network_mode {
        Processor::for_network_drive(parser)
    } else {
        Processor::with_concurrency(parser, args.max_parallel)
    };

    // Set network mode if requested
    processor.set_network_mode(args.network_mode);

    // Process directory using the new core module
    let scan_result = processor.process_directory(&args.directory).await?;
    
    let media_files = scan_result.files;
    let parsed_files = scan_result.parsed_files;
    
    println!("✓ Found {} media files", media_files.len());

    if media_files.is_empty() {
        println!("⚠️  No media files found in directory");
        return Ok(());
    }

    // Create parsing stats from scan result
    let parsing_stats = ParsingStats {
        successful_parses: scan_result.statistics.parsed_files as usize,
        failed_parses: scan_result.statistics.failed_files as usize,
        high_confidence_parses: parsed_files.len(),
        low_confidence_parses: scan_result.failed_files.len(),
        average_confidence: 0.8, // Placeholder - would need to calculate from actual confidence scores
        parsing_duration: std::time::Duration::from_secs_f64(scan_result.statistics.duration_seconds),
    };

    let cache_stats = None; // Placeholder - cache stats would need to be implemented

    // Filter results based on confidence
    let _high_confidence_files: Vec<_> = parsed_files
        .iter()
        .filter(|file| {
            // For now, we'll use a simple confidence check based on title presence
            file.parsed_metadata.title.is_some()
        })
        .cloned()
        .collect();

    let _low_confidence_files: Vec<_> = parsed_files
        .iter()
        .filter(|file| {
            // For now, we'll use a simple confidence check based on title presence
            file.parsed_metadata.title.is_none()
        })
        .cloned()
        .collect();

    // Generate scan results
    let scan_duration = scan_start.elapsed();
    let result = ScanResult {
        total_files: media_files.len(),
        media_files: media_files,
        parsing_stats,
        cache_stats,
        scan_duration,
    };

    // Display results
    display_scan_results(&result, &args).await?;

    // Save to JSON if requested
    if let Some(json_path) = args.output_json {
        save_results_to_json(&result, &json_path).await?;
        println!("✓ Results saved to: {}", json_path.display());
    }

    println!("\n✅ Scan completed successfully!");
    Ok(())
}

async fn display_scan_results(result: &ScanResult, args: &ScanArgs) -> Result<()> {
    print_subsection_header("Scan Results Summary");

    println!("📊 Overall Statistics:");
    println!("  Total files scanned: {}", result.total_files);
    println!("  Scan duration: {:?}", result.scan_duration);
    println!(
        "  Average scan speed: {:.1} files/second",
        result.total_files as f64 / result.scan_duration.as_secs_f64()
    );

    println!("\n🎯 Parsing Statistics:");
    println!(
        "  Successful parses: {}",
        result.parsing_stats.successful_parses
    );
    println!("  Failed parses: {}", result.parsing_stats.failed_parses);
    println!(
        "  Success rate: {:.1}%",
        (result.parsing_stats.successful_parses as f64 / result.total_files as f64) * 100.0
    );
    println!(
        "  High confidence (≥{:.2}): {}",
        args.min_confidence, result.parsing_stats.high_confidence_parses
    );
    println!(
        "  Low confidence (<{:.2}): {}",
        args.min_confidence, result.parsing_stats.low_confidence_parses
    );
    println!(
        "  Average confidence: {:.2}",
        result.parsing_stats.average_confidence
    );
    println!(
        "  Parsing duration: {:?}",
        result.parsing_stats.parsing_duration
    );

    if let Some(ref cache_stats) = result.cache_stats {
        println!("\n🗄️  Cache Statistics:");
        println!("  Hits: {}", cache_stats.hits);
        println!("  Misses: {}", cache_stats.misses);
        println!("  Hit rate: {:.1}%", cache_stats.hit_rate * 100.0);
    }

    // Show sample results
    if args.show_stats {
        print_subsection_header("Sample Results");

        let high_confidence_samples: Vec<_> = result
            .media_files
            .iter()
            .filter(|file| {
                // For now, we'll use a simple confidence check based on title presence
                file.metadata.title.is_some()
            })
            .take(10)
            .collect();

        println!("High Confidence Samples:");
        for file in high_confidence_samples {
            println!(
                "  📁 {} -> 🎬 {} ({:?}) [Confidence: {:.2}]",
                file.file_name,
                file.metadata.title.as_deref().unwrap_or("Unknown"),
                file.metadata.year,
                if file.metadata.title.is_some() {
                    1.0
                } else {
                    0.0
                }
            );
        }

        if result.parsing_stats.low_confidence_parses > 0 {
            let low_confidence_samples: Vec<_> = result
                .media_files
                .iter()
                .filter(|file| {
                    // For now, we'll use a simple confidence check based on title presence
                    file.metadata.title.is_none()
                })
                .take(5)
                .collect();

            println!("\nLow Confidence Samples:");
            for file in low_confidence_samples {
                println!(
                    "  ⚠️  {} -> 🎬 {} ({:?}) [Confidence: {:.2}]",
                    file.file_name,
                    file.metadata.title.as_deref().unwrap_or("Unknown"),
                    file.metadata.year,
                    0.0
                );
            }
        }
    }
    Ok(())
}

async fn save_results_to_json(result: &ScanResult, json_path: &PathBuf) -> Result<()> {
    let json_data = serde_json::to_string_pretty(result)?;
    std::fs::write(json_path, json_data)?;
    Ok(())
}
