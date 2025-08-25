//! Series detection for movie collections and sequels

use regex::Regex;

use crate::parsers::types::SeriesInfo;

/// Series detection patterns and logic
#[derive(Clone, Debug)]
pub struct SeriesDetector {
    series_patterns: Vec<String>,
    collection_patterns: Vec<String>,
    #[allow(dead_code)]
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
                    if (1..=10).contains(&number) {
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
            .split(['.', '_', '-'])
            .filter(|part| !part.trim().is_empty())
            .collect::<Vec<_>>()
            .join(" ");

        title.trim().to_string()
    }

    /// Legacy method: Detect series patterns in a title (from filename_parser.rs)
    pub fn detect_series_pattern(&self, title: &str) -> Option<(String, u32)> {
        // Common series patterns - order matters for precedence
        // Use regex for precise pattern matching
        let series_patterns = [
            // "Part" patterns: "Iron Man Part 2", "Matrix Part 2" (check first to avoid conflicts)
            (r"^(.+?)\s+Part\s+(\d+)$", 2),
            // "Chapter" patterns: "Iron Man Chapter 2"
            (r"^(.+?)\s+Chapter\s+(\d+)$", 2),
            // "Volume" patterns: "Iron Man Volume 2"
            (r"^(.+?)\s+Volume\s+(\d+)$", 2),
            // "Episode" patterns: "Iron Man Episode 2"
            (r"^(.+?)\s+Episode\s+(\d+)$", 2),
            // "Season" patterns: "Iron Man Season 2"
            (r"^(.+?)\s+Season\s+(\d+)$", 2),
            // Number patterns: "Iron Man 2", "Matrix 2", "Avengers 2" (must end with number)
            (r"^(.+?)\s+(\d+)$", 2),
            // Roman numeral patterns: "Iron Man II", "Matrix II"
            (r"^(.+?)\s+(I{1,3}|IV|V|VI{1,3}|IX|X)$", 2),
        ];

        for (pattern, capture_group) in series_patterns.iter() {
            if let Some(captures) = regex::Regex::new(pattern)
                .ok()
                .and_then(|re| re.captures(title))
            {
                if let Some(series_name) = captures.get(1) {
                    if let Some(series_num_str) = captures.get(*capture_group) {
                        // Try to parse the series number
                        if let Ok(series_num) = series_num_str.as_str().parse::<u32>() {
                            let series_name_trimmed = series_name.as_str().trim();
                            // Validate that the series name doesn't end with a number (to avoid "Iron Man 2 3" -> "Iron Man 2", 3)
                            if !series_name_trimmed.ends_with(|c: char| c.is_ascii_digit()) {
                                return Some((series_name_trimmed.to_string(), series_num));
                            }
                        }
                        // Handle Roman numerals
                        if *capture_group == 2 && pattern.contains("I{1,3}|IV|V|VI{1,3}|IX|X") {
                            let roman_num = series_num_str.as_str();
                            if let Some(num) = self.roman_to_arabic(roman_num) {
                                return Some((series_name.as_str().trim().to_string(), num));
                            }
                        }
                    }
                }
            }
        }

        None
    }

    /// Convert Roman numeral to Arabic number (from filename_parser.rs)
    fn roman_to_arabic(&self, roman: &str) -> Option<u32> {
        let roman_map = [
            ("I", 1),
            ("II", 2),
            ("III", 3),
            ("IV", 4),
            ("V", 5),
            ("VI", 6),
            ("VII", 7),
            ("VIII", 8),
            ("IX", 9),
            ("X", 10),
        ];

        for (r, a) in roman_map.iter() {
            if roman == *r {
                return Some(*a);
            }
        }
        None
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

    #[test]
    fn test_legacy_detect_series_pattern() {
        let detector = SeriesDetector::new();

        // Test "Part" pattern
        let result = detector.detect_series_pattern("Iron Man Part 2");
        assert_eq!(result, Some(("Iron Man".to_string(), 2)));

        // Test number pattern
        let result = detector.detect_series_pattern("Matrix 2");
        assert_eq!(result, Some(("Matrix".to_string(), 2)));

        // Test Roman numeral pattern
        let result = detector.detect_series_pattern("Avengers II");
        assert_eq!(result, Some(("Avengers".to_string(), 2)));

        // Test no pattern
        let result = detector.detect_series_pattern("The Matrix");
        assert_eq!(result, None);
    }
}
