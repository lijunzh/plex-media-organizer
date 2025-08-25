//! Efficient metadata extraction from media files and external metadata files

use crate::media::MovieInfo;
use anyhow::{Context, Result};
use serde_json;
use std::fs;
use std::path::Path;

/// Efficient metadata extractor that reads headers and external files
#[derive(Debug)]
pub struct MetadataExtractor;

impl MetadataExtractor {
    /// Extract metadata using the priority order: external files -> media headers -> filename
    pub fn extract_metadata(file_path: &Path) -> Result<MovieInfo> {
        let mut metadata = MovieInfo::default();

        // 1. Try external metadata files first (highest priority)
        if let Some(external_meta) = Self::extract_from_external_files(file_path)? {
            metadata = Self::merge_movie_info(metadata, external_meta);
        }

        // 2. Extract from media file headers (if no external file or incomplete)
        if metadata.title.is_empty() || metadata.year.is_none() {
            if let Some(header_meta) = Self::extract_from_media_headers(file_path)? {
                metadata = Self::merge_movie_info(metadata, header_meta);
            }
        }

        // 3. Fallback to filename parsing (lowest priority)
        if metadata.title.is_empty() {
            if let Some(filename_meta) = Self::extract_from_filename(file_path)? {
                metadata = Self::merge_movie_info(metadata, filename_meta);
            }
        }

        Ok(metadata)
    }

    /// Extract metadata from reliable external files (.nfo, .txt, .info, .json)
    fn extract_from_external_files(file_path: &Path) -> Result<Option<MovieInfo>> {
        let base_path = file_path.with_extension("");

        // Only check reliable metadata file extensions
        let reliable_extensions = ["nfo", "txt", "info", "json"];

        for ext in &reliable_extensions {
            let metadata_path = base_path.with_extension(ext);
            if metadata_path.exists() {
                if let Some(meta) = Self::parse_metadata_file(&metadata_path)? {
                    return Ok(Some(meta));
                }
            }
        }

        Ok(None)
    }

    /// Parse different types of metadata files
    fn parse_metadata_file(file_path: &Path) -> Result<Option<MovieInfo>> {
        let extension = file_path.extension().and_then(|s| s.to_str()).unwrap_or("");

        match extension {
            "nfo" => Self::parse_nfo_file(file_path),
            "json" => Self::parse_json_file(file_path),
            "txt" | "info" => Self::parse_text_file(file_path),
            _ => Ok(None),
        }
    }

    /// Parse NFO (XML) files used by Plex/Kodi
    fn parse_nfo_file(file_path: &Path) -> Result<Option<MovieInfo>> {
        let content = fs::read_to_string(file_path).context("Failed to read NFO file")?;

        // Simple XML parsing for common NFO fields
        let mut movie_info = MovieInfo::default();

        // Extract title
        if let Some(title) = Self::extract_xml_tag(&content, "title") {
            movie_info.title = title;
        }

        // Extract year
        if let Some(year_str) = Self::extract_xml_tag(&content, "year") {
            if let Ok(year) = year_str.parse::<u32>() {
                movie_info.year = Some(year);
            }
        }

        // Extract original title
        if let Some(original_title) = Self::extract_xml_tag(&content, "originaltitle") {
            movie_info.original_title = Some(original_title);
        }

        // Only return if we found meaningful data
        if !movie_info.title.is_empty() {
            Ok(Some(movie_info))
        } else {
            Ok(None)
        }
    }

    /// Parse JSON metadata files
    fn parse_json_file(file_path: &Path) -> Result<Option<MovieInfo>> {
        let content = fs::read_to_string(file_path).context("Failed to read JSON file")?;
        let json: serde_json::Value = serde_json::from_str(&content).context("Invalid JSON")?;

        let mut movie_info = MovieInfo::default();

        // Extract title
        if let Some(title) = json.get("title").and_then(|v| v.as_str()) {
            movie_info.title = title.to_string();
        }

        // Extract year
        if let Some(year) = json.get("year").and_then(|v| v.as_u64()) {
            movie_info.year = Some(year as u32);
        }

        // Extract original title
        if let Some(original_title) = json.get("original_title").and_then(|v| v.as_str()) {
            movie_info.original_title = Some(original_title.to_string());
        }

        // Only return if we found meaningful data
        if !movie_info.title.is_empty() {
            Ok(Some(movie_info))
        } else {
            Ok(None)
        }
    }

    /// Parse text-based metadata files
    fn parse_text_file(file_path: &Path) -> Result<Option<MovieInfo>> {
        let content = fs::read_to_string(file_path).context("Failed to read text file")?;
        let lines: Vec<&str> = content.lines().collect();

        let mut movie_info = MovieInfo::default();

        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Try to parse key-value pairs
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim().to_lowercase();
                let value = value.trim();

                match key.as_str() {
                    "title" | "name" => {
                        if movie_info.title.is_empty() {
                            movie_info.title = value.to_string();
                        }
                    }
                    "year" | "release_year" => {
                        if let Ok(year) = value.parse::<u32>() {
                            movie_info.year = Some(year);
                        }
                    }
                    "original_title" | "original" => {
                        movie_info.original_title = Some(value.to_string());
                    }
                    _ => {}
                }
            } else {
                // If no key-value format, assume it's a title if we don't have one
                if movie_info.title.is_empty() && !line.is_empty() {
                    movie_info.title = line.to_string();
                }
            }
        }

        // Only return if we found meaningful data
        if !movie_info.title.is_empty() {
            Ok(Some(movie_info))
        } else {
            Ok(None)
        }
    }

    /// Extract metadata from media file headers
    fn extract_from_media_headers(_file_path: &Path) -> Result<Option<MovieInfo>> {
        // This is a placeholder for future media header extraction
        // For now, we'll return None to fall back to filename parsing
        Ok(None)
    }

    /// Extract basic metadata from filename
    fn extract_from_filename(file_path: &Path) -> Result<Option<MovieInfo>> {
        let filename = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        if filename.is_empty() {
            return Ok(None);
        }

        let mut movie_info = MovieInfo::default();

        // Simple filename parsing - this is a basic implementation
        // More sophisticated parsing is handled by the dedicated parser modules
        let parts: Vec<&str> = filename.split(|c| c == '.' || c == '_' || c == '-').collect();

        if !parts.is_empty() {
            movie_info.title = parts[0].to_string();
        }

        // Try to find a year in the filename
        for part in &parts {
            if part.len() == 4 && part.chars().all(|c| c.is_ascii_digit()) {
                if let Ok(year) = part.parse::<u32>() {
                    if year >= 1900 && year <= 2030 {
                        movie_info.year = Some(year);
                        break;
                    }
                }
            }
        }

        // Only return if we found a title
        if !movie_info.title.is_empty() {
            Ok(Some(movie_info))
        } else {
            Ok(None)
        }
    }

    /// Extract XML tag content from a string
    fn extract_xml_tag(content: &str, tag_name: &str) -> Option<String> {
        let start_tag = format!("<{}>", tag_name);
        let end_tag = format!("</{}>", tag_name);

        if let Some(start) = content.find(&start_tag) {
            let start_pos = start + start_tag.len();
            if let Some(end) = content[start_pos..].find(&end_tag) {
                let tag_content = &content[start_pos..start_pos + end];
                return Some(tag_content.trim().to_string());
            }
        }

        None
    }

    /// Merge two MovieInfo structs, preferring non-empty values from the second
    fn merge_movie_info(mut base: MovieInfo, additional: MovieInfo) -> MovieInfo {
        if !additional.title.is_empty() {
            base.title = additional.title;
        }
        if additional.year.is_some() {
            base.year = additional.year;
        }
        if additional.original_title.is_some() {
            base.original_title = additional.original_title;
        }
        base
    }
}