//! Application configuration loaded from TOML.

use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Top-level configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    /// Directories to scan for media files.
    pub source_dirs: Vec<String>,
    /// Root destination directory for organized files.
    pub destination: String,
    /// Confidence threshold above which files are auto-organized.
    pub auto_organize_threshold: f64,
    /// Below this threshold, files are flagged for manual review.
    pub review_threshold: f64,
    pub organize: OrganizeSettings,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            source_dirs: Vec::new(),
            destination: String::new(),
            auto_organize_threshold: 90.0,
            review_threshold: 50.0,
            organize: OrganizeSettings::default(),
        }
    }
}

/// Settings for file organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct OrganizeSettings {
    /// File operation strategy: "move", "copy", or "symlink".
    pub strategy: String,
    /// Subdirectory name for movies.
    pub movies_dir: String,
    /// Subdirectory name for TV shows.
    pub tv_dir: String,
    /// Subdirectory name for music.
    pub music_dir: String,
}

impl Default for OrganizeSettings {
    fn default() -> Self {
        Self {
            strategy: "move".to_string(),
            movies_dir: "Movies".to_string(),
            tv_dir: "TV Shows".to_string(),
            music_dir: "Music".to_string(),
        }
    }
}

impl AppConfig {
    /// Load config from a TOML file, falling back to defaults for missing fields.
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        let config: Self =
            toml::from_str(&content).with_context(|| "Failed to parse TOML config")?;
        Ok(config)
    }

    /// Load from a file if it exists, otherwise return defaults.
    pub fn load_or_default(path: Option<&Path>) -> Self {
        match path {
            Some(p) if p.exists() => Self::load(p).unwrap_or_default(),
            _ => Self::default(),
        }
    }
}
