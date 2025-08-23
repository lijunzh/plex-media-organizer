//! Config command handler for configuration management

use crate::{
    config::AppConfig,
    output::{print_section_header, print_subsection_header},
};
use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
pub struct ConfigArgs {
    /// Show configuration file path
    #[arg(short, long)]
    path: bool,
}

pub async fn handle_config(args: ConfigArgs) -> Result<()> {
    print_section_header("Configuration");

    if args.path {
        // Show config file path
        let config_dir = AppConfig::get_config_dir()?;
        let config_path = config_dir.join("config.toml");
        println!("📁 Configuration file: {}", config_path.display());
        println!("📂 Configuration directory: {}", config_dir.display());
        return Ok(());
    }

    // Load and display current configuration
    match AppConfig::load() {
        Ok(config) => {
            display_config(&config).await?;
        }
        Err(e) => {
            println!("❌ Failed to load configuration: {}", e);
            println!("💡 Run 'setup' command to create a new configuration.");
            return Ok(());
        }
    }

    Ok(())
}

async fn display_config(config: &AppConfig) -> Result<()> {
    // API Configuration
    print_subsection_header("API Configuration");
    if let Some(api_key) = &config.apis.tmdb_api_key {
        let masked_key = if api_key.len() > 8 {
            format!("{}***{}", &api_key[..4], &api_key[api_key.len() - 4..])
        } else {
            "***".to_string()
        };
        println!("🔑 TMDB API Key: {}", masked_key);
    } else {
        println!("⚠️  TMDB API Key: Not configured");
    }

    // Database Configuration
    print_subsection_header("Database Configuration");
    println!("💾 Database Path: {}", config.database.path);
    println!("🔗 Max Connections: {}", config.database.max_connections);
    println!("⏰ Cache TTL: {} hours", config.database.cache_ttl_hours);
    println!(
        "📝 WAL Mode: {}",
        if config.database.enable_wal {
            "enabled"
        } else {
            "disabled"
        }
    );

    // Organization Configuration
    print_subsection_header("Organization Configuration");

    // Quality
    if let Some(quality) = &config.organization.quality.preferred_quality {
        println!("🎬 Preferred Quality: {}", quality);
    } else {
        println!("🎬 Preferred Quality: Not set");
    }

    // Original titles
    println!(
        "🌏 Prefer Original Titles: {}",
        if config.organization.original_titles.prefer_original_titles {
            "Yes"
        } else {
            "No"
        }
    );
    println!(
        "📄 Include English Subtitle: {}",
        if config.organization.original_titles.include_english_subtitle {
            "Yes"
        } else {
            "No"
        }
    );
    println!(
        "🔄 Fallback to English: {}",
        if config
            .organization
            .original_titles
            .fallback_to_english_on_error
        {
            "Yes"
        } else {
            "No"
        }
    );

    // Matching
    println!(
        "🎯 Min Confidence Threshold: {:.2}",
        config.organization.matching.min_confidence_threshold
    );
    println!(
        "⏭️  Skip Unmatched Movies: {}",
        if config.organization.matching.skip_unmatched_movies {
            "Yes"
        } else {
            "No"
        }
    );
    println!(
        "⚠️  Warn on Low Confidence: {}",
        if config.organization.matching.warn_on_low_confidence {
            "Yes"
        } else {
            "No"
        }
    );
    println!(
        "❓ Allow Unknown Year: {}",
        if config.organization.matching.allow_unknown_year {
            "Yes"
        } else {
            "No"
        }
    );

    // Statistics
    print_subsection_header("Configuration Statistics");
    println!(
        "📊 Technical Terms: {} categories configured",
        count_technical_terms(config)
    );
    println!(
        "🏷️  Language Codes: {} codes",
        config.organization.language.language_codes.len()
    );
    println!(
        "🎭 Known Titles: {} preserved titles",
        config.organization.title_preservation.known_titles.len()
    );
    println!(
        "📝 Common Words: {} preserved words",
        config.organization.title_preservation.common_words.len()
    );
    println!(
        "🚫 Content Filters: {} patterns",
        config
            .organization
            .content_filtering
            .problematic_patterns
            .len()
    );

    // Configuration Status
    print_subsection_header("Configuration Status");
    let status = check_config_status(config);
    for (category, is_ok, message) in status {
        let icon = if is_ok { "✅" } else { "⚠️ " };
        println!("{} {}: {}", icon, category, message);
    }

    Ok(())
}

fn count_technical_terms(config: &AppConfig) -> usize {
    let mut count = 0;

    if !config
        .organization
        .technical_terms
        .release_groups
        .is_empty()
    {
        count += 1;
    }
    if !config
        .organization
        .technical_terms
        .video_audio_terms
        .is_empty()
    {
        count += 1;
    }
    if !config
        .organization
        .technical_terms
        .source_platform_terms
        .is_empty()
    {
        count += 1;
    }
    if !config
        .organization
        .technical_terms
        .file_format_terms
        .is_empty()
    {
        count += 1;
    }
    if !config
        .organization
        .technical_terms
        .special_edition_terms
        .is_empty()
    {
        count += 1;
    }
    if !config.organization.technical_terms.custom_terms.is_empty() {
        count += 1;
    }

    count
}

fn check_config_status(config: &AppConfig) -> Vec<(&'static str, bool, &'static str)> {
    let mut status = Vec::new();

    // Check API configuration
    let has_tmdb_key = config.apis.tmdb_api_key.is_some();
    status.push((
        "TMDB API",
        has_tmdb_key,
        if has_tmdb_key {
            "Configured"
        } else {
            "Missing API key"
        },
    ));

    // Check database path
    let db_path = std::path::Path::new(&config.database.path);
    let db_dir_exists = db_path.parent().map(|p| p.exists()).unwrap_or(false);
    status.push((
        "Database",
        db_dir_exists,
        if db_dir_exists {
            "Path accessible"
        } else {
            "Directory does not exist"
        },
    ));

    // Check confidence threshold
    let confidence_ok = config.organization.matching.min_confidence_threshold >= 0.0
        && config.organization.matching.min_confidence_threshold <= 1.0;
    status.push((
        "Confidence Threshold",
        confidence_ok,
        if confidence_ok {
            "Valid range"
        } else {
            "Invalid range"
        },
    ));

    // Check if basic patterns are configured
    let has_patterns = !config
        .organization
        .technical_terms
        .release_groups
        .is_empty();
    status.push((
        "Parsing Patterns",
        has_patterns,
        if has_patterns {
            "Configured"
        } else {
            "No patterns defined"
        },
    ));

    status
}
