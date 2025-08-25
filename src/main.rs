//! Main CLI application for Plex Media Organizer

use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod config;
mod parser;
mod tmdb;

use config::AppConfig;
use parser::UnifiedMovieParser;

#[derive(Parser)]
#[command(name = "plex-media-organizer")]
#[command(about = "Intelligent media file organizer following Plex naming conventions")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a single filename
    Parse {
        /// The filename to parse
        filename: String,

        /// Show detailed output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Organize files in a directory
    Organize {
        /// Directory containing files to organize
        directory: PathBuf,

        /// Output directory for organized files
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Show what would be done without actually doing it
        #[arg(short, long)]
        dry_run: bool,

        /// Show detailed output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Show current configuration
    Config {
        /// Show detailed configuration
        #[arg(short, long)]
        verbose: bool,
    },

    /// Setup configuration interactively
    Setup,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { filename, verbose } => {
            handle_parse(filename, verbose).await?;
        }
        Commands::Organize {
            directory,
            output,
            dry_run,
            verbose,
        } => {
            handle_organize(directory, output, dry_run, verbose).await?;
        }
        Commands::Config { verbose } => {
            handle_config(verbose).await?;
        }
        Commands::Setup => {
            handle_setup().await?;
        }
    }

    Ok(())
}

/// Handle the parse command
async fn handle_parse(filename: String, verbose: bool) -> anyhow::Result<()> {
    let config = AppConfig::load()?;
    let parser = UnifiedMovieParser::new(config);

    println!("🎬 Parsing filename: {}", filename);

    match parser.parse_async(&filename).await {
        Ok(result) => {
            println!("✅ Parsed successfully!");
            println!("📝 Title: {}", result.data.title);
            println!("📅 Year: {:?}", result.data.year);
            println!("🎯 Quality: {:?}", result.data.quality);
            println!("📊 Confidence: {:.2}", result.confidence);
            println!("🔧 Method: {}", result.parsing_method);

            if verbose {
                println!("\n📋 Detailed Information:");
                println!("   Language: {:?}", result.data.language);
                println!("   Source: {:?}", result.data.source);
                println!("   Audio: {:?}", result.data.audio);
                println!("   Codec: {:?}", result.data.codec);
                println!("   Group: {:?}", result.data.group);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to parse filename: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Handle the organize command
async fn handle_organize(
    directory: PathBuf,
    output: Option<PathBuf>,
    dry_run: bool,
    verbose: bool,
) -> anyhow::Result<()> {
    let config = AppConfig::load()?;
    let parser = UnifiedMovieParser::new(config);
    let output_dir = output.unwrap_or_else(|| directory.join("organized"));

    if !directory.exists() {
        anyhow::bail!("Directory does not exist: {}", directory.display());
    }

    if !directory.is_dir() {
        anyhow::bail!("Path is not a directory: {}", directory.display());
    }

    println!("🎬 Organizing files in: {}", directory.display());
    println!("📁 Output directory: {}", output_dir.display());

    if dry_run {
        println!("🔍 DRY RUN MODE - No files will be moved");
    }

    // Find movie files
    let movie_files = find_movie_files(&directory)?;

    if movie_files.is_empty() {
        println!("⚠️  No movie files found in directory");
        return Ok(());
    }

    println!("📊 Found {} movie files", movie_files.len());

    let mut organized_count = 0;
    let mut skipped_count = 0;
    let mut failed_count = 0;

    for file_path in &movie_files {
        let filename = file_path.file_name().unwrap().to_string_lossy();

        if verbose {
            println!("\n🔍 Processing: {}", filename);
        }

        match parser.parse_async(&filename).await {
            Ok(result) => {
                if result.confidence < 0.5 {
                    if verbose {
                        println!("   ⏭️  Skipped (low confidence: {:.2})", result.confidence);
                    }
                    skipped_count += 1;
                    continue;
                }

                let new_filename = format!(
                    "{}.{}",
                    result.data.title,
                    file_path.extension().unwrap_or_default().to_string_lossy()
                );
                let new_path = output_dir.join(&new_filename);

                if verbose {
                    println!("   ✅ {} → {}", filename, new_filename);
                }

                if !dry_run {
                    // Create output directory if it doesn't exist
                    if !output_dir.exists() {
                        std::fs::create_dir_all(&output_dir)?;
                    }

                    // Move the file
                    std::fs::rename(file_path, &new_path)?;
                }

                organized_count += 1;
            }
            Err(e) => {
                if verbose {
                    println!("   ❌ Failed to parse: {}", e);
                }
                failed_count += 1;
            }
        }
    }

    println!("\n📊 Organization Summary:");
    println!("   ✅ Organized: {}", organized_count);
    println!("   ⏭️  Skipped: {}", skipped_count);
    println!("   ❌ Failed: {}", failed_count);

    if dry_run {
        println!("\n💡 Run without --dry-run to actually organize the files");
    }

    Ok(())
}

/// Handle the config command
async fn handle_config(verbose: bool) -> anyhow::Result<()> {
    let config = AppConfig::load()?;

    println!("⚙️  Configuration");
    println!("================");
    println!("📋 Version: {}", config.version);

    if config.apis.tmdb_api_key.is_some() {
        println!("🔑 TMDB API Key: [SET]");
    } else {
        println!("🔑 TMDB API Key: [NOT SET]");
    }

    if verbose {
        println!("\n📊 Parsing Configuration:");
        println!(
            "   TMDB Min Confidence: {}",
            config.parsing.tmdb.min_confidence
        );
        println!(
            "   Prioritize English: {}",
            config.parsing.tmdb.prioritize_english
        );
        println!(
            "   Include Quality: {}",
            config.parsing.output.include_quality
        );
        println!("   Include Year: {}", config.parsing.output.include_year);

        println!("\n📁 Organization Configuration:");
        println!(
            "   Create Plex Structure: {}",
            config.organization.create_plex_structure
        );
        println!("   Move Files: {}", config.organization.move_files);
        println!(
            "   Preserve Original Names: {}",
            config.organization.preserve_original_names
        );
    }

    Ok(())
}

/// Handle the setup command
async fn handle_setup() -> anyhow::Result<()> {
    println!("🔧 Interactive Setup");
    println!("===================");

    let mut config = AppConfig::default();

    // TMDB API Key setup
    println!("\n🔑 TMDB API Key Setup");
    println!("Get a free API key from: https://www.themoviedb.org/settings/api");

    let mut input = String::new();
    print!("Enter your TMDB API key (or press Enter to skip): ");
    std::io::stdin().read_line(&mut input)?;
    let api_key = input.trim();

    if !api_key.is_empty() {
        config.apis.tmdb_api_key = Some(api_key.to_string());
        println!("✅ TMDB API key saved");
    } else {
        println!("⚠️  TMDB API key not set - some features will be limited");
    }

    // Output preferences
    println!("\n📊 Output Preferences");

    print!("Include quality in titles? (y/N): ");
    input.clear();
    std::io::stdin().read_line(&mut input)?;
    config.parsing.output.include_quality = input.trim().to_lowercase() == "y";

    print!("Include year in titles? (Y/n): ");
    input.clear();
    std::io::stdin().read_line(&mut input)?;
    config.parsing.output.include_year = input.trim().to_lowercase() != "n";

    // Save configuration
    config.save()?;
    println!("\n✅ Configuration saved successfully!");

    Ok(())
}

/// Find movie files in a directory
fn find_movie_files(directory: &PathBuf) -> anyhow::Result<Vec<PathBuf>> {
    let mut movie_files = Vec::new();
    let movie_extensions = ["mkv", "mp4", "avi", "mov", "wmv", "flv", "webm"];

    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file()
            && let Some(extension) = path.extension()
            && let Some(ext_str) = extension.to_str()
            && movie_extensions.contains(&ext_str.to_lowercase().as_str())
        {
            movie_files.push(path);
        }
    }

    Ok(movie_files)
}
