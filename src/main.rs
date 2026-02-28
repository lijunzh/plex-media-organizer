//! CLI entry point for plex-media-organizer.

use clap::Parser;
use tracing_subscriber::EnvFilter;

mod cli;

fn main() -> anyhow::Result<()> {
    let args = cli::Cli::parse();

    // Configure tracing based on verbosity
    let filter = match args.verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(filter)),
        )
        .with_target(false)
        .init();

    cli::run(args)
}
