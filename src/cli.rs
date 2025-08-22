//! Command-line interface for the Plex Media Organizer

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::config::AppConfig;
use crate::movie_parser::MovieParser;
use crate::scanner::Scanner;
use crate::tmdb_client::TmdbClient;

/// Confidence filtering settings
#[derive(Debug, Clone)]
struct ConfidenceSettings {
    min_confidence: f32,
    skip_unmatched: bool,
    no_warnings: bool,
}

/// Plex Media Organizer - Intelligent media file organization
#[derive(Parser)]
#[command(name = "plex-media-organizer")]
#[command(about = "Intelligent media file organizer following Plex naming conventions")]
#[command(version = "0.1.0")]
#[derive(Debug)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Scan a directory for media files
    Scan {
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
    },

    /// Set up configuration interactively
    Setup {
        /// Force reconfiguration even if config exists
        #[arg(short, long)]
        force: bool,
    },

    /// Show current configuration
    Config {
        /// Show configuration file path
        #[arg(short, long)]
        path: bool,
    },

    /// Test parsing and organization
    Test {
        /// File or directory to test
        #[arg(value_name = "PATH")]
        path: PathBuf,

        /// Test organization (scan + parse + organize)
        #[arg(short, long)]
        organize: bool,

        /// Preview organization changes (dry-run)
        #[arg(short, long)]
        preview: bool,

        /// Show detailed output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Organize files to Plex naming conventions
    Organize {
        /// Directory to organize
        #[arg(value_name = "DIRECTORY")]
        directory: PathBuf,

        /// Preview changes without making them (dry-run)
        #[arg(short, long)]
        preview: bool,

        /// Backup directory for rollback files
        #[arg(short, long, value_name = "BACKUP_DIR")]
        backup: Option<PathBuf>,

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
    },

    /// Rollback a previous organization operation
    Rollback {
        /// Path to the organization result JSON file
        #[arg(value_name = "OPERATION_FILE")]
        operation_file: PathBuf,

        /// Preview rollback changes without making them (dry-run)
        #[arg(short, long)]
        preview: bool,

        /// Show detailed output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Clean up old organization result files
    Cleanup {
        /// Keep files newer than this many days (default: 30)
        #[arg(long, default_value = "30")]
        keep_days: u32,

        /// Keep at most this many recent files (default: 100)
        #[arg(long, default_value = "100")]
        keep_count: usize,

        /// Preview cleanup without making changes (dry-run)
        #[arg(short, long)]
        preview: bool,

        /// Show detailed output
        #[arg(short, long)]
        verbose: bool,
    },
}

impl Cli {
    /// Run the CLI application
    pub async fn run() -> Result<()> {
        let cli = Cli::parse();

        match cli.command {
            Commands::Scan {
                directory,
                verbose,
                network_mode,
                max_parallel,
                batch_size,
                min_confidence,
                skip_unmatched,
                no_warnings,
            } => {
                let confidence_settings = ConfidenceSettings {
                    min_confidence,
                    skip_unmatched,
                    no_warnings,
                };
                Self::handle_scan(
                    directory,
                    network_mode,
                    max_parallel,
                    batch_size,
                    confidence_settings,
                    verbose,
                )
                .await
            }
            Commands::Setup { force } => Self::handle_setup(force).await,
            Commands::Config { path } => Self::handle_config(path).await,
            Commands::Test {
                path,
                organize,
                preview,
                verbose,
            } => Self::handle_test(path, organize, preview, verbose).await,
            Commands::Organize {
                directory,
                preview,
                backup,
                verbose,
                network_mode,
                max_parallel,
                batch_size,
                min_confidence,
                skip_unmatched,
                no_warnings,
            } => {
                let confidence_settings = ConfidenceSettings {
                    min_confidence,
                    skip_unmatched,
                    no_warnings,
                };
                Self::handle_organize(
                    directory,
                    preview,
                    backup,
                    verbose,
                    network_mode,
                    max_parallel,
                    batch_size,
                    confidence_settings,
                )
                .await
            }
            Commands::Rollback {
                operation_file,
                preview,
                verbose,
            } => Self::handle_rollback(operation_file, preview, verbose).await,
            Commands::Cleanup {
                keep_days,
                keep_count,
                preview,
                verbose,
            } => Self::handle_cleanup(keep_days, keep_count, preview, verbose).await,
        }
    }

    /// Handle the scan command
    async fn handle_scan(
        directory: PathBuf,
        network_mode: bool,
        max_parallel: usize,
        batch_size: usize,
        confidence_settings: ConfidenceSettings,
        verbose: bool,
    ) -> Result<()> {
        println!("🎬 Plex Media Organizer - Scanning Directory");
        println!("Directory: {}", directory.display());
        println!();

        // Load configuration ONCE at the entry point
        let config = match AppConfig::load() {
            Ok(config) => config,
            Err(_) => {
                println!("⚠️  No configuration found. Run 'setup' first.");
                return Ok(());
            }
        };

        // Validate API keys
        if let Err(e) = config.validate_api_keys() {
            println!("⚠️  Configuration issue: {}", e);
            println!("Run 'setup' to configure API keys.");
            return Ok(());
        }

        // Create TMDB client
        let tmdb_client = config.apis.tmdb_api_key.clone().map(TmdbClient::new);

        // Create movie parser with the loaded config (single load)
        let movie_parser = MovieParser::with_config(tmdb_client, config.clone());

        // Create scanner with network optimizations if requested
        let mut scanner = if network_mode {
            Scanner::for_network_drive_with_config(movie_parser, &config)
        } else {
            Scanner::with_concurrency_and_config(movie_parser, max_parallel, &config)
        };

        // Apply custom settings
        scanner.set_batch_size(batch_size);

        // Override confidence settings from CLI arguments
        scanner
            .config
            .organization
            .matching
            .min_confidence_threshold = confidence_settings.min_confidence;
        scanner.config.organization.matching.skip_unmatched_movies =
            confidence_settings.skip_unmatched;
        scanner.config.organization.matching.warn_on_low_confidence =
            !confidence_settings.no_warnings;

        // Auto-detect network drives if not explicitly set
        if !network_mode && Scanner::detect_network_drive(&directory) {
            println!("🌐 Auto-detected network drive - enabling optimizations");
            scanner.set_network_mode(true);
        }

        // Scan directory
        let scan_result = scanner
            .scan_directory(&directory)
            .await
            .context("Failed to scan directory")?;

        // Display results
        Self::display_scan_results(&scan_result, verbose);

        Ok(())
    }

    /// Handle the setup command
    async fn handle_setup(force: bool) -> Result<()> {
        println!("🔧 Plex Media Organizer - Setup");

        // Check if config already exists
        if !force
            && let Ok(config) = AppConfig::load()
            && config.apis.tmdb_api_key.is_some()
        {
            println!("Configuration already exists. Use --force to reconfigure.");
            return Ok(());
        }

        // Run interactive setup
        let _config = AppConfig::interactive_setup().context("Setup failed")?;

        println!("✅ Setup completed successfully!");
        println!(
            "Configuration saved to: {}",
            AppConfig::get_config_dir()?.join("config.toml").display()
        );

        Ok(())
    }

    /// Handle the config command
    async fn handle_config(path: bool) -> Result<()> {
        if path {
            let config_dir = AppConfig::get_config_dir()?;
            let config_file = config_dir.join("config.toml");
            println!("Configuration file: {}", config_file.display());
            return Ok(());
        }

        match AppConfig::load() {
            Ok(config) => {
                println!("📋 Current Configuration:");
                println!(
                    "TMDB API Key: {}",
                    if config.apis.tmdb_api_key.is_some() {
                        "✅ Set"
                    } else {
                        "❌ Not set"
                    }
                );
            }
            Err(e) => {
                println!("❌ Failed to load configuration: {}", e);
                println!("Run 'setup' to create configuration.");
            }
        }

        Ok(())
    }

    /// Handle the test command
    async fn handle_test(
        path: PathBuf,
        organize: bool,
        preview: bool,
        verbose: bool,
    ) -> Result<()> {
        if organize {
            // Test full workflow: scan + parse + organize
            println!("🧪 Plex Media Organizer - Testing Full Workflow");
            println!("Path: {}", path.display());
            println!(
                "Mode: {}",
                if preview {
                    "Preview (dry-run)"
                } else {
                    "Live test"
                }
            );
            println!();

            if !path.exists() {
                anyhow::bail!("Path does not exist: {}", path.display());
            }

            // Load configuration
            let config = match AppConfig::load() {
                Ok(config) => config,
                Err(_) => {
                    println!("⚠️  No configuration found. Running without TMDB integration.");
                    // Continue without config
                    AppConfig::default()
                }
            };

            // Create TMDB client if available
            let tmdb_client = config.apis.tmdb_api_key.map(TmdbClient::new);

            // Create movie parser and scanner
            let movie_parser = MovieParser::new(tmdb_client);
            let scanner = Scanner::new(movie_parser);

            // Scan directory
            println!("📋 Step 1: Scanning directory...");
            let scan_result = scanner
                .scan_directory(&path)
                .await
                .context("Failed to scan directory")?;

            if scan_result.parsed_files.is_empty() {
                println!("❌ No media files found to test.");
                return Ok(());
            }

            println!("Found {} files to test", scan_result.parsed_files.len());

            // Display scan results
            if verbose {
                Self::display_scan_results(&scan_result, verbose);
            }

            // Test organization
            println!("\n📋 Step 2: Testing organization...");
            let organizer = crate::organizer::Organizer::new(preview, None);

            let organization_result = organizer
                .organize_scan_result(&scan_result)
                .await
                .context("Failed to organize files")?;

            println!("\n✅ Full workflow test completed!");
            println!(
                "Scan: {} files found and parsed",
                scan_result.parsed_files.len()
            );
            println!(
                "Organize: {} files processed",
                organization_result.statistics.total_files
            );
            println!(
                "Success rate: {:.1}%",
                organization_result.statistics.success_rate * 100.0
            );
        } else {
            // Test single file parsing
            println!("🧪 Plex Media Organizer - Testing File Parsing");
            println!("File: {}", path.display());
            println!();

            if !path.exists() {
                anyhow::bail!("File does not exist: {}", path.display());
            }

            // Load configuration
            let config = match AppConfig::load() {
                Ok(config) => config,
                Err(_) => {
                    println!("⚠️  No configuration found. Running without TMDB integration.");
                    // Continue without config
                    AppConfig::default()
                }
            };

            // Create TMDB client if available
            let tmdb_client = config.apis.tmdb_api_key.map(TmdbClient::new);

            // Create movie parser and test parsing
            let movie_parser = MovieParser::new(tmdb_client);

            match movie_parser.parse_movie(&path).await {
                Ok(result) => {
                    println!("✅ Parsing successful!");
                    println!(
                        "Title: {}",
                        result.parsed_metadata.title.as_deref().unwrap_or("Unknown")
                    );
                    if let Some(original_title) = &result.parsed_metadata.original_title {
                        println!("Original Title: {}", original_title);
                    }
                    if let Some(year) = result.parsed_metadata.year {
                        println!("Year: {}", year);
                    }
                    if let Some(quality) = &result.parsed_metadata.quality {
                        println!("Quality: {}", quality);
                    }
                    if let Some(source) = &result.parsed_metadata.source {
                        println!("Source: {}", source);
                    }
                    println!("Confidence: {:.1}%", result.confidence_score * 100.0);
                    println!("Strategy: {:?}", result.parsing_strategy);

                    if !result.external_sources.is_empty() {
                        println!("External Sources:");
                        for source in &result.external_sources {
                            println!("  - {}: {}", source.name, source.external_id);
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Parsing failed: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Display scan results
    fn display_scan_results(scan_result: &crate::types::ScanResult, verbose: bool) {
        println!("📊 Scan Results:");
        println!("Directory: {}", scan_result.directory.display());
        println!("Total files: {}", scan_result.statistics.total_files);
        println!("Media files: {}", scan_result.files.len());
        println!(
            "Successfully parsed: {}",
            scan_result.statistics.parsed_files
        );
        println!("Failed to parse: {}", scan_result.statistics.failed_files);
        println!(
            "Success rate: {:.1}%",
            scan_result.statistics.success_rate * 100.0
        );
        println!(
            "Average confidence: {:.1}%",
            scan_result.statistics.average_confidence * 100.0
        );
        println!(
            "Scan duration: {:.2}s",
            scan_result.statistics.duration_seconds
        );
        println!();

        if verbose && !scan_result.parsed_files.is_empty() {
            println!("📋 Parsed Files:");
            for (i, result) in scan_result.parsed_files.iter().enumerate() {
                println!("{}. {}", i + 1, result.media_file.file_name);
                if let Some(title) = &result.parsed_metadata.title {
                    println!("   Title: {}", title);
                }
                if let Some(year) = result.parsed_metadata.year {
                    println!("   Year: {}", year);
                }
                if let Some(quality) = &result.parsed_metadata.quality {
                    println!("   Quality: {}", quality);
                }
                println!("   Confidence: {:.1}%", result.confidence_score * 100.0);
                println!();
            }
        }

        if !scan_result.failed_files.is_empty() {
            println!("❌ Failed Files:");
            for failed in &scan_result.failed_files {
                println!("- {}: {}", failed.media_file.file_name, failed.error);
            }
            println!();
        }

        println!("✅ Scan completed!");
    }

    /// Handle the organize command
    #[allow(clippy::too_many_arguments)]
    async fn handle_organize(
        directory: PathBuf,
        preview: bool,
        backup: Option<PathBuf>,
        _verbose: bool,
        network_mode: bool,
        max_parallel: usize,
        batch_size: usize,
        confidence_settings: ConfidenceSettings,
    ) -> Result<()> {
        println!("🎬 Plex Media Organizer - File Organization");
        println!("Directory: {}", directory.display());
        if preview {
            println!("Mode: Preview (dry-run)");
        } else {
            println!("Mode: Live organization");
        }
        if let Some(backup_dir) = &backup {
            println!("Backup directory: {}", backup_dir.display());
        }
        println!();

        // Safety check: require preview mode for lower confidence thresholds
        if confidence_settings.min_confidence < 0.7 && !preview {
            println!("❌ SAFETY CHECK FAILED");
            println!(
                "   • Lower confidence threshold ({:.1}) requires preview mode",
                confidence_settings.min_confidence
            );
            println!("   • Use --preview to review results before applying changes");
            println!("   • This prevents accidental incorrect organization");
            println!();
            anyhow::bail!("Lower confidence threshold requires --preview mode for safety");
        }

        // Display important warnings about the conservative approach
        if confidence_settings.min_confidence >= 0.7 {
            println!("⚠️  CONSERVATIVE MODE ENABLED");
            println!(
                "   • High confidence threshold ({:.1}) ensures accuracy over completeness",
                confidence_settings.min_confidence
            );
            println!("   • Some movies may be skipped to avoid incorrect organization");
            println!(
                "   • Use --min-confidence 0.5 for more permissive matching (review carefully)"
            );
            println!();
        } else if confidence_settings.min_confidence < 0.6 {
            println!("⚠️  PERMISSIVE MODE ENABLED");
            println!(
                "   • Lower confidence threshold ({:.1}) may include incorrect matches",
                confidence_settings.min_confidence
            );
            println!("   • Please review results carefully before applying changes");
            println!("   • Preview mode is required for safety");
            println!();
        } else {
            println!("⚠️  MODERATE MODE ENABLED");
            println!(
                "   • Moderate confidence threshold ({:.1}) - review results carefully",
                confidence_settings.min_confidence
            );
            println!("   • Preview mode is required for safety");
            println!();
        }

        // Load configuration
        let config = match AppConfig::load() {
            Ok(config) => config,
            Err(_) => {
                println!("⚠️  No configuration found. Run 'setup' first.");
                return Ok(());
            }
        };

        // Validate API keys
        if let Err(e) = config.validate_api_keys() {
            println!("⚠️  Configuration issue: {}", e);
            println!("Run 'setup' to configure API keys.");
            return Ok(());
        }

        // Create TMDB client
        let tmdb_client = config.apis.tmdb_api_key.map(TmdbClient::new);

        // Create movie parser and scanner
        let movie_parser = MovieParser::new(tmdb_client);

        // Create scanner with network optimizations if requested
        let mut scanner = if network_mode {
            Scanner::for_network_drive(movie_parser)
        } else {
            Scanner::with_concurrency(movie_parser, max_parallel)
        };

        // Apply custom settings
        scanner.set_batch_size(batch_size);

        // Override confidence settings from CLI arguments
        scanner
            .config
            .organization
            .matching
            .min_confidence_threshold = confidence_settings.min_confidence;
        scanner.config.organization.matching.skip_unmatched_movies =
            confidence_settings.skip_unmatched;
        scanner.config.organization.matching.warn_on_low_confidence =
            !confidence_settings.no_warnings;

        // Auto-detect network drives if not explicitly set
        if !network_mode && Scanner::detect_network_drive(&directory) {
            println!("🌐 Auto-detected network drive - enabling optimizations");
            scanner.set_network_mode(true);
        }

        // Scan directory first
        println!("📋 Scanning directory for media files...");
        let scan_result = scanner
            .scan_directory(&directory)
            .await
            .context("Failed to scan directory")?;

        if scan_result.parsed_files.is_empty() {
            println!("❌ No media files found to organize.");
            return Ok(());
        }

        println!("Found {} files to organize", scan_result.parsed_files.len());

        // Create organizer
        let organizer = crate::organizer::Organizer::new(preview, backup);

        // Organize files
        let organization_result = organizer
            .organize_scan_result(&scan_result)
            .await
            .context("Failed to organize files")?;

        // Save organization result for potential rollback
        if !preview {
            let result_file = format!(
                "organization_result_{}.json",
                organization_result.operation_id
            );
            if let Ok(json) = serde_json::to_string_pretty(&organization_result) {
                if let Err(e) = std::fs::write(&result_file, json) {
                    println!("⚠️  Warning: Could not save organization result: {}", e);
                } else {
                    println!("📄 Organization result saved to: {}", result_file);
                }
            }
        }

        Ok(())
    }

    /// Handle the rollback command
    async fn handle_rollback(operation_file: PathBuf, preview: bool, verbose: bool) -> Result<()> {
        println!("🔄 Plex Media Organizer - Operation Rollback");
        println!("Operation file: {}", operation_file.display());
        if preview {
            println!("Mode: Preview (dry-run)");
        } else {
            println!("Mode: Live rollback");
        }
        println!();

        // Check if operation file exists
        if !operation_file.exists() {
            anyhow::bail!(
                "Operation file does not exist: {}",
                operation_file.display()
            );
        }

        // Load the organization result
        let operation_json = std::fs::read_to_string(&operation_file).with_context(|| {
            format!(
                "Failed to read operation file: {}",
                operation_file.display()
            )
        })?;

        let organization_result: crate::organizer::OrganizationResult =
            serde_json::from_str(&operation_json)
                .with_context(|| "Failed to parse operation file - invalid JSON format")?;

        println!("📋 Operation Details:");
        println!("Operation ID: {}", organization_result.operation_id);
        println!("Timestamp: {}", organization_result.timestamp);
        println!(
            "Files organized: {}",
            organization_result.statistics.organized_files
        );
        println!();

        if organization_result.organized_files.is_empty() {
            println!("ℹ️  No files to rollback - operation had no successful organizations.");
            return Ok(());
        }

        // Check if files can be rolled back
        let mut rollback_plan = Vec::new();
        let mut cannot_rollback = Vec::new();

        for organized_file in &organization_result.organized_files {
            if organized_file.dry_run {
                cannot_rollback.push((organized_file, "Original operation was dry-run"));
                continue;
            }

            if !organized_file.new_path.exists() {
                cannot_rollback.push((organized_file, "Organized file no longer exists"));
                continue;
            }

            if organized_file.original_path.exists() {
                cannot_rollback.push((organized_file, "Original path already exists"));
                continue;
            }

            rollback_plan.push(organized_file);
        }

        // Display rollback plan
        if !rollback_plan.is_empty() {
            println!("📋 Rollback Plan ({} files):", rollback_plan.len());
            for (i, file) in rollback_plan.iter().enumerate() {
                println!(
                    "{}. {} ← {}",
                    i + 1,
                    file.original_path.display(),
                    file.new_path.display()
                );
                if verbose {
                    println!(
                        "   Title: {}",
                        file.parsed_metadata.title.as_deref().unwrap_or("Unknown")
                    );
                    if let Some(year) = file.parsed_metadata.year {
                        println!("   Year: {}", year);
                    }
                }
            }
            println!();
        }

        if !cannot_rollback.is_empty() {
            println!("⚠️  Cannot Rollback ({} files):", cannot_rollback.len());
            for (file, reason) in &cannot_rollback {
                println!("- {}: {}", file.media_file.file_name, reason);
            }
            println!();
        }

        if rollback_plan.is_empty() {
            println!("❌ No files can be rolled back.");
            return Ok(());
        }

        // Perform rollback
        if preview {
            println!("🔍 DRY-RUN COMPLETE: No actual changes were made");
            println!("Run without --preview to perform the actual rollback.");
        } else {
            println!("🔄 Performing rollback...");
            let mut success_count = 0;
            let mut failed_rollbacks = Vec::new();

            for organized_file in &rollback_plan {
                match Self::perform_single_rollback(organized_file).await {
                    Ok(()) => {
                        success_count += 1;
                        println!("✅ {}", organized_file.media_file.file_name);
                    }
                    Err(e) => {
                        failed_rollbacks.push((organized_file, e.to_string()));
                        println!("❌ {}: {}", organized_file.media_file.file_name, e);
                    }
                }
            }

            println!();
            println!("📊 Rollback Results:");
            println!(
                "Successfully rolled back: {}/{}",
                success_count,
                rollback_plan.len()
            );

            if !failed_rollbacks.is_empty() {
                println!("Failed rollbacks: {}", failed_rollbacks.len());
                if verbose {
                    println!("\n❌ Failed Rollbacks:");
                    for (file, error) in failed_rollbacks {
                        println!("- {}: {}", file.media_file.file_name, error);
                    }
                }
            }

            if success_count == rollback_plan.len() {
                println!("\n✅ Rollback completed successfully!");

                // Remove empty directories created during organization
                for organized_file in &rollback_plan {
                    if let Some(parent) = organized_file.new_path.parent()
                        && parent.exists()
                        && parent
                            .read_dir()
                            .is_ok_and(|mut entries| entries.next().is_none())
                    {
                        if let Err(e) = std::fs::remove_dir(parent) {
                            if verbose {
                                println!(
                                    "⚠️  Could not remove empty directory {}: {}",
                                    parent.display(),
                                    e
                                );
                            }
                        } else if verbose {
                            println!("🗑️  Removed empty directory: {}", parent.display());
                        }
                    }
                }
            } else {
                println!(
                    "\n⚠️  Rollback completed with some failures. Check the error messages above."
                );
            }
        }

        Ok(())
    }

    /// Perform rollback for a single file
    async fn perform_single_rollback(
        organized_file: &crate::organizer::OrganizedFile,
    ) -> Result<()> {
        // Create parent directory for original path if needed
        if let Some(parent) = organized_file.original_path.parent()
            && !parent.exists()
        {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        // Move file back to original location
        std::fs::rename(&organized_file.new_path, &organized_file.original_path).with_context(
            || {
                format!(
                    "Failed to move file from {} to {}",
                    organized_file.new_path.display(),
                    organized_file.original_path.display()
                )
            },
        )?;

        Ok(())
    }

    /// Handle the cleanup command
    async fn handle_cleanup(
        keep_days: u32,
        keep_count: usize,
        preview: bool,
        verbose: bool,
    ) -> Result<()> {
        println!("🧹 Plex Media Organizer - Cleanup Old Organization Files");
        println!("Keep files newer than: {} days", keep_days);
        println!("Keep at most: {} recent files", keep_count);
        if preview {
            println!("Mode: Preview (dry-run)");
        } else {
            println!("Mode: Live cleanup");
        }
        println!();

        // Find all organization result files
        let current_dir = std::env::current_dir()?;
        let mut json_files = Vec::new();

        for entry in std::fs::read_dir(&current_dir)? {
            let entry = entry?;
            let file_name = entry.file_name().to_string_lossy().to_string();

            if file_name.starts_with("organization_result_") && file_name.ends_with(".json") {
                let metadata = entry.metadata()?;
                let created_time = metadata.created()?.elapsed()?.as_secs() as u32 / 86400; // days

                json_files.push((entry.path(), created_time, metadata));
            }
        }

        if json_files.is_empty() {
            println!("ℹ️  No organization result files found to clean up.");
            return Ok(());
        }

        // Sort by creation time (oldest first)
        json_files.sort_by_key(|(_, days, _)| *days);

        let cutoff_days = keep_days;
        let mut files_to_delete = Vec::new();
        let mut files_to_keep = Vec::new();

        // Apply retention policies
        for (file_path, days_old, metadata) in &json_files {
            let should_delete = *days_old > cutoff_days || files_to_keep.len() >= keep_count;

            if should_delete {
                files_to_delete.push((file_path.clone(), *days_old, metadata.len()));
            } else {
                files_to_keep.push((file_path.clone(), *days_old, metadata.len()));
            }
        }

        // Display cleanup plan
        println!("📋 Cleanup Plan:");
        println!(
            "Files to keep: {} (within {} days, max {})",
            files_to_keep.len(),
            keep_days,
            keep_count
        );
        println!(
            "Files to delete: {} (older than {} days or beyond limit)",
            files_to_delete.len(),
            keep_days
        );

        if verbose && !files_to_keep.is_empty() {
            println!("\n📁 Files to Keep:");
            for (file_path, days_old, size) in &files_to_keep {
                println!(
                    "  • {} ({} days old, {} bytes)",
                    file_path.file_name().unwrap().to_string_lossy(),
                    days_old,
                    size
                );
            }
        }

        if !files_to_delete.is_empty() {
            if verbose {
                println!("\n🗑️  Files to Delete:");
                for (file_path, days_old, size) in &files_to_delete {
                    println!(
                        "  • {} ({} days old, {} bytes)",
                        file_path.file_name().unwrap().to_string_lossy(),
                        days_old,
                        size
                    );
                }
            }

            let total_size: u64 = files_to_delete.iter().map(|(_, _, size)| size).sum();
            println!(
                "\n💾 Space to be freed: {} bytes ({:.2} MB)",
                total_size,
                total_size as f64 / 1024.0 / 1024.0
            );

            // Perform cleanup
            if preview {
                println!("\n🔍 DRY-RUN COMPLETE: No files were actually deleted");
                println!("Run without --preview to perform the actual cleanup.");
            } else {
                println!("\n🧹 Performing cleanup...");
                let mut deleted_count = 0;
                let mut failed_deletions = Vec::new();

                for (file_path, days_old, _) in &files_to_delete {
                    match std::fs::remove_file(file_path) {
                        Ok(()) => {
                            deleted_count += 1;
                            if verbose {
                                println!(
                                    "✅ Deleted: {} ({} days old)",
                                    file_path.file_name().unwrap().to_string_lossy(),
                                    days_old
                                );
                            }
                        }
                        Err(e) => {
                            failed_deletions.push((file_path.clone(), e.to_string()));
                            println!(
                                "❌ Failed to delete {}: {}",
                                file_path.file_name().unwrap().to_string_lossy(),
                                e
                            );
                        }
                    }
                }

                println!("\n📊 Cleanup Results:");
                println!(
                    "Successfully deleted: {}/{}",
                    deleted_count,
                    files_to_delete.len()
                );

                if !failed_deletions.is_empty() {
                    println!("Failed deletions: {}", failed_deletions.len());
                    if verbose {
                        println!("\n❌ Failed Deletions:");
                        for (file_path, error) in failed_deletions {
                            println!(
                                "  • {}: {}",
                                file_path.file_name().unwrap().to_string_lossy(),
                                error
                            );
                        }
                    }
                }

                if deleted_count == files_to_delete.len() {
                    println!("\n✅ Cleanup completed successfully!");
                } else {
                    println!("\n⚠️  Cleanup completed with some failures.");
                }
            }
        } else {
            println!("\n✅ No files need to be cleaned up!");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_creation() {
        let cli = Cli::parse_from(&["plex-media-organizer", "scan", "/test/dir"]);
        match cli.command {
            Commands::Scan {
                directory,
                verbose,
                network_mode,
                max_parallel,
                batch_size,
                min_confidence,
                skip_unmatched,
                no_warnings,
            } => {
                assert_eq!(directory, PathBuf::from("/test/dir"));
                assert!(!verbose);
                assert!(!network_mode);
                assert_eq!(max_parallel, 16);
                assert_eq!(batch_size, 100);
                assert_eq!(min_confidence, 0.7);
                assert!(skip_unmatched);
                assert!(!no_warnings);
            }
            _ => panic!("Expected scan command"),
        }
    }

    #[test]
    fn test_network_mode_scan() {
        let cli = Cli::parse_from(&[
            "plex-media-organizer",
            "scan",
            "/test/dir",
            "--network-mode",
            "--max-parallel",
            "4",
            "--batch-size",
            "25",
        ]);
        match cli.command {
            Commands::Scan {
                directory,
                verbose,
                network_mode,
                max_parallel,
                batch_size,
                min_confidence,
                skip_unmatched,
                no_warnings,
            } => {
                assert_eq!(directory, PathBuf::from("/test/dir"));
                assert!(!verbose);
                assert!(network_mode);
                assert_eq!(max_parallel, 4);
                assert_eq!(batch_size, 25);
                assert_eq!(min_confidence, 0.7);
                assert!(skip_unmatched);
                assert!(!no_warnings);
            }
            _ => panic!("Expected scan command with network mode"),
        }
    }

    #[test]
    fn test_test_command_creation() {
        let cli = Cli::parse_from(&["plex-media-organizer", "test", "/test/file.mkv"]);
        match cli.command {
            Commands::Test {
                path,
                organize,
                preview,
                verbose,
            } => {
                assert_eq!(path, PathBuf::from("/test/file.mkv"));
                assert!(!organize);
                assert!(!preview);
                assert!(!verbose);
            }
            _ => panic!("Expected test command"),
        }
    }

    #[test]
    fn test_test_organize_command_creation() {
        let cli = Cli::parse_from(&[
            "plex-media-organizer",
            "test",
            "/test/dir",
            "--organize",
            "--preview",
        ]);
        match cli.command {
            Commands::Test {
                path,
                organize,
                preview,
                verbose,
            } => {
                assert_eq!(path, PathBuf::from("/test/dir"));
                assert!(organize);
                assert!(preview);
                assert!(!verbose);
            }
            _ => panic!("Expected test organize command"),
        }
    }

    #[test]
    fn test_rollback_command_creation() {
        let cli = Cli::parse_from(&[
            "plex-media-organizer",
            "rollback",
            "operation_result_123.json",
            "--preview",
            "--verbose",
        ]);
        match cli.command {
            Commands::Rollback {
                operation_file,
                preview,
                verbose,
            } => {
                assert_eq!(operation_file, PathBuf::from("operation_result_123.json"));
                assert!(preview);
                assert!(verbose);
            }
            _ => panic!("Expected rollback command"),
        }
    }

    #[test]
    fn test_cleanup_command_creation() {
        let cli = Cli::parse_from(&[
            "plex-media-organizer",
            "cleanup",
            "--keep-days",
            "60",
            "--keep-count",
            "50",
            "--preview",
            "--verbose",
        ]);
        match cli.command {
            Commands::Cleanup {
                keep_days,
                keep_count,
                preview,
                verbose,
            } => {
                assert_eq!(keep_days, 60);
                assert_eq!(keep_count, 50);
                assert!(preview);
                assert!(verbose);
            }
            _ => panic!("Expected cleanup command"),
        }
    }
}
