//! Series detection for movie collections and sequels

use regex::Regex;

use crate::parsers::types::SeriesInfo;

/// Series detection patterns and logic
#[derive(Clone, Debug)]
pub struct SeriesDetector {
    series_patterns: Vec<String>,
    collection_patterns: Vec<String>,
    part_patterns: Vec<String>,
}

impl Default for SeriesDetector {
    fn default() -> Self {
        Self {
            series_patterns: vec![
                "Part".to_string(),
                "CD".to_string(),
                "Disc".to_string(),
                "Volume".to_string(),
                "Vol".to_string(),
            ],
            collection_patterns: vec![
                "Collection".to_string(),
                "Trilogy".to_string(),
                "Saga".to_string(),
                "Series".to_string(),
            ],
            part_patterns: vec![
                "Part 1".to_string(),
                "Part 2".to_string(),
                "Part 3".to_string(),
                "CD1".to_string(),
                "CD2".to_string(),
                "CD3".to_string(),
                "Disc 1".to_string(),
                "Disc 2".to_string(),
                "Disc 3".to_string(),
            ],
        }
    }
}

impl SeriesDetector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Detect if a filename contains series information
    pub fn detect_series(&self, filename: &str) -> SeriesInfo {
        let mut series_info = SeriesInfo {
            is_series: false,
            series_number: None,
            series_type: None,
            total_parts: None,
        };

        // Check for series patterns
        for pattern in &self.series_patterns {
            if filename.to_uppercase().contains(&pattern.to_uppercase()) {
                series_info.is_series = true;
                series_info.series_type = Some(pattern.clone());
                break;
            }
        }

        // Check for collection patterns
        for pattern in &self.collection_patterns {
            if filename.to_uppercase().contains(&pattern.to_uppercase()) {
                series_info.is_series = true;
                series_info.series_type = Some(pattern.clone());
                break;
            }
        }

        // Extract series number
        series_info.series_number = self.extract_series_number(filename);

        // Estimate total parts based on filename
        series_info.total_parts = self.estimate_total_parts(filename);

        series_info
    }

    /// Extract series number from filename
    fn extract_series_number(&self, filename: &str) -> Option<u32> {
        // Look for patterns like "Part 1", "CD1", "Disc 2", etc.
        let patterns = vec![
            r"Part\s+(\d+)",
            r"CD(\d+)",
            r"Disc\s+(\d+)",
            r"Volume\s+(\d+)",
            r"Vol\s+(\d+)",
        ];

        for pattern in patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(captures) = regex.captures(filename) {
                    if let Some(number_str) = captures.get(1) {
                        if let Ok(number) = number_str.as_str().parse::<u32>() {
                            return Some(number);
                        }
                    }
                }
            }
        }

        // Also check for simple number patterns that might be series numbers
        let simple_number_regex = Regex::new(r"\b(\d+)\b").unwrap();
        if let Some(captures) = simple_number_regex.captures(filename) {
            if let Some(number_str) = captures.get(1) {
                if let Ok(number) = number_str.as_str().parse::<u32>() {
                    // Only return if it's a reasonable series number (1-10)
                    if number >= 1 && number <= 10 {
                        return Some(number);
                    }
                }
            }
        }

        None
    }

    /// Estimate total number of parts based on filename patterns
    fn estimate_total_parts(&self, filename: &str) -> Option<u32> {
        // Look for patterns that indicate multiple parts
        if filename.to_uppercase().contains("TRILOGY") {
            return Some(3);
        }
        if filename.to_uppercase().contains("COLLECTION") {
            // Default to 2 for collections, but this is just an estimate
            return Some(2);
        }
        if filename.to_uppercase().contains("SAGA") {
            // Default to 3 for sagas
            return Some(3);
        }

        None
    }

    /// Analyze if a movie is part of a collection
    pub fn analyze_collection(&self, filename: &str) -> CollectionAnalysis {
        let series_info = self.detect_series(filename);

        CollectionAnalysis {
            is_collection: series_info.is_series,
            collection_type: series_info.series_type,
            part_number: series_info.series_number,
            total_parts: series_info.total_parts,
            base_title: self.extract_base_title(filename),
        }
    }

    /// Extract the base title without series information
    pub fn extract_base_title(&self, filename: &str) -> String {
        let mut title = filename.to_string();

        // Remove file extension
        if let Some(dot_pos) = title.rfind('.') {
            title = title[..dot_pos].to_string();
        }

        // Remove series patterns
        for pattern in &self.series_patterns {
            let pattern_upper = pattern.to_uppercase();
            if title.to_uppercase().contains(&pattern_upper) {
                // Remove the pattern and any following numbers
                let regex_pattern = format!(r"(?i){}\s*\d*", regex::escape(pattern));
                if let Ok(regex) = Regex::new(&regex_pattern) {
                    title = regex.replace_all(&title, "").to_string();
                }
            }
        }

        // Also remove standalone numbers that might be series numbers
        let number_regex = Regex::new(r"\b\d+\b").unwrap();
        title = number_regex.replace_all(&title, "").to_string();

        // Remove collection patterns
        for pattern in &self.collection_patterns {
            let pattern_upper = pattern.to_uppercase();
            if title.to_uppercase().contains(&pattern_upper) {
                let regex_pattern = format!(r"(?i){}", regex::escape(pattern));
                if let Ok(regex) = Regex::new(&regex_pattern) {
                    title = regex.replace_all(&title, "").to_string();
                }
            }
        }

        // Remove common technical terms
        let technical_terms = vec![
            "1080p", "720p", "4K", "HDR", "UHD", "HD", "SD", "BluRay", "WEB-DL", "HDTV", "DVDRip",
            "BRRip", "HDRip", "x264", "x265", "H264", "H265", "AVC", "HEVC", "DTS", "AC3", "AAC",
            "FLAC", "DD5.1", "DTS-HD",
        ];

        for term in technical_terms {
            let term_upper = term.to_uppercase();
            if title.to_uppercase().contains(&term_upper) {
                let regex_pattern = format!(r"(?i){}", regex::escape(term));
                if let Ok(regex) = Regex::new(&regex_pattern) {
                    title = regex.replace_all(&title, "").to_string();
                }
            }
        }

        // Clean up extra whitespace and separators
        title = title
            .split(|c| c == '.' || c == '_' || c == '-')
            .filter(|part| !part.trim().is_empty())
            .collect::<Vec<_>>()
            .join(" ");

        title.trim().to_string()
    }
}

/// Collection analysis result
#[derive(Debug, Clone)]
pub struct CollectionAnalysis {
    pub is_collection: bool,
    pub collection_type: Option<String>,
    pub part_number: Option<u32>,
    pub total_parts: Option<u32>,
    pub base_title: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_series_part() {
        let detector = SeriesDetector::new();
        let result = detector.detect_series("Movie.Part.1.1080p.mkv");

        assert!(result.is_series);
        assert_eq!(result.series_number, Some(1));
        assert_eq!(result.series_type, Some("Part".to_string()));
    }

    #[test]
    fn test_detect_series_cd() {
        let detector = SeriesDetector::new();
        let result = detector.detect_series("Movie.CD1.1080p.mkv");

        assert!(result.is_series);
        assert_eq!(result.series_number, Some(1));
        assert_eq!(result.series_type, Some("CD".to_string()));
    }

    #[test]
    fn test_detect_collection() {
        let detector = SeriesDetector::new();
        let result = detector.detect_series("Movie.Collection.1080p.mkv");

        assert!(result.is_series);
        assert_eq!(result.series_type, Some("Collection".to_string()));
        assert_eq!(result.total_parts, Some(2)); // Default estimate
    }

    #[test]
    fn test_extract_base_title() {
        let detector = SeriesDetector::new();
        let base_title = detector.extract_base_title("The.Matrix.Part.1.1080p.mkv");

        assert_eq!(base_title, "The Matrix");
    }

    #[test]
    fn test_extract_base_title_with_collection() {
        let detector = SeriesDetector::new();
        let base_title = detector.extract_base_title("Lord.of.the.Rings.Collection.1080p.mkv");

        assert_eq!(base_title, "Lord of the Rings");
    }
}
