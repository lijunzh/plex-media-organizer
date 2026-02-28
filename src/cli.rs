//! CLI command dispatch — thin layer over library functions.

use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::info;

use plex_media_organizer::config::AppConfig;
use plex_media_organizer::enricher::Enricher;
use plex_media_organizer::models::MediaType;
use plex_media_organizer::organizer;
use plex_media_organizer::parser;
use plex_media_organizer::scanner::{self, ScanOptions};
use plex_media_organizer::utils;

// ── CLI definition ─────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(
    name = "plex-org",
    about = "Organize media files into Plex-compatible folder structures",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Increase verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Config file path
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Scan a directory and show discovered media files.
    Scan {
        /// Directory to scan.
        path: PathBuf,
    },
    /// Preview the organization plan (dry-run).
    Plan {
        /// Source directory.
        path: PathBuf,
        /// Destination root directory.
        #[arg(short, long)]
        dest: PathBuf,
        /// File strategy: move, copy, or symlink.
        #[arg(short, long, default_value = "move")]
        strategy: String,
    },
    /// Execute the organization plan.
    Organize {
        /// Source directory.
        path: PathBuf,
        /// Destination root directory.
        #[arg(short, long)]
        dest: PathBuf,
        /// File strategy: move, copy, or symlink.
        #[arg(short, long, default_value = "move")]
        strategy: String,
        /// Actually execute (without this flag, acts as dry-run).
        #[arg(long)]
        execute: bool,
    },
    /// Reverse the last organize operation.
    Undo,
    /// Show current configuration.
    Config,
}

// ── Command dispatch ───────────────────────────────────────────────────────

pub fn run(cli: Cli) -> Result<()> {
    let config = AppConfig::load_or_default(cli.config.as_deref());

    match cli.command {
        Command::Scan { path } => cmd_scan(&path, &config),
        Command::Plan {
            path,
            dest,
            strategy,
        } => cmd_plan(&path, &dest, &strategy, &config),
        Command::Organize {
            path,
            dest,
            strategy,
            execute,
        } => cmd_organize(&path, &dest, &strategy, execute, &config),
        Command::Undo => cmd_undo(),
        Command::Config => cmd_config(&config),
    }
}

// ── Command implementations ────────────────────────────────────────────────

fn cmd_scan(path: &Path, _config: &AppConfig) -> Result<()> {
    let opts = ScanOptions {
        min_video_size: 0, // Don't filter by size in scan (show everything)
        ..Default::default()
    };
    let files = scanner::scan_directory(path, &opts)?;

    if files.is_empty() {
        println!("No media files found in {}", path.display());
        return Ok(());
    }

    // Table header
    println!(
        "{:<50} {:<8} {:<40} {:<6} {:<8} {:<10}",
        "Filename", "Type", "Title", "Year", "S/E", "Size"
    );
    println!("{}", "-".repeat(122));

    for file in &files {
        let parsed = parser::parse_media_file(file);

        let type_str = match parsed.media_type {
            MediaType::Movie => "movie",
            MediaType::Tv => "tv",
            MediaType::Music => "music",
            MediaType::Unknown => "?",
        };

        let se = match (parsed.season, parsed.episode) {
            (Some(s), Some(e)) => format!("S{s:02}E{e:02}"),
            _ => String::new(),
        };

        let year_str = parsed.year.map(|y| y.to_string()).unwrap_or_default();

        let name = truncate(&file.full_name(), 48);
        let title = truncate(&parsed.title, 38);

        println!(
            "{:<50} {:<8} {:<40} {:<6} {:<8} {:<10}",
            name,
            type_str,
            title,
            year_str,
            se,
            utils::format_size(file.size_bytes),
        );
    }

    println!("\n{} media files found.", files.len());
    Ok(())
}

fn cmd_plan(path: &Path, dest: &Path, strategy: &str, config: &AppConfig) -> Result<()> {
    let items = scan_parse_enrich(path, config)?;

    if items.is_empty() {
        println!("No media files found.");
        return Ok(());
    }

    let actions = organizer::plan_actions(&items, dest, config, strategy);

    println!("\n📋 Plan ({} actions):\n", actions.len());
    for action in &actions {
        println!(
            "  {} → {}",
            action.source.display(),
            action.destination.display()
        );
    }
    println!("\nDry-run complete. Use `organize --execute` to apply.");
    Ok(())
}

fn cmd_organize(
    path: &Path,
    dest: &Path,
    strategy: &str,
    execute: bool,
    config: &AppConfig,
) -> Result<()> {
    let items = scan_parse_enrich(path, config)?;

    if items.is_empty() {
        println!("No media files found.");
        return Ok(());
    }

    let actions = organizer::plan_actions(&items, dest, config, strategy);

    if !execute {
        println!("\n📋 Plan ({} actions):\n", actions.len());
        for action in &actions {
            println!(
                "  {} → {}",
                action.source.display(),
                action.destination.display()
            );
        }
        println!("\nDry-run. Add --execute to apply.");
        return Ok(());
    }

    let undo_dir = dirs_undo();
    let manifest = organizer::execute_actions(&actions, &undo_dir)?;
    println!(
        "\n✅ Organized {} files. Undo manifest saved.",
        manifest.entries.len()
    );
    Ok(())
}

fn cmd_undo() -> Result<()> {
    let undo_dir = dirs_undo();
    let reversed = organizer::undo_last(&undo_dir)?;
    println!("Undo complete: {reversed} files reversed.");
    Ok(())
}

fn cmd_config(config: &AppConfig) -> Result<()> {
    let toml_str = toml::to_string_pretty(config)?;
    println!("{toml_str}");
    Ok(())
}

// ── Helpers ─────────────────────────────────────────────────────────────────

/// Run the full scan → parse → enrich pipeline, returning items for plan/organize.
fn scan_parse_enrich(
    path: &Path,
    config: &AppConfig,
) -> Result<Vec<(PathBuf, plex_media_organizer::models::EnrichedMedia)>> {
    let files = scanner::scan_directory(path, &ScanOptions::default())?;
    let enricher = Enricher::new(config.clone());

    let items: Vec<_> = files
        .iter()
        .map(|f| {
            let parsed = parser::parse_media_file(f);
            let enriched = enricher.enrich(parsed);
            (f.source_path.clone(), enriched)
        })
        .collect();

    info!("{} files scanned and enriched", items.len());
    Ok(items)
}

/// Default undo directory: ~/.plex-organizer/undo/
fn dirs_undo() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".plex-organizer").join("undo")
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max - 3])
    }
}
