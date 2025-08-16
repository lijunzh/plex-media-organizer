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
    /// Learning and pattern recognition settings
    pub learning: LearningConfig,
    /// Database configuration
    pub database: DatabaseConfig,
}

/// API configuration for external services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// TMDB API key for movie data
    pub tmdb_api_key: Option<String>,
    /// TVDB API key for TV show data
    pub tvdb_api_key: Option<String>,
    /// MusicBrainz user agent
    pub musicbrainz_user_agent: Option<String>,
    /// AniDB credentials
    pub anidb_username: Option<String>,
    pub anidb_password: Option<String>,
    /// Rate limiting settings
    pub rate_limits: RateLimitConfig,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// TMDB requests per day
    pub tmdb_requests_per_day: u32,
    /// TVDB requests per day
    pub tvdb_requests_per_day: u32,
    /// MusicBrainz requests per second
    pub musicbrainz_requests_per_second: f32,
    /// AniDB requests per second
    pub anidb_requests_per_second: f32,
}

/// Organization preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationConfig {
    /// Movie naming template
    pub movies_template: String,
    /// TV show naming template
    pub tv_shows_template: String,
    /// Music naming template
    pub music_template: String,
    /// Quality preferences
    pub quality: QualityConfig,
    /// Whether to create organized directory structures
    pub create_directories: bool,
    /// Whether to rename files
    pub rename_files: bool,
    /// Whether to move files to organized locations
    pub move_files: bool,
}

/// Quality preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityConfig {
    /// Preferred quality (720p, 1080p, 4K)
    pub preferred_quality: Option<String>,
    /// Minimum acceptable quality
    pub minimum_quality: Option<String>,
    /// Whether to prefer higher quality when duplicates exist
    pub prefer_higher_quality: bool,
}

/// Learning and pattern recognition settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningConfig {
    /// Whether to enable pattern learning
    pub enable_pattern_learning: bool,
    /// Confidence threshold for accepting patterns
    pub confidence_threshold: f32,
    /// Maximum patterns to store per media type
    pub max_patterns_per_type: u32,
    /// Pattern expiration in days
    pub pattern_expiration_days: u32,
    /// Weight given to user feedback
    pub user_feedback_weight: f32,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database file path
    pub database_path: Option<PathBuf>,
    /// Whether to enable database logging
    pub enable_logging: bool,
    /// Database connection pool size
    pub connection_pool_size: u32,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            tmdb_api_key: None,
            tvdb_api_key: None,
            musicbrainz_user_agent: Some("PlexMediaOrganizer/1.0".to_string()),
            anidb_username: None,
            anidb_password: None,
            rate_limits: RateLimitConfig::default(),
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            tmdb_requests_per_day: 1000,
            tvdb_requests_per_day: 1000,
            musicbrainz_requests_per_second: 1.0,
            anidb_requests_per_second: 0.5,
        }
    }
}

impl Default for OrganizationConfig {
    fn default() -> Self {
        Self {
            movies_template: "{title} ({year}) {quality}".to_string(),
            tv_shows_template:
                "{title}/Season {season:02}/{title} S{season:02}E{episode:02} {episode_title}"
                    .to_string(),
            music_template: "{artist}/{album}/{track:02} - {title}".to_string(),
            quality: QualityConfig::default(),
            create_directories: true,
            rename_files: true,
            move_files: false,
        }
    }
}

impl Default for QualityConfig {
    fn default() -> Self {
        Self {
            preferred_quality: Some("1080p".to_string()),
            minimum_quality: Some("720p".to_string()),
            prefer_higher_quality: true,
        }
    }
}

impl Default for LearningConfig {
    fn default() -> Self {
        Self {
            enable_pattern_learning: true,
            confidence_threshold: 0.7,
            max_patterns_per_type: 1000,
            pattern_expiration_days: 365,
            user_feedback_weight: 0.8,
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_path: None,
            enable_logging: true,
            connection_pool_size: 5,
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
        config_builder = config_builder
            .set_default("apis.tmdb_api_key", "")?
            .set_default("apis.tvdb_api_key", "")?
            .set_default("apis.musicbrainz_user_agent", "PlexMediaOrganizer/1.0")?
            .set_default("apis.anidb_username", "")?
            .set_default("apis.anidb_password", "")?;

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

    /// Get the database file path
    pub fn get_database_path(&self) -> Result<PathBuf> {
        let config_dir = Self::get_config_dir()?;
        let db_path = self
            .database
            .database_path
            .clone()
            .unwrap_or_else(|| config_dir.join("plex_media_organizer.db"));

        Ok(db_path)
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
        assert_eq!(config.learning.confidence_threshold, 0.7);
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
            config.learning.confidence_threshold,
            deserialized.learning.confidence_threshold
        );
    }
}
