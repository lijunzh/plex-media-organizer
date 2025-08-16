//! Command-line interface for the Plex Media Organizer

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::config::AppConfig;
use crate::movie_parser::MovieParser;
use crate::scanner::Scanner;
use crate::tmdb_client::TmdbClient;

/// Plex Media Organizer - Intelligent media file organization
#[derive(Parser)]
#[command(name = "plex-media-organizer")]
#[command(about = "Intelligent media file organizer following Plex naming conventions")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a directory for media files
    Scan {
        /// Directory to scan
        #[arg(value_name = "DIRECTORY")]
        directory: PathBuf,
        
        /// Show detailed output
        #[arg(short, long)]
        verbose: bool,
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
    
    /// Test parsing with a single file
    Test {
        /// File to test parsing
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

}

impl Cli {
    /// Run the CLI application
    pub async fn run() -> Result<()> {
        let cli = Cli::parse();
        
        match cli.command {
            Commands::Scan { directory, verbose } => {
                Self::handle_scan(directory, verbose).await
            }
            Commands::Setup { force } => {
                Self::handle_setup(force).await
            }
            Commands::Config { path } => {
                Self::handle_config(path).await
            }
            Commands::Test { file } => {
                Self::handle_test(file).await
            }

        }
    }
    
    /// Handle the scan command
    async fn handle_scan(directory: PathBuf, verbose: bool) -> Result<()> {
        println!("🎬 Plex Media Organizer - Scanning Directory");
        println!("Directory: {}", directory.display());
        println!();
        
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
        let tmdb_client = if let Some(api_key) = config.apis.tmdb_api_key {
            Some(TmdbClient::new(api_key))
        } else {
            None
        };
        
        // Create movie parser and scanner
        let movie_parser = MovieParser::new(tmdb_client);
        let scanner = Scanner::new(movie_parser);
        
        // Scan directory
        let scan_result = scanner.scan_directory(&directory).await
            .context("Failed to scan directory")?;
        
        // Display results
        Self::display_scan_results(&scan_result, verbose);
        
        Ok(())
    }
    
    /// Handle the setup command
    async fn handle_setup(force: bool) -> Result<()> {
        println!("🔧 Plex Media Organizer - Setup");
        
        // Check if config already exists
        if !force {
            if let Ok(config) = AppConfig::load() {
                if config.apis.tmdb_api_key.is_some() {
                    println!("Configuration already exists. Use --force to reconfigure.");
                    return Ok(());
                }
            }
        }
        
        // Run interactive setup
        let _config = AppConfig::interactive_setup()
            .context("Setup failed")?;
        
        println!("✅ Setup completed successfully!");
        println!("Configuration saved to: {}", AppConfig::get_config_dir()?.join("config.toml").display());
        
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
                println!("TMDB API Key: {}", 
                    if config.apis.tmdb_api_key.is_some() { "✅ Set" } else { "❌ Not set" });
                println!("TVDB API Key: {}", 
                    if config.apis.tvdb_api_key.is_some() { "✅ Set" } else { "❌ Not set" });
                println!("MusicBrainz User Agent: {}", 
                    if config.apis.musicbrainz_user_agent.is_some() { "✅ Set" } else { "❌ Not set" });
                println!("AniDB Username: {}", 
                    if config.apis.anidb_username.is_some() { "✅ Set" } else { "❌ Not set" });
            }
            Err(e) => {
                println!("❌ Failed to load configuration: {}", e);
                println!("Run 'setup' to create configuration.");
            }
        }
        
        Ok(())
    }
    
    /// Handle the test command
    async fn handle_test(file: PathBuf) -> Result<()> {
        println!("🧪 Plex Media Organizer - Testing File Parsing");
        println!("File: {}", file.display());
        println!();
        
        if !file.exists() {
            anyhow::bail!("File does not exist: {}", file.display());
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
        let tmdb_client = config.apis.tmdb_api_key
            .map(|api_key| TmdbClient::new(api_key));
        
        // Create movie parser and test parsing
        let movie_parser = MovieParser::new(tmdb_client);
        
        match movie_parser.parse_movie(&file).await {
            Ok(result) => {
                println!("✅ Parsing successful!");
                println!("Title: {}", result.parsed_metadata.title.as_deref().unwrap_or("Unknown"));
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
        
        Ok(())
    }
    

    
    /// Display scan results
    fn display_scan_results(scan_result: &crate::types::ScanResult, verbose: bool) {
        println!("📊 Scan Results:");
        println!("Directory: {}", scan_result.directory.display());
        println!("Total files: {}", scan_result.statistics.total_files);
        println!("Media files: {}", scan_result.files.len());
        println!("Successfully parsed: {}", scan_result.statistics.parsed_files);
        println!("Failed to parse: {}", scan_result.statistics.failed_files);
        println!("Success rate: {:.1}%", scan_result.statistics.success_rate * 100.0);
        println!("Average confidence: {:.1}%", scan_result.statistics.average_confidence * 100.0);
        println!("Scan duration: {:.2}s", scan_result.statistics.duration_seconds);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cli_creation() {
        let cli = Cli::parse_from(&["plex-media-organizer", "scan", "/test/dir"]);
        match cli.command {
            Commands::Scan { directory, verbose } => {
                assert_eq!(directory, PathBuf::from("/test/dir"));
                assert!(!verbose);
            }
            _ => panic!("Expected scan command"),
        }
    }
}
