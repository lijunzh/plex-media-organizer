//! CLI command definitions

use clap::{Parser, Subcommand};

use crate::cli::handlers::cleanup::{CleanupArgs, handle_cleanup};
use crate::cli::handlers::config::{ConfigArgs, handle_config};
use crate::cli::handlers::organize::{OrganizeArgs, handle_organize};
use crate::cli::handlers::rollback::{RollbackArgs, handle_rollback};
use crate::cli::handlers::scan::{ScanArgs, handle_scan};
use crate::cli::handlers::setup::{SetupArgs, handle_setup};
use crate::cli::handlers::test::{TestArgs, handle_test};

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
    Scan(ScanArgs),

    /// Set up configuration interactively
    Setup(SetupArgs),

    /// Show current configuration
    Config(ConfigArgs),

    /// Test parsing and organization
    Test(TestArgs),

    /// Organize media files
    Organize(OrganizeArgs),

    /// Rollback previous organization operation
    Rollback(RollbackArgs),

    /// Clean up old operations and optimize database
    Cleanup(CleanupArgs),
}

impl Cli {
    /// Run the CLI application
    pub async fn run() -> anyhow::Result<()> {
        let cli = Cli::parse();

        match cli.command {
            Commands::Scan(args) => handle_scan(args).await,
            Commands::Setup(args) => handle_setup(args).await,
            Commands::Config(args) => handle_config(args).await,
            Commands::Test(args) => handle_test(args).await,
            Commands::Organize(args) => handle_organize(args).await,
            Commands::Rollback(args) => handle_rollback(args).await,
            Commands::Cleanup(args) => handle_cleanup(args).await,
        }
    }
}
