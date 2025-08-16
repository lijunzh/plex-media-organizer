//! Plex Media Organizer - Main entry point

use anyhow::Result;
use plex_media_organizer::cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Run the CLI application
    Cli::run().await
}
