//! Setup command handler for interactive configuration

use crate::{
    config::AppConfig,
    database::DatabaseManager,
    output::{print_section_header, print_subsection_header},
};
use anyhow::Result;
use clap::Args;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct SetupArgs {
    /// Force reconfiguration even if config exists
    #[arg(short, long)]
    force: bool,
}

pub async fn handle_setup(args: SetupArgs) -> Result<()> {
    print_section_header("Plex Media Organizer Setup");

    // Check if configuration already exists
    let config_dir = AppConfig::get_config_dir()?;
    let config_path = config_dir.join("config.toml");
    let config_exists = config_path.exists();

    if config_exists && !args.force {
        println!(
            "⚠️  Configuration file already exists at: {}",
            config_path.display()
        );
        println!("   Use --force to reconfigure or 'config' command to view current settings.");
        return Ok(());
    }

    if config_exists && args.force {
        println!("🔄 Reconfiguring existing setup...");
    } else {
        println!("🆕 Welcome! Let's set up your Plex Media Organizer configuration.");
    }

    // Load existing config if available, otherwise use defaults
    let mut config = if config_exists {
        AppConfig::load().unwrap_or_default()
    } else {
        AppConfig::default()
    };

    println!(
        "\n📝 We'll walk through the essential settings. Press Enter for defaults shown in [brackets]."
    );

    // TMDB Configuration
    setup_tmdb_config(&mut config).await?;

    // Database Configuration
    setup_database_config(&mut config).await?;

    // Organization Configuration
    setup_organization_config(&mut config).await?;

    // Save configuration
    print_subsection_header("Saving Configuration");
    config.save()?;
    println!("✅ Configuration saved to: {}", config_path.display());

    // Initialize database
    print_subsection_header("Initializing Database");
    let db_path = PathBuf::from(&config.database.path);

    std::fs::create_dir_all(db_path.parent().unwrap())?;
    let _db = DatabaseManager::new(&db_path).await?;
    println!("✅ Database initialized at: {}", db_path.display());

    // Setup complete
    print_subsection_header("Setup Complete");
    println!("🎉 Setup completed successfully!");
    println!("\n📚 Next steps:");
    println!("   • Run 'scan <directory>' to analyze your media files");
    println!("   • Run 'test <directory>' to test parsing with sample files");
    println!("   • Run 'config' to view your current configuration");
    println!("   • Run 'organize <directory>' to start organizing your media");

    Ok(())
}

async fn setup_tmdb_config(config: &mut AppConfig) -> Result<()> {
    print_subsection_header("TMDB Configuration");
    println!("📽️  The Movie Database (TMDB) provides accurate movie metadata.");
    println!("   Get your free API key at: https://www.themoviedb.org/settings/api");

    let current_key = config
        .apis
        .tmdb_api_key
        .as_ref()
        .map(|k| format!("{}***", &k[..4.min(k.len())]))
        .unwrap_or_else(|| "none".to_string());

    loop {
        print!("\n🔑 TMDB API Key [{}]: ", current_key);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() && config.apis.tmdb_api_key.is_some() {
            // Keep existing key
            break;
        } else if input.is_empty() {
            println!("⚠️  TMDB API key is required for movie metadata. Please enter a valid key.");
            continue;
        } else if input.len() < 10 {
            println!("⚠️  API key seems too short. Please check and enter a valid key.");
            continue;
        } else {
            // Set new key
            config.apis.tmdb_api_key = Some(input.to_string());
            println!("✅ TMDB API key configured successfully!");
            break;
        }
    }

    Ok(())
}

async fn setup_organization_config(config: &mut AppConfig) -> Result<()> {
    print_subsection_header("Organization Configuration");
    println!("📋 Configure organization preferences.");

    // Preferred quality
    let current_quality = config
        .organization
        .quality
        .preferred_quality
        .as_deref()
        .unwrap_or("1080p");
    print!("\n🎬 Preferred video quality [{}]: ", current_quality);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if !input.is_empty() {
        config.organization.quality.preferred_quality = Some(input.to_string());
    }

    // Original title preference
    let current_prefer_original = config.organization.original_titles.prefer_original_titles;
    print!(
        "\n🌏 Prefer original (non-English) titles? [{}]: ",
        if current_prefer_original { "yes" } else { "no" }
    );
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    if !input.is_empty() {
        config.organization.original_titles.prefer_original_titles =
            input == "y" || input == "yes" || input == "true";
    }

    // Minimum confidence
    let current_min_confidence = config.organization.matching.min_confidence_threshold;
    print!(
        "\n🎯 Minimum confidence threshold for organizing files (0.0-1.0) [{}]: ",
        current_min_confidence
    );
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if !input.is_empty() {
        match input.parse::<f32>() {
            Ok(confidence) if (0.0..=1.0).contains(&confidence) => {
                config.organization.matching.min_confidence_threshold = confidence;
                println!("✅ Confidence threshold set to: {:.2}", confidence);
            }
            _ => {
                println!("⚠️  Invalid confidence value. Must be between 0.0 and 1.0.");
            }
        }
    }

    Ok(())
}

async fn setup_database_config(config: &mut AppConfig) -> Result<()> {
    print_subsection_header("Database Configuration");
    println!("🗄️  Configure caching and database settings.");

    let current_db_path = &config.database.path;

    print!("\n💾 Database path [{}]: ", current_db_path);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if !input.is_empty() {
        let path = PathBuf::from(input);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                print!("⚠️  Directory does not exist. Create it? (y/N): ");
                io::stdout().flush()?;
                let mut create_input = String::new();
                io::stdin().read_line(&mut create_input)?;

                if create_input.trim().to_lowercase() == "y" {
                    std::fs::create_dir_all(parent)?;
                    config.database.path = input.to_string();
                    println!("✅ Created directory and set database path: {}", input);
                } else {
                    println!("⚠️  Using default database path.");
                }
            } else {
                config.database.path = input.to_string();
            }
        }
    }

    // Cache TTL
    let current_ttl = config.database.cache_ttl_hours;
    print!("\n⏰ Cache TTL in hours [{}]: ", current_ttl);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if !input.is_empty() {
        match input.parse::<i64>() {
            Ok(ttl) if ttl > 0 => {
                config.database.cache_ttl_hours = ttl;
                println!("✅ Cache TTL set to: {} hours", ttl);
            }
            _ => {
                println!("⚠️  Invalid TTL value. Must be a positive integer.");
            }
        }
    }

    Ok(())
}
