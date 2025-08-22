//! CLI command definitions

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Plex Media Organizer - Intelligent media file organization
#[derive(Parser)]
#[command(name = "plex-media-organizer")]
#[command(about = "Intelligent media file organizer following Plex naming conventions")]
#[command(version = "0.1.0")]
#[derive(Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
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

        /// Custom database path (overrides config file and environment variable)
        #[arg(long)]
        database_path: Option<PathBuf>,
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

    /// Organize media files
    Organize {
        /// Directory to organize
        #[arg(value_name = "DIRECTORY")]
        directory: PathBuf,

        /// Preview organization changes (dry-run)
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
    },

    /// Rollback previous organization operation
    Rollback {
        /// Operation file to rollback
        #[arg(value_name = "OPERATION_FILE")]
        operation_file: PathBuf,

        /// Preview rollback changes (dry-run)
        #[arg(short, long)]
        preview: bool,

        /// Show detailed output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Clean up old operation files and backups
    Cleanup {
        /// Keep operation files for this many days
        #[arg(long, default_value = "30")]
        keep_days: u32,

        /// Keep this many most recent operation files
        #[arg(long, default_value = "10")]
        keep_count: usize,

        /// Preview cleanup changes (dry-run)
        #[arg(short, long)]
        preview: bool,

        /// Show detailed output
        #[arg(short, long)]
        verbose: bool,
    },
}

impl Cli {
    /// Run the CLI application
    pub async fn run() -> anyhow::Result<()> {
        let cli = Cli::parse();

        match cli.command {
            Commands::Scan { .. } => {
                // TODO: Implement scan handler
                println!("Scan command - implementation in progress");
                Ok(())
            }
            Commands::Setup { .. } => {
                // TODO: Implement setup handler
                println!("Setup command - implementation in progress");
                Ok(())
            }
            Commands::Config { .. } => {
                // TODO: Implement config handler
                println!("Config command - implementation in progress");
                Ok(())
            }
            Commands::Test { .. } => {
                // TODO: Implement test handler
                println!("Test command - implementation in progress");
                Ok(())
            }
            Commands::Organize { .. } => {
                // TODO: Implement organize handler
                println!("Organize command - implementation in progress");
                Ok(())
            }
            Commands::Rollback { .. } => {
                // TODO: Implement rollback handler
                println!("Rollback command - implementation in progress");
                Ok(())
            }
            Commands::Cleanup { .. } => {
                // TODO: Implement cleanup handler
                println!("Cleanup command - implementation in progress");
                Ok(())
            }
        }
    }
}
