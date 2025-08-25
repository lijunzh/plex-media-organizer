//! Efficient metadata extraction from media files and external metadata files

use crate::types::MovieInfo;
use anyhow::{Context, Result};
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};

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

        // Try to parse as JSON
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            let mut movie_info = MovieInfo::default();

            if let Some(title) = json["title"].as_str() {
                movie_info.title = title.to_string();
            }

            if let Some(year) = json["year"].as_u64() {
                movie_info.year = Some(year as u32);
            }

            if let Some(original_title) = json["original_title"].as_str() {
                movie_info.original_title = Some(original_title.to_string());
            }

            if !movie_info.title.is_empty() {
                return Ok(Some(movie_info));
            }
        }

        Ok(None)
    }

    /// Parse text metadata files
    fn parse_text_file(file_path: &Path) -> Result<Option<MovieInfo>> {
        let content = fs::read_to_string(file_path).context("Failed to read text file")?;

        let mut movie_info = MovieInfo::default();

        // Simple key-value parsing
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once(':') {
                match key.trim().to_lowercase().as_str() {
                    "title" => movie_info.title = value.trim().to_string(),
                    "year" => {
                        if let Ok(year) = value.trim().parse::<u32>() {
                            movie_info.year = Some(year);
                        }
                    }
                    "original_title" => movie_info.original_title = Some(value.trim().to_string()),
                    _ => {}
                }
            }
        }

        if !movie_info.title.is_empty() {
            Ok(Some(movie_info))
        } else {
            Ok(None)
        }
    }

    /// Extract metadata from media file headers (no full file reading)
    fn extract_from_media_headers(file_path: &Path) -> Result<Option<MovieInfo>> {
        let extension = file_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "mp4" | "m4v" | "mov" => Self::extract_mp4_metadata(file_path),
            "mkv" => Self::extract_mkv_metadata(file_path),
            "mp3" => Self::extract_mp3_metadata(file_path),
            _ => Ok(None), // Other formats not supported yet
        }
    }

    /// Extract metadata from MP4 files (read only headers)
    fn extract_mp4_metadata(_file_path: &Path) -> Result<Option<MovieInfo>> {
        // For now, return None - we'll implement this later
        // This would use the mp4parse crate to read moov atom
        Ok(None)
    }

    /// Extract metadata from MKV files (read only headers)
    fn extract_mkv_metadata(_file_path: &Path) -> Result<Option<MovieInfo>> {
        // For now, return None - we'll implement this later
        // This would use the matroska crate to read EBML metadata
        Ok(None)
    }

    /// Extract metadata from MP3 files (read only headers)
    fn extract_mp3_metadata(_file_path: &Path) -> Result<Option<MovieInfo>> {
        // For now, return None - we'll implement this later
        // This would use the id3 crate to read ID3 tags
        Ok(None)
    }

    /// Extract basic info from filename (fallback)
    fn extract_from_filename(file_path: &Path) -> Result<Option<MovieInfo>> {
        let filename = file_path.file_name().and_then(|s| s.to_str()).unwrap_or("");

        if filename.is_empty() {
            return Ok(None);
        }

        // Basic filename parsing - this is a simplified version
        // The full implementation would use the existing movie_parser logic
        let mut movie_info = MovieInfo::default();

        // Simple pattern: Movie Name (Year) Quality.ext
        if let Some(captures) = regex::Regex::new(r"^(.+?)\s*\((\d{4})\)")
            .unwrap()
            .captures(filename)
        {
            if let Some(title) = captures.get(1) {
                movie_info.title = title.as_str().trim().to_string();
            }
            if let Some(year_str) = captures.get(2) {
                if let Ok(year) = year_str.as_str().parse::<u32>() {
                    movie_info.year = Some(year);
                }
            }
        }

        if !movie_info.title.is_empty() {
            Ok(Some(movie_info))
        } else {
            Ok(None)
        }
    }

    /// Merge two MovieInfo structs, preferring the first one for conflicts
    fn merge_movie_info(primary: MovieInfo, secondary: MovieInfo) -> MovieInfo {
        MovieInfo {
            title: if !primary.title.is_empty() {
                primary.title
            } else {
                secondary.title
            },
            original_title: primary.original_title.or(secondary.original_title),
            original_language: primary.original_language.or(secondary.original_language),
            year: primary.year.or(secondary.year),
            part_number: primary.part_number.or(secondary.part_number),
            is_collection: primary.is_collection || secondary.is_collection,
            collection_name: primary.collection_name.or(secondary.collection_name),
            is_series: primary.is_series || secondary.is_series,
            series_name: primary.series_name.or(secondary.series_name),
            series_number: primary.series_number.or(secondary.series_number),
            is_anime: primary.is_anime || secondary.is_anime,
            anime_movie_number: primary.anime_movie_number.or(secondary.anime_movie_number),
            has_japanese_title: primary.has_japanese_title || secondary.has_japanese_title,
            has_chinese_title: primary.has_chinese_title || secondary.has_chinese_title,
            quality: primary.quality.or(secondary.quality),
            source: primary.source.or(secondary.source),
            language: primary.language.or(secondary.language),
        }
    }

    /// Extract XML tag content from string
    fn extract_xml_tag(content: &str, tag: &str) -> Option<String> {
        let start_tag = format!("<{}>", tag);
        let end_tag = format!("</{}>", tag);

        if let Some(start) = content.find(&start_tag) {
            let start_pos = start + start_tag.len();
            if let Some(end) = content[start_pos..].find(&end_tag) {
                return Some(content[start_pos..start_pos + end].trim().to_string());
            }
        }

        None
    }

    /// Determine if a metadata file should be kept after organization
    pub fn should_keep_metadata_file(file_path: &Path) -> bool {
        match file_path.extension().and_then(|s| s.to_str()) {
            Some("nfo") => true,   // Keep - valuable metadata
            Some("txt") => false,  // Delete - usually temporary
            Some("info") => false, // Delete - usually temporary
            Some("json") => false, // Delete - usually temporary
            _ => false,            // Delete everything else
        }
    }

    /// Get all metadata files that should be moved with the media file
    pub fn get_metadata_files_to_move(media_file_path: &Path) -> Result<Vec<PathBuf>> {
        let base_path = media_file_path.with_extension("");
        let mut metadata_files = Vec::new();

        // Check for .nfo files (the only ones we keep)
        let nfo_path = base_path.with_extension("nfo");
        if nfo_path.exists() {
            metadata_files.push(nfo_path);
        }

        Ok(metadata_files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_should_keep_metadata_file() {
        assert!(MetadataExtractor::should_keep_metadata_file(Path::new(
            "movie.nfo"
        )));
        assert!(!MetadataExtractor::should_keep_metadata_file(Path::new(
            "movie.txt"
        )));
        assert!(!MetadataExtractor::should_keep_metadata_file(Path::new(
            "movie.info"
        )));
        assert!(!MetadataExtractor::should_keep_metadata_file(Path::new(
            "movie.json"
        )));
    }

    #[test]
    fn test_extract_xml_tag() {
        let content = "<title>Test Movie</title><year>2023</year>";
        assert_eq!(
            MetadataExtractor::extract_xml_tag(content, "title"),
            Some("Test Movie".to_string())
        );
        assert_eq!(
            MetadataExtractor::extract_xml_tag(content, "year"),
            Some("2023".to_string())
        );
        assert_eq!(
            MetadataExtractor::extract_xml_tag(content, "nonexistent"),
            None
        );
    }

    #[test]
    fn test_parse_text_file() {
        let temp_dir = tempdir().unwrap();
        let text_file = temp_dir.path().join("movie.txt");

        fs::write(
            &text_file,
            "title: Test Movie\nyear: 2023\noriginal_title: Original Name",
        )
        .unwrap();

        let result = MetadataExtractor::parse_text_file(&text_file).unwrap();
        assert!(result.is_some());

        let movie_info = result.unwrap();
        assert_eq!(movie_info.title, "Test Movie");
        assert_eq!(movie_info.year, Some(2023));
        assert_eq!(movie_info.original_title, Some("Original Name".to_string()));
    }
}
