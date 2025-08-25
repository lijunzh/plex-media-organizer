//! Configuration management for the Plex Media Organizer

use anyhow::{Context, Result};
use config::{Environment, File};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    /// Configuration version
    #[serde(default = "AppConfig::get_current_version")]
    pub version: String,
    /// API configuration for external services
    pub apis: ApiConfig,
    /// Parsing configuration
    pub parsing: ParsingConfig,
    /// Organization configuration
    pub organization: OrganizationConfig,
}

/// API configuration for external services
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiConfig {
    /// TMDB API key for movie data
    pub tmdb_api_key: Option<String>,
}

/// Parsing configuration for the TMDB-first approach
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParsingConfig {
    /// Technical terms to filter out from titles
    pub technical_terms: TechnicalTermsConfig,
    /// TMDB integration settings
    pub tmdb: TmdbConfig,
    /// Output formatting preferences
    pub output: OutputConfig,
}

/// Technical terms configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TechnicalTermsConfig {
    /// Release groups to filter out
    pub release_groups: Vec<String>,
    /// Video/audio codecs and formats to filter out
    pub codecs: Vec<String>,
    /// Quality indicators to filter out
    pub quality: Vec<String>,
    /// Source indicators to filter out
    pub sources: Vec<String>,
    /// File extensions to filter out
    pub extensions: Vec<String>,
}

/// TMDB configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmdbConfig {
    /// Minimum confidence threshold for TMDB enhancement
    pub min_confidence: f32,
    /// Whether to prioritize English titles for TMDB search
    pub prioritize_english: bool,
    /// Whether to use TMDB for language detection
    pub use_language_detection: bool,
}

impl Default for TmdbConfig {
    fn default() -> Self {
        Self {
            min_confidence: 0.3,
            prioritize_english: true,
            use_language_detection: true,
        }
    }
}

/// Output formatting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Whether to include quality in the final title
    pub include_quality: bool,
    /// Whether to include year in the final title
    pub include_year: bool,
    /// Format for non-English movies: "original" or "bilingual"
    pub non_english_format: String,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            include_quality: false,
            include_year: true,
            non_english_format: "bilingual".to_string(),
        }
    }
}

/// Organization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationConfig {
    /// Whether to create Plex-style directory structure
    pub create_plex_structure: bool,
    /// Whether to move files or copy them
    pub move_files: bool,
    /// Whether to preserve original filenames
    pub preserve_original_names: bool,
}

impl Default for OrganizationConfig {
    fn default() -> Self {
        Self {
            create_plex_structure: true,
            move_files: true,
            preserve_original_names: false,
        }
    }
}

impl AppConfig {
    /// Get the current configuration version
    pub fn get_current_version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// Load configuration from file and environment
    pub fn load() -> Result<Self> {
        let config_dir = Self::get_config_dir()?;
        let config_file = config_dir.join("config.toml");

        let mut config_builder = config::ConfigBuilder::<config::builder::DefaultState>::default();

        // Load default configuration first
        let default_config = include_str!("../config/default.toml");
        config_builder =
            config_builder.add_source(File::from_str(default_config, config::FileFormat::Toml));

        // Load user configuration file if it exists (overrides defaults)
        let user_config_exists = config_file.exists();
        if user_config_exists {
            config_builder = config_builder.add_source(File::from(config_file.as_path()));
        }

        // Load environment variables (highest priority)
        config_builder =
            config_builder.add_source(Environment::with_prefix("PLEX_MEDIA_ORGANIZER"));

        // Build and deserialize configuration
        let config = config_builder.build()?;
        let mut app_config: AppConfig = config
            .try_deserialize()
            .context("Failed to deserialize configuration")?;

        // Override TMDB API key with environment variable if set
        if let Ok(tmdb_key) = std::env::var("PLEX_MEDIA_ORGANIZER_TMDB_API_KEY") {
            app_config.apis.tmdb_api_key = Some(tmdb_key);
        }

        Ok(app_config)
    }

    /// Get the configuration directory
    pub fn get_config_dir() -> Result<PathBuf> {
        let config_dir = if cfg!(target_os = "macos") {
            dirs::home_dir()
                .map(|home| home.join("Library/Application Support/plex-media-organizer"))
                .unwrap_or_else(|| PathBuf::from("data"))
        } else if cfg!(target_os = "linux") {
            dirs::data_local_dir()
                .map(|data_dir| data_dir.join("plex-media-organizer"))
                .unwrap_or_else(|| {
                    dirs::home_dir()
                        .unwrap_or_else(|| PathBuf::from("."))
                        .join(".plex-media-organizer")
                })
        } else if cfg!(target_os = "windows") {
            dirs::config_dir()
                .map(|config_dir| config_dir.join("plex-media-organizer"))
                .unwrap_or_else(|| PathBuf::from("data"))
        } else {
            dirs::home_dir()
                .map(|home| home.join(".plex-media-organizer"))
                .unwrap_or_else(|| PathBuf::from("data"))
        };

        // Create the directory if it doesn't exist
        fs::create_dir_all(&config_dir).context("Failed to create configuration directory")?;

        Ok(config_dir)
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_dir = Self::get_config_dir()?;
        let config_file = config_dir.join("config.toml");

        let toml_string =
            toml::to_string_pretty(self).context("Failed to serialize configuration to TOML")?;

        fs::write(&config_file, toml_string).context("Failed to write configuration file")?;

        Ok(())
    }

    /// Get all technical terms as a single vector for easy filtering
    pub fn get_all_technical_terms(&self) -> Vec<String> {
        let mut terms = Vec::new();
        terms.extend(self.parsing.technical_terms.release_groups.clone());
        terms.extend(self.parsing.technical_terms.codecs.clone());
        terms.extend(self.parsing.technical_terms.quality.clone());
        terms.extend(self.parsing.technical_terms.sources.clone());
        terms.extend(self.parsing.technical_terms.extensions.clone());
        terms
    }
}
