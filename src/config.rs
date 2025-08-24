//! Configuration management for the Plex Media Organizer

use anyhow::{Context, Result};
use config::{Environment, File};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    /// Configuration version for migration tracking
    #[serde(default = "AppConfig::get_current_version")]
    pub version: String,
    /// API configuration for external services
    pub apis: ApiConfig,
    /// Database configuration
    pub database: DatabaseConfig,
    /// Organization preferences
    pub organization: OrganizationConfig,
}

/// API configuration for external services
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiConfig {
    /// TMDB API key for movie data
    pub tmdb_api_key: Option<String>,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database file path
    pub path: String,
    /// Maximum number of database connections in the pool
    pub max_connections: usize,
    /// Cache TTL in hours
    pub cache_ttl_hours: i64,
    /// Enable WAL mode for better concurrency
    pub enable_wal: bool,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            path: Self::get_default_database_path(),
            max_connections: 10,
            cache_ttl_hours: 24,
            enable_wal: true,
        }
    }
}

impl DatabaseConfig {
    /// Get the platform-appropriate default database path
    pub fn get_default_database_path() -> String {
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

        config_dir.join("movies.db").to_string_lossy().to_string()
    }
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
    /// Title preservation configuration
    pub title_preservation: TitlePreservationConfig,
    /// Language detection and processing configuration
    pub language: LanguageConfig,
    /// Technical terms filtering configuration
    pub technical_terms: TechnicalTermsConfig,
    /// Content filtering configuration for problematic patterns
    pub content_filtering: ContentFilteringConfig,
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
    /// Include English title in brackets: 英雄 \[Hero\] (2002)
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

/// Title preservation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitlePreservationConfig {
    /// Known movie titles to preserve from filtering
    pub known_titles: Vec<String>,
    /// Common English words to preserve from filtering
    pub common_words: Vec<String>,
}

impl Default for TitlePreservationConfig {
    fn default() -> Self {
        Self {
            known_titles: vec![
                "灌篮高手".to_string(),
                "灌篮".to_string(),
                "Slam".to_string(),
                "Dunk".to_string(),
            ],
            common_words: vec![
                "The".to_string(),
                "A".to_string(),
                "An".to_string(),
                "Of".to_string(),
                "In".to_string(),
                "On".to_string(),
                "At".to_string(),
                "To".to_string(),
                "For".to_string(),
                "With".to_string(),
                "From".to_string(),
                "By".to_string(),
            ],
        }
    }
}

/// Language detection and processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    /// Language codes for detection in filenames
    pub language_codes: Vec<String>,
    /// Japanese technical terms to filter out (language/audio descriptions)
    pub technical_japanese_terms: Vec<String>,
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            language_codes: vec![
                "JPN".to_string(),
                "ENG".to_string(),
                "CHI".to_string(),
                "KOR".to_string(),
                "JAP".to_string(),
                "EN".to_string(),
                "CN".to_string(),
            ],
            technical_japanese_terms: vec![
                "国日双语".to_string(),
                "双语".to_string(),
                "国日".to_string(),
                "日英".to_string(),
                "英日".to_string(),
                "中日".to_string(),
                "日中".to_string(),
            ],
        }
    }
}

/// Technical terms filtering configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalTermsConfig {
    /// Release group names to filter out from movie titles
    pub release_groups: Vec<String>,
    /// Video/audio codec and quality terms to filter out
    pub video_audio_terms: Vec<String>,
    /// Source/platform names to filter out
    pub source_platform_terms: Vec<String>,
    /// File format and container terms to filter out
    pub file_format_terms: Vec<String>,
    /// Special edition and version terms to filter out
    pub special_edition_terms: Vec<String>,
    /// Additional custom terms to filter out
    pub custom_terms: Vec<String>,
}

impl Default for TechnicalTermsConfig {
    fn default() -> Self {
        Self {
            release_groups: vec![
                "YIFY".to_string(),
                "YTS".to_string(),
                "RARBG".to_string(),
                "3L".to_string(),
            ],
            video_audio_terms: vec![
                "x264".to_string(),
                "x265".to_string(),
                "H264".to_string(),
                "H265".to_string(),
                "AVC".to_string(),
                "HEVC".to_string(),
                "10bit".to_string(),
                "8bit".to_string(),
                "TrueHD".to_string(),
                "7.1".to_string(),
                "5.1".to_string(),
                "2.0".to_string(),
                "DoVi".to_string(),
            ],
            source_platform_terms: vec![
                "BluRay".to_string(),
                "WEB-DL".to_string(),
                "HDTV".to_string(),
                "DVDRip".to_string(),
                "BRRip".to_string(),
                "HDRip".to_string(),
                "WEBRip".to_string(),
                "REMUX".to_string(),
            ],
            file_format_terms: vec!["mkv".to_string(), "mp4".to_string(), "avi".to_string()],
            special_edition_terms: vec![
                "EXTENDED".to_string(),
                "修复加长版".to_string(),
                "导演剪辑版".to_string(),
            ],
            custom_terms: vec![],
        }
    }
}

/// Content filtering configuration for problematic patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentFilteringConfig {
    /// Patterns that indicate problematic content (extras, documentaries, etc.)
    pub problematic_patterns: Vec<String>,
    /// File extensions that indicate extras content
    pub extras_extensions: Vec<String>,
    /// Specific patterns that indicate extras content
    pub extras_patterns: Vec<String>,
}

impl Default for ContentFilteringConfig {
    fn default() -> Self {
        Self {
            problematic_patterns: vec![
                "production report".to_string(),
                "making of".to_string(),
                "behind the scenes".to_string(),
                "documentary".to_string(),
                "special feature".to_string(),
                "bonus content".to_string(),
                "extras".to_string(),
                "commentary".to_string(),
                "interview".to_string(),
                "photo gallery".to_string(),
                "gallery".to_string(),
                "trailer".to_string(),
                "teaser".to_string(),
                "preview".to_string(),
                "sneak peek".to_string(),
                "deleted scene".to_string(),
                "alternate ending".to_string(),
                "bloopers".to_string(),
                "outtakes".to_string(),
                "featurette".to_string(),
                "promo".to_string(),
                "promotional".to_string(),
                "music video".to_string(),
                "soundtrack".to_string(),
                "score".to_string(),
                "ost".to_string(),
            ],
            extras_extensions: vec!["ifo".to_string(), "bup".to_string(), "vob".to_string()],
            extras_patterns: vec![
                "sample".to_string(),
                "trailer".to_string(),
                "pv".to_string(),
                "menu".to_string(),
                "bdmv".to_string(),
                "interview".to_string(),
                "commentary".to_string(),
                "featurette".to_string(),
                "deleted.scene".to_string(),
                "bloopers".to_string(),
                "outtakes".to_string(),
            ],
        }
    }
}

impl AppConfig {
    /// Get the current configuration version
    pub fn get_current_version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// Load configuration from file and environment variables with migration support
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

        // Override database path with platform-specific default if it's still the old default
        if app_config.database.path == "data/movies.db" {
            app_config.database.path = DatabaseConfig::get_default_database_path();
        }

        // Override database path with environment variable if set (highest priority)
        if let Ok(db_path) = std::env::var("PLEX_MEDIA_ORGANIZER_DATABASE_PATH") {
            app_config.database.path = db_path;
        }

        // Handle configuration migration if user config exists
        if user_config_exists {
            app_config = Self::migrate_configuration(app_config, &config_file)?;
        }

        Ok(app_config)
    }

    /// Migrate configuration to current version while preserving user customizations
    fn migrate_configuration(
        mut app_config: AppConfig,
        config_file: &PathBuf,
    ) -> Result<AppConfig> {
        let current_version = Self::get_current_version();

        // If versions match, no migration needed
        if app_config.version == current_version {
            return Ok(app_config);
        }

        println!("🔄 Configuration migration detected:");
        println!("   From version: {}", app_config.version);
        println!("   To version: {}", current_version);

        // Create backup of current configuration
        let backup_path = config_file.with_extension("toml.backup");
        if config_file.exists() {
            fs::copy(config_file, &backup_path).context("Failed to create configuration backup")?;
            println!("   📋 Backup created: {}", backup_path.display());
        }

        // Load current defaults for comparison
        let default_config = Self::load_defaults_only()?;

        // Merge new defaults with user customizations
        app_config = Self::merge_with_defaults(app_config, default_config);

        // Update version
        app_config.version = current_version;

        // Save migrated configuration
        app_config.save()?;
        println!("   ✅ Configuration migrated successfully");

        Ok(app_config)
    }

    /// Load only the default configuration (no user overrides)
    pub fn load_defaults_only() -> Result<Self> {
        let mut config_builder = config::ConfigBuilder::<config::builder::DefaultState>::default();

        // Load default configuration
        let default_config = include_str!("../config/default.toml");
        config_builder =
            config_builder.add_source(File::from_str(default_config, config::FileFormat::Toml));

        // Build and deserialize
        let config = config_builder.build()?;
        let app_config: AppConfig = config
            .try_deserialize()
            .context("Failed to deserialize default configuration")?;

        Ok(app_config)
    }

    /// Merge user configuration with new defaults, preserving user customizations
    pub fn merge_with_defaults(user_config: AppConfig, default_config: AppConfig) -> AppConfig {
        let mut merged = user_config.clone();

        // Merge technical terms - add new defaults while preserving user customizations
        merged.organization.technical_terms = Self::merge_technical_terms(
            &user_config.organization.technical_terms,
            &default_config.organization.technical_terms,
        );

        // Merge title preservation - add new defaults while preserving user customizations
        merged.organization.title_preservation = Self::merge_title_preservation(
            &user_config.organization.title_preservation,
            &default_config.organization.title_preservation,
        );

        // Merge language configuration
        merged.organization.language = Self::merge_language_config(
            &user_config.organization.language,
            &default_config.organization.language,
        );

        // Merge content filtering
        merged.organization.content_filtering = Self::merge_content_filtering(
            &user_config.organization.content_filtering,
            &default_config.organization.content_filtering,
        );

        merged
    }

    /// Merge technical terms, preserving user customizations and adding new defaults
    fn merge_technical_terms(
        user_terms: &TechnicalTermsConfig,
        default_terms: &TechnicalTermsConfig,
    ) -> TechnicalTermsConfig {
        TechnicalTermsConfig {
            release_groups: Self::merge_string_lists(
                &user_terms.release_groups,
                &default_terms.release_groups,
            ),
            video_audio_terms: Self::merge_string_lists(
                &user_terms.video_audio_terms,
                &default_terms.video_audio_terms,
            ),
            source_platform_terms: Self::merge_string_lists(
                &user_terms.source_platform_terms,
                &default_terms.source_platform_terms,
            ),
            file_format_terms: Self::merge_string_lists(
                &user_terms.file_format_terms,
                &default_terms.file_format_terms,
            ),
            special_edition_terms: Self::merge_string_lists(
                &user_terms.special_edition_terms,
                &default_terms.special_edition_terms,
            ),
            custom_terms: Self::merge_string_lists(
                &user_terms.custom_terms,
                &default_terms.custom_terms,
            ),
        }
    }

    /// Merge title preservation configuration
    fn merge_title_preservation(
        user_preservation: &TitlePreservationConfig,
        default_preservation: &TitlePreservationConfig,
    ) -> TitlePreservationConfig {
        TitlePreservationConfig {
            known_titles: Self::merge_string_lists(
                &user_preservation.known_titles,
                &default_preservation.known_titles,
            ),
            common_words: Self::merge_string_lists(
                &user_preservation.common_words,
                &default_preservation.common_words,
            ),
        }
    }

    /// Merge language configuration
    fn merge_language_config(
        user_language: &LanguageConfig,
        default_language: &LanguageConfig,
    ) -> LanguageConfig {
        LanguageConfig {
            language_codes: Self::merge_string_lists(
                &user_language.language_codes,
                &default_language.language_codes,
            ),
            technical_japanese_terms: Self::merge_string_lists(
                &user_language.technical_japanese_terms,
                &default_language.technical_japanese_terms,
            ),
        }
    }

    /// Merge content filtering configuration
    fn merge_content_filtering(
        user_filtering: &ContentFilteringConfig,
        default_filtering: &ContentFilteringConfig,
    ) -> ContentFilteringConfig {
        ContentFilteringConfig {
            problematic_patterns: Self::merge_string_lists(
                &user_filtering.problematic_patterns,
                &default_filtering.problematic_patterns,
            ),
            extras_extensions: Self::merge_string_lists(
                &user_filtering.extras_extensions,
                &default_filtering.extras_extensions,
            ),
            extras_patterns: Self::merge_string_lists(
                &user_filtering.extras_patterns,
                &default_filtering.extras_patterns,
            ),
        }
    }

    /// Merge two string lists, preserving user items and adding new defaults
    fn merge_string_lists(user_list: &[String], default_list: &[String]) -> Vec<String> {
        let mut merged = user_list.to_vec();

        // Add new defaults that aren't already in user list
        for default_item in default_list {
            if !merged.contains(default_item) {
                merged.push(default_item.clone());
            }
        }

        merged
    }

    /// Check if configuration needs migration
    pub fn needs_migration(&self) -> bool {
        self.version != Self::get_current_version()
    }

    /// Get migration information
    pub fn get_migration_info(&self) -> (String, String) {
        (self.version.clone(), Self::get_current_version())
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

    /// Get all technical terms as a single list for filtering
    pub fn get_all_technical_terms(&self) -> Vec<String> {
        let mut all_terms = Vec::new();

        all_terms.extend(self.organization.technical_terms.release_groups.clone());
        all_terms.extend(self.organization.technical_terms.video_audio_terms.clone());
        all_terms.extend(
            self.organization
                .technical_terms
                .source_platform_terms
                .clone(),
        );
        all_terms.extend(self.organization.technical_terms.file_format_terms.clone());
        all_terms.extend(
            self.organization
                .technical_terms
                .special_edition_terms
                .clone(),
        );
        all_terms.extend(self.organization.technical_terms.custom_terms.clone());

        all_terms
    }

    /// Get all content filtering patterns
    pub fn get_all_content_filtering_patterns(&self) -> Vec<String> {
        let mut patterns = Vec::new();
        patterns.extend(
            self.organization
                .content_filtering
                .problematic_patterns
                .clone(),
        );
        patterns.extend(self.organization.content_filtering.extras_patterns.clone());
        patterns
    }

    /// Get extras extensions
    pub fn get_extras_extensions(&self) -> Vec<String> {
        self.organization
            .content_filtering
            .extras_extensions
            .clone()
    }

    /// Get language codes
    pub fn get_language_codes(&self) -> Vec<String> {
        self.organization.language.language_codes.clone()
    }

    /// Get common words
    pub fn get_common_words(&self) -> Vec<String> {
        self.organization.title_preservation.common_words.clone()
    }

    /// Get known titles
    pub fn get_known_titles(&self) -> Vec<String> {
        self.organization.title_preservation.known_titles.clone()
    }

    /// Get technical Japanese terms
    pub fn get_technical_japanese_terms(&self) -> Vec<String> {
        self.organization.language.technical_japanese_terms.clone()
    }

    /// Get release groups
    pub fn get_release_groups(&self) -> Vec<String> {
        self.organization.technical_terms.release_groups.clone()
    }
}
