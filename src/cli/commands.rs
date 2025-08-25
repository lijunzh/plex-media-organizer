//! CLI command definitions

use clap::{Args, Parser, Subcommand};

use crate::cli::handlers::cleanup::{CleanupArgs, handle_cleanup};
use crate::cli::handlers::config::{ConfigArgs, handle_config};
use crate::cli::handlers::migrate::{MigrateArgs, handle_migrate};
use crate::cli::handlers::organize::{OrganizeArgs, handle_organize};
use crate::cli::handlers::rollback::{RollbackArgs, handle_rollback};
use crate::cli::handlers::scan::{ScanArgs, handle_scan};
use crate::cli::handlers::setup::{SetupArgs, handle_setup};
// use crate::cli::handlers::test::{TestArgs, handle_test}; // Temporarily disabled

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
    /// Test filename parsing
    // Test(TestArgs), // Temporarily disabled
    /// Scan directories for media files
    Scan(ScanArgs),
    /// Setup initial configuration
    Setup(SetupArgs),
    /// Show or edit configuration
    Config(ConfigArgs),
    /// Organize media files
    Organize(OrganizeArgs),
    /// Rollback previous organization operations
    Rollback(RollbackArgs),
    /// Clean up old operation history
    Cleanup(CleanupArgs),
    /// Manage technical terms for title filtering
    Terms(TermsArgs),
    /// Migrate configuration to latest defaults
    Migrate(MigrateArgs),
}

/// Arguments for the terms command
#[derive(Debug, Clone, Args)]
pub struct TermsArgs {
    /// List all technical terms
    #[arg(long)]
    pub list: bool,

    /// Add a new technical term
    #[arg(long)]
    pub add: Option<String>,

    /// Remove a technical term
    #[arg(long)]
    pub remove: Option<String>,

    /// Show terms by category
    #[arg(long)]
    pub categories: bool,

    /// Export terms to a file
    #[arg(long)]
    pub export: Option<String>,

    /// Import terms from a file
    #[arg(long)]
    pub import: Option<String>,
}

impl Cli {
    /// Run the CLI application
    pub async fn run() -> anyhow::Result<()> {
        let cli = Cli::parse();

        match cli.command {
            Commands::Scan(args) => handle_scan(args).await,
            Commands::Setup(args) => handle_setup(args).await,
            Commands::Config(args) => handle_config(args).await,
            // Commands::Test(args) => handle_test(args).await, // Temporarily disabled
            Commands::Organize(args) => handle_organize(args).await,
            Commands::Rollback(args) => handle_rollback(args).await,
            Commands::Cleanup(args) => handle_cleanup(args).await,
            Commands::Terms(_args) => {
                // This case is not yet implemented in handlers,
                // so we'll just print a placeholder message.
                println!("Terms management command not yet implemented.");
                Ok(())
            }
            Commands::Migrate(args) => handle_migrate(args).await,
        }
    }
}
