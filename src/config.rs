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
    /// Technical terms filtering configuration
    pub technical_terms: TechnicalTermsConfig,
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
                // Common release groups
                "CMCT".to_string(),
                "WiKi".to_string(),
                "FRDS".to_string(),
                "HDS".to_string(),
                "ADWeb".to_string(),
                "TLF".to_string(),
                "CHDWEB".to_string(),
                "PTerWEB".to_string(),
                "GREENOTEA".to_string(),
                "ZmWeb".to_string(),
                "HDVWEB".to_string(),
                "NukeHD".to_string(),
                "TJUPT".to_string(),
                "CMCTV".to_string(),
                "NTG".to_string(),
                "HDWTV".to_string(),
                "NowOur".to_string(),
                "PandaQT".to_string(),
                "HANDJOB".to_string(),
                "npuer".to_string(),
                "BYRHD".to_string(),
                "c0kE".to_string(),
                "TBMovies".to_string(),
                "MNHD".to_string(),
                "YTS".to_string(),
                "MX".to_string(),
                "HDWinG".to_string(),
                "NYPAD".to_string(),
                "ZigZag".to_string(),
                "NTb".to_string(),
                "REMUX".to_string(),
                "iT".to_string(),
                "mUHD".to_string(),
                "IAMABLE".to_string(),
                "KRaLiMaRKo".to_string(),
                "HDChina".to_string(),
                "CtrlHD".to_string(),
                "SWTYBLZ".to_string(),
                "ADE".to_string(),
                "PHOBOS".to_string(),
                "PTHOME".to_string(),
                "SyncUP".to_string(),
                "YIFY".to_string(),
                "SPARKS".to_string(),
                "HiDt".to_string(),
                "Geek".to_string(),
                "TayTO".to_string(),
                "nikt0".to_string(),
                "beAst".to_string(),
                "FoRM".to_string(),
                "CRiME".to_string(),
                "HVAC".to_string(),
                "MaoZhan".to_string(),
                "VietHD".to_string(),
                "JYK".to_string(),
                "PiRaTeS".to_string(),
                "GalaxyRG265".to_string(),
                "PaODEQUEiJO".to_string(),
                "Silence".to_string(),
                "LoRD".to_string(),
                "SA89".to_string(),
                "FANDANGO".to_string(),
                "DON".to_string(),
                "D-Z0N3".to_string(),
                "PTer".to_string(),
                "ABM".to_string(),
                "MZABI".to_string(),
                "BYRPAD".to_string(),
                "NCmt".to_string(),
                "MTeam".to_string(),
                "playWEB".to_string(),
                "FLUX".to_string(),
                "CMRG".to_string(),
                "MZABARBiE".to_string(),
                "SMURF".to_string(),
                "AREY".to_string(),
                "RABiDS".to_string(),
                "ETHEL".to_string(),
                "RightSiZE".to_string(),
                "CiNEPHiLES".to_string(),
                "Kitsune".to_string(),
                "CHD".to_string(),
                "LolHD".to_string(),
                "DDP5".to_string(),
                "WiKi".to_string(),
                "SyncUP".to_string(),
                "HDChina".to_string(),
                "FRDS".to_string(),
                "BYRHD".to_string(),
                "playWEB".to_string(),
                "IAMABLE".to_string(),
                "EtHD".to_string(),
                "FANDANGO".to_string(),
                "LoRD".to_string(),
                "MNHD".to_string(),
                "PTer".to_string(),
                "DON".to_string(),
                "D-Z0N3".to_string(),
                "BYRPAD".to_string(),
                "iPad".to_string(),
            ],
            video_audio_terms: vec![
                // Video/audio codecs and quality
                "10bit".to_string(),
                "10bits".to_string(),
                "bit".to_string(),
                "bits".to_string(),
                "DDP".to_string(),
                "DTS".to_string(),
                "AC3".to_string(),
                "AAC".to_string(),
                "FLAC".to_string(),
                "THD".to_string(),
                "MA".to_string(),
                "HD".to_string(),
                "x264".to_string(),
                "x265".to_string(),
                "H264".to_string(),
                "H265".to_string(),
                "AVC".to_string(),
                "HEVC".to_string(),
                "Atmos".to_string(),
                "TrueHD".to_string(),
                "DualAudio".to_string(),
                "2Audio".to_string(),
                "2Audios".to_string(),
                "4Audios".to_string(),
                "60fps".to_string(),
                "HQ".to_string(),
                "AAC(5".to_string(),
                "1)".to_string(),
                "Hi10P".to_string(),
                "DD5".to_string(),
                "TrueHD7".to_string(),
                "H".to_string(),
                "264".to_string(),
                "265".to_string(),
                "4Audio".to_string(),
                "3Audio".to_string(),
                "5Audio".to_string(),
                "REPACK".to_string(),
                "Remux".to_string(),
                "VC-1".to_string(),
                "DoVi".to_string(),
                "HDR10".to_string(),
                "EDR".to_string(),
                "MULTi".to_string(),
                "HDTS".to_string(),
                "IMAX".to_string(),
                "DSNP".to_string(),
                "DTS-HD".to_string(),
                "HDR".to_string(),
                "120FPS".to_string(),
                "4K".to_string(),
                "WEB".to_string(),
                "WEBRip".to_string(),
                "UHD".to_string(),
                "Blu-ray".to_string(),
                "Bluray".to_string(),
                "BluRay".to_string(),
                "DD5".to_string(),
                "DD+".to_string(),
                "AC3".to_string(),
                "AAC5".to_string(),
                "AAC1".to_string(),
                "10bit".to_string(),
                "DV".to_string(),
                "MP4".to_string(),
                "MKV".to_string(),
            ],
            source_platform_terms: vec![
                // Source/platform names
                "NF".to_string(),
                "AMZN".to_string(),
                "HKG".to_string(),
                "ESP".to_string(),
                "GBR".to_string(),
                "INT".to_string(),
                "JPN".to_string(),
                "CHN".to_string(),
                "CCTV6HD".to_string(),
                "CHC".to_string(),
                "Star".to_string(),
                "Movie-HD".to_string(),
                "AKA".to_string(),
                "Chinese".to_string(),
                "iTunes".to_string(),
                "AMZN".to_string(),
                "NF".to_string(),
                "Netflix".to_string(),
                "HMAX".to_string(),
                "NOW".to_string(),
                "ATVP".to_string(),
                "HULU".to_string(),
                "DSNP".to_string(),
            ],
            file_format_terms: vec![
                // File formats and containers
                "HDTVRip".to_string(),
                "DVDRip".to_string(),
                "BDRip".to_string(),
                "HDRip".to_string(),
                "WEBRip".to_string(),
                "HDTV".to_string(),
                "MP3".to_string(),
            ],
            special_edition_terms: vec![
                // Special editions and versions
                "EXTENDED".to_string(),
                "修复加长版".to_string(),
                "导演剪辑版".to_string(),
                "Extended".to_string(),
                "RERIP".to_string(),
                "Hybrid".to_string(),
                "ES".to_string(),
            ],
            custom_terms: vec![
                // Additional custom terms
                "Blu".to_string(),
                "ray".to_string(),
                "VC".to_string(),
                "YTS".to_string(),
                "MX".to_string(),
                "AM".to_string(),
                "iNT".to_string(),
                "HHWEB".to_string(),
                "HDxT".to_string(),
                "BYNDR".to_string(),
            ],
        }
    }
}

impl AppConfig {
    /// Load configuration from file and environment variables
    pub fn load() -> Result<Self> {
        let config_dir = Self::get_config_dir()?;
        let config_file = config_dir.join("config.toml");

        let mut config_builder = config::ConfigBuilder::<config::builder::DefaultState>::default();

        // Load default configuration first
        let default_config = include_str!("../config/default.toml");
        config_builder =
            config_builder.add_source(File::from_str(default_config, config::FileFormat::Toml));

        // Load user configuration file if it exists (overrides defaults)
        if config_file.exists() {
            config_builder = config_builder.add_source(File::from(config_file.as_path()));
        }

        // Load environment variables (highest priority)
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
