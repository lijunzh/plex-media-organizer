//! Configuration management for the Plex Media Organizer

use anyhow::{Context, Result};
use config::{Environment, File};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    /// API configuration for external services
    pub apis: ApiConfig,
    /// Organization preferences
    pub organization: OrganizationConfig,
}

/// API configuration for external services
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiConfig {
    /// TMDB API key for movie data
    pub tmdb_api_key: Option<String>,
}

/// Organization preferences
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrganizationConfig {
    /// Quality preferences
    pub quality: QualityConfig,
    /// CJK (Chinese/Japanese/Korean) title preferences
    pub original_titles: OriginalTitleConfig,
    /// Confidence and matching preferences
    pub matching: MatchingConfig,
}

/// Quality preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityConfig {
    /// Preferred quality (720p, 1080p, 4K)
    pub preferred_quality: Option<String>,
}

/// CJK (Chinese/Japanese/Korean) title handling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginalTitleConfig {
    /// Use original titles for organization instead of English
    pub prefer_original_titles: bool,
    /// Include English title in brackets: 英雄 [Hero] (2002)
    pub include_english_subtitle: bool,
    /// Fallback to English if original title causes file system issues
    pub fallback_to_english_on_error: bool,
    /// Always preserve original title in metadata
    pub preserve_original_in_metadata: bool,
}

impl Default for QualityConfig {
    fn default() -> Self {
        Self {
            preferred_quality: Some("1080p".to_string()),
        }
    }
}

impl Default for OriginalTitleConfig {
    fn default() -> Self {
        Self {
            prefer_original_titles: true,        // Prioritize original titles
            include_english_subtitle: true,      // Include English subtitle for clarity
            fallback_to_english_on_error: true,  // Safe fallback
            preserve_original_in_metadata: true, // Always preserve original
        }
    }
}

/// Confidence and matching preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchingConfig {
    /// Minimum confidence score required to organize a movie (0.0-1.0)
    pub min_confidence_threshold: f32, // Default: 0.5 (higher threshold to avoid wrong matches)
    /// Skip movies with no TMDB match instead of using fallback data
    pub skip_unmatched_movies: bool,
    /// Show warnings for low confidence matches
    pub warn_on_low_confidence: bool,
    /// Allow organizing movies with "Unknown Year" when no year is found
    pub allow_unknown_year: bool,
}

impl Default for MatchingConfig {
    fn default() -> Self {
        Self {
            min_confidence_threshold: 0.7, // High threshold to avoid incorrect matches - requires strong evidence
            skip_unmatched_movies: true,   // Default: skip files with no TMDB match
            warn_on_low_confidence: true,  // Warn about low confidence matches
            allow_unknown_year: true,      // Allow "Unknown Year" directories
        }
    }
}

impl AppConfig {
    /// Load configuration from file and environment variables
    pub fn load() -> Result<Self> {
        let config_dir = Self::get_config_dir()?;
        let config_file = config_dir.join("config.toml");

        let mut config_builder = config::ConfigBuilder::<config::builder::DefaultState>::default();

        // Set default configuration
        config_builder = config_builder.set_default("apis.tmdb_api_key", "")?;

        // Load configuration file if it exists
        if config_file.exists() {
            config_builder = config_builder.add_source(File::from(config_file.as_path()));
        }

        // Load environment variables
        config_builder =
            config_builder.add_source(Environment::with_prefix("PLEX_MEDIA_ORGANIZER"));

        // Build and deserialize configuration
        let config = config_builder.build()?;
        let app_config: AppConfig = config
            .try_deserialize()
            .context("Failed to deserialize configuration")?;

        Ok(app_config)
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_dir = Self::get_config_dir()?;
        let config_file = config_dir.join("config.toml");

        // Create config directory if it doesn't exist
        fs::create_dir_all(&config_dir).context("Failed to create configuration directory")?;

        // Convert to TOML and save
        let toml_string =
            toml::to_string_pretty(self).context("Failed to serialize configuration to TOML")?;

        fs::write(&config_file, toml_string).context("Failed to write configuration file")?;

        Ok(())
    }

    /// Get the configuration directory for the current platform
    pub fn get_config_dir() -> Result<PathBuf> {
        let config_dir = if cfg!(target_os = "macos") {
            dirs::home_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
                .join("Library/Application Support/plex-media-organizer")
        } else if cfg!(target_os = "linux") {
            dirs::config_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?
                .join("plex-media-organizer")
        } else if cfg!(target_os = "windows") {
            dirs::config_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?
                .join("plex-media-organizer")
        } else {
            dirs::home_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
                .join(".plex-media-organizer")
        };

        Ok(config_dir)
    }

    /// Check if required API keys are configured
    pub fn validate_api_keys(&self) -> Result<()> {
        let mut missing_keys = Vec::new();

        if self.apis.tmdb_api_key.is_none() {
            missing_keys.push("TMDB API key");
        }

        if !missing_keys.is_empty() {
            anyhow::bail!("Missing required API keys: {}", missing_keys.join(", "));
        }

        Ok(())
    }

    /// Create a new configuration with interactive setup
    pub fn interactive_setup() -> Result<Self> {
        println!("Welcome to Plex Media Organizer Setup!");
        println!("This will help you configure the application.\n");

        let mut config = AppConfig::default();

        // Get TMDB API key
        println!("To use this application, you need a TMDB API key.");
        println!("Get one for free at: https://www.themoviedb.org/settings/api");
        println!();

        let tmdb_key = Self::prompt_input("Enter your TMDB API key: ")?;
        if !tmdb_key.trim().is_empty() {
            config.apis.tmdb_api_key = Some(tmdb_key.trim().to_string());
        }

        // Save configuration
        config.save()?;
        println!("\nConfiguration saved successfully!");

        Ok(config)
    }

    /// Prompt for user input
    fn prompt_input(prompt: &str) -> Result<String> {
        use std::io::{self, Write};

        print!("{}", prompt);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = AppConfig::default();
        assert!(config.apis.tmdb_api_key.is_none());
        assert_eq!(
            config.organization.quality.preferred_quality,
            Some("1080p".to_string())
        );
    }

    #[test]
    fn test_config_serialization() {
        let config = AppConfig::default();
        let toml_string = toml::to_string(&config).unwrap();
        let deserialized: AppConfig = toml::from_str(&toml_string).unwrap();

        assert_eq!(
            config.organization.quality.preferred_quality,
            deserialized.organization.quality.preferred_quality
        );
    }
}
