//! Test command handler for testing parser functionality

use crate::{
    config::AppConfig,
    database::DatabaseManager,
    output::{print_section_header, print_subsection_header},
    parsers::UnifiedMovieParser,
};
use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct TestArgs {
    /// Directory containing test files
    #[arg(value_name = "DIRECTORY")]
    directory: PathBuf,

    /// Test specific filename patterns
    #[arg(short, long)]
    patterns: Vec<String>,

    /// Show detailed output
    #[arg(short, long)]
    verbose: bool,

    /// Use database caching
    #[arg(long)]
    use_cache: bool,

    /// Show cache statistics
    #[arg(long)]
    cache_stats: bool,
}

pub async fn handle_test(args: TestArgs) -> Result<()> {
    print_section_header("Parser Test Mode");

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

    // Test patterns
    if !args.patterns.is_empty() {
        test_patterns(&parser, &args.patterns, args.verbose, args.use_cache).await?;
    }

    // Test directory
    if args.directory.exists() {
        test_directory(&parser, &args.directory, args.verbose, args.use_cache).await?;
    } else {
        println!("⚠️  Directory does not exist: {}", args.directory.display());
    }

    // Show cache statistics if requested
    if args.cache_stats && args.use_cache {
        show_cache_stats(&parser).await?;
    }

    println!("\n✅ Test completed successfully!");
    Ok(())
}

async fn test_patterns(
    parser: &UnifiedMovieParser,
    patterns: &[String],
    verbose: bool,
    use_cache: bool,
) -> Result<()> {
    print_subsection_header("Testing Filename Patterns");

    let mut results = Vec::new();
    let mut cache_hits = 0;
    let mut cache_misses = 0;

    for (i, pattern) in patterns.iter().enumerate() {
        if verbose {
            println!("\n--- Pattern {}: {} ---", i + 1, pattern);
        }

        let start = std::time::Instant::now();
        let result = if use_cache {
            parser.parse_async(pattern).await
        } else {
            parser.parse(pattern)
        };
        let duration = start.elapsed();

        match result {
            Ok(parser_result) => {
                let cache_status = if use_cache && parser_result.parsing_method == "unified_cached"
                {
                    cache_hits += 1;
                    "CACHE HIT"
                } else if use_cache {
                    cache_misses += 1;
                    "CACHE MISS"
                } else {
                    "N/A"
                };

                results.push((
                    pattern.clone(),
                    parser_result.data.title.clone(),
                    parser_result.data.year,
                    parser_result.data.quality.clone(),
                    parser_result.data.source.clone(),
                    format!("{:.2}", parser_result.data.confidence),
                    format!("{:?}", duration),
                    cache_status.to_string(),
                ));

                if verbose {
                    println!("  Title: {}", parser_result.data.title);
                    println!("  Year: {:?}", parser_result.data.year);
                    println!("  Quality: {:?}", parser_result.data.quality);
                    println!("  Source: {:?}", parser_result.data.source);
                    println!("  Confidence: {:.2}", parser_result.data.confidence);
                    println!("  Duration: {:?}", duration);
                    println!("  Cache: {}", cache_status);
                }
            }
            Err(e) => {
                results.push((
                    pattern.clone(),
                    "ERROR".to_string(),
                    None,
                    None,
                    None,
                    "0.00".to_string(),
                    format!("{:?}", duration),
                    "ERROR".to_string(),
                ));

                if verbose {
                    println!("  Error: {}", e);
                }
            }
        }
    }

    // Print summary table
    println!("\nResults Summary:");
    println!(
        "{:<40} {:<20} {:<6} {:<8} {:<10} {:<10} {:<12} {:<8}",
        "Filename", "Title", "Year", "Quality", "Source", "Confidence", "Duration", "Cache"
    );
    println!("{}", "-".repeat(120));

    for (filename, title, year, quality, source, confidence, duration, cache) in &results {
        let year_str = year
            .map(|y| y.to_string())
            .unwrap_or_else(|| "N/A".to_string());
        let quality_str = quality.as_deref().unwrap_or("N/A");
        let source_str = source.as_deref().unwrap_or("N/A");

        println!(
            "{:<40} {:<20} {:<6} {:<8} {:<10} {:<10} {:<12} {:<8}",
            truncate_string(filename, 38),
            truncate_string(title, 18),
            year_str,
            quality_str,
            source_str,
            confidence,
            duration,
            cache
        );
    }

    if use_cache {
        println!("\nCache Statistics:");
        println!("  Hits: {}", cache_hits);
        println!("  Misses: {}", cache_misses);
        if cache_hits + cache_misses > 0 {
            let hit_rate = cache_hits as f64 / (cache_hits + cache_misses) as f64;
            println!("  Hit Rate: {:.1}%", hit_rate * 100.0);
        }
    }

    Ok(())
}

/// Truncate string to specified length with ellipsis
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}..", &s[..max_len - 2])
    }
}

async fn test_directory(
    parser: &UnifiedMovieParser,
    directory: &PathBuf,
    verbose: bool,
    use_cache: bool,
) -> Result<()> {
    print_subsection_header("Testing Directory Files");

    let mut results = Vec::new();
    let mut total_files = 0;
    let mut successful_parses = 0;
    let mut cache_hits = 0;
    let mut cache_misses = 0;

    // Common video file extensions
    let video_extensions = [
        "mkv", "mp4", "avi", "mov", "wmv", "flv", "webm", "m4v", "3gp", "ogv",
    ];

    for entry in walkdir::WalkDir::new(directory)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            if let Some(extension) = entry.path().extension() {
                if video_extensions.contains(&extension.to_string_lossy().to_lowercase().as_str()) {
            total_files += 1;
            let filename = entry.path().file_name().unwrap().to_string_lossy();

            if verbose {
                println!("\n--- File {}: {} ---", total_files, filename);
            }

            let start = std::time::Instant::now();
            let result = if use_cache {
                parser.parse_async(&filename).await
            } else {
                parser.parse(&filename)
            };
            let duration = start.elapsed();

            match result {
                Ok(parser_result) => {
                    successful_parses += 1;

                    let cache_status =
                        if use_cache && parser_result.parsing_method == "unified_cached" {
                            cache_hits += 1;
                            "CACHE HIT"
                        } else if use_cache {
                            cache_misses += 1;
                            "CACHE MISS"
                        } else {
                            "N/A"
                        };

                    results.push((
                        filename.to_string(),
                        parser_result.data.title.clone(),
                        parser_result.data.year,
                        parser_result.data.quality.clone(),
                        parser_result.data.source.clone(),
                        format!("{:.2}", parser_result.data.confidence),
                        format!("{:?}", duration),
                        cache_status.to_string(),
                    ));

                    if verbose {
                        println!("  Title: {}", parser_result.data.title);
                        println!("  Year: {:?}", parser_result.data.year);
                        println!("  Quality: {:?}", parser_result.data.quality);
                        println!("  Source: {:?}", parser_result.data.source);
                        println!("  Confidence: {:.2}", parser_result.data.confidence);
                        println!("  Duration: {:?}", duration);
                        println!("  Cache: {}", cache_status);
                    }
                }
                Err(e) => {
                    results.push((
                        filename.to_string(),
                        "ERROR".to_string(),
                        None,
                        None,
                        None,
                        "0.00".to_string(),
                        format!("{:?}", duration),
                        "ERROR".to_string(),
                    ));

                    if verbose {
                        println!("  Error: {}", e);
                    }
                }
            }
        }
    }

    // Print summary
    println!("\nDirectory Scan Summary:");
    println!("  Total video files found: {}", total_files);
    println!("  Successful parses: {}", successful_parses);
    println!(
        "  Success rate: {:.1}%",
        if total_files > 0 {
            (successful_parses as f64 / total_files as f64) * 100.0
        } else {
            0.0
        }
    );

    if use_cache {
        println!("  Cache hits: {}", cache_hits);
        println!("  Cache misses: {}", cache_misses);
        if cache_hits + cache_misses > 0 {
            let hit_rate = cache_hits as f64 / (cache_hits + cache_misses) as f64;
            println!("  Cache hit rate: {:.1}%", hit_rate * 100.0);
        }
    }

    // Show top results (limit to 20 for readability)
    if results.len() > 20 {
        println!("\nShowing top 20 results:");
        results.truncate(20);
    }

    println!("\nResults Summary:");
    println!(
        "{:<40} {:<20} {:<6} {:<8} {:<10} {:<10} {:<12} {:<8}",
        "Filename", "Title", "Year", "Quality", "Source", "Confidence", "Duration", "Cache"
    );
    println!("{}", "-".repeat(120));

    for (filename, title, year, quality, source, confidence, duration, cache) in &results {
        let year_str = year
            .map(|y| y.to_string())
            .unwrap_or_else(|| "N/A".to_string());
        let quality_str = quality.as_deref().unwrap_or("N/A");
        let source_str = source.as_deref().unwrap_or("N/A");

        println!(
            "{:<40} {:<20} {:<6} {:<8} {:<10} {:<10} {:<12} {:<8}",
            truncate_string(filename, 38),
            truncate_string(title, 18),
            year_str,
            quality_str,
            source_str,
            confidence,
            duration,
            cache
        );
    }

    Ok(())
}

async fn show_cache_stats(parser: &UnifiedMovieParser) -> Result<()> {
    print_subsection_header("Cache Statistics");

    if let Some(stats) = parser.get_cache_stats().await? {
        println!("Parsing Cache:");
        println!("  Entries: {}", stats.0);
        println!("  TTL: {:?}", stats.1);

        println!("\nTMDB Cache:");
        println!("  Entries: {}", stats.0);
        println!("  TTL: {:?}", stats.1);
    } else {
        println!("No database cache available");
    }

    Ok(())
}
