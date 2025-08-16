use plex_media_organizer::movie_parser::MovieParser;
use plex_media_organizer::types::MovieInfo;
use std::fs;
use std::path::Path;

/// Test utilities for dynamic testing against real-world data
pub struct DynamicTestRunner {
    parser: MovieParser,
}

impl DynamicTestRunner {
    pub fn new() -> Self {
        Self {
            parser: MovieParser::new(None),
        }
    }

    /// Parse tree output and extract movie filenames
    pub fn extract_filenames_from_tree(&self, tree_content: &str) -> Vec<String> {
        tree_content
            .lines()
            .filter_map(|line| {
                // Look for lines ending with media file extensions
                if line.trim().ends_with(".mkv")
                    || line.trim().ends_with(".mp4")
                    || line.trim().ends_with(".avi")
                    || line.trim().ends_with(".mov")
                    || line.trim().ends_with(".wmv")
                    || line.trim().ends_with(".flv")
                {
                    // Extract filename from tree line
                    self.extract_filename_from_tree_line(line)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Extract filename from a tree output line
    fn extract_filename_from_tree_line(&self, line: &str) -> Option<String> {
        // Tree lines look like:
        // ├── filename.mkv
        // └── filename.mkv
        // │   ├── filename.mkv
        // │   └── filename.mkv

        let trimmed = line.trim();

        // Use char-based operations to handle Unicode properly
        let chars: Vec<char> = trimmed.chars().collect();

        // Find the last occurrence of common tree patterns
        for i in (0..chars.len()).rev() {
            if i + 3 < chars.len() {
                // Check for "├── " pattern
                if chars[i] == '├'
                    && chars[i + 1] == '─'
                    && chars[i + 2] == '─'
                    && chars[i + 3] == ' '
                {
                    let filename: String = chars[i + 4..].iter().collect();
                    return Some(filename.trim().to_string());
                }
                // Check for "└── " pattern
                if chars[i] == '└'
                    && chars[i + 1] == '─'
                    && chars[i + 2] == '─'
                    && chars[i + 3] == ' '
                {
                    let filename: String = chars[i + 4..].iter().collect();
                    return Some(filename.trim().to_string());
                }
            }
            if i + 7 < chars.len() {
                // Check for "│   ├── " pattern
                if chars[i] == '│'
                    && chars[i + 1] == ' '
                    && chars[i + 2] == ' '
                    && chars[i + 3] == ' '
                    && chars[i + 4] == '├'
                    && chars[i + 5] == '─'
                    && chars[i + 6] == '─'
                    && chars[i + 7] == ' '
                {
                    let filename: String = chars[i + 8..].iter().collect();
                    return Some(filename.trim().to_string());
                }
                // Check for "│   └── " pattern
                if chars[i] == '│'
                    && chars[i + 1] == ' '
                    && chars[i + 2] == ' '
                    && chars[i + 3] == ' '
                    && chars[i + 4] == '└'
                    && chars[i + 5] == '─'
                    && chars[i + 6] == '─'
                    && chars[i + 7] == ' '
                {
                    let filename: String = chars[i + 8..].iter().collect();
                    return Some(filename.trim().to_string());
                }
            }
        }

        None
    }

    /// Run comprehensive tests against a collection of filenames
    pub fn run_dynamic_tests(&self, filenames: &[String]) -> DynamicTestResults {
        let mut results = DynamicTestResults::new();

        for filename in filenames {
            match self.parser.parse_filename(filename) {
                Ok(movie_info) => {
                    results.add_success(filename, movie_info);
                }
                Err(e) => {
                    results.add_failure(filename, e.to_string());
                }
            }
        }

        results
    }

    /// Test against a tree file
    pub fn test_tree_file(
        &self,
        tree_file_path: &Path,
    ) -> Result<DynamicTestResults, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(tree_file_path)?;
        let filenames = self.extract_filenames_from_tree(&content);

        println!("📁 Found {} movie files in tree output", filenames.len());

        let results = self.run_dynamic_tests(&filenames);
        Ok(results)
    }

    /// Analyze pattern distribution in successful parses
    pub fn analyze_patterns(&self, results: &DynamicTestResults) -> PatternAnalysis {
        let mut analysis = PatternAnalysis::new();

        for (filename, movie_info) in &results.successful_parses {
            // Analyze Chinese-English bilingual patterns
            if self.is_chinese_english_bilingual(filename) {
                analysis.chinese_english_count += 1;
            }

            // Analyze bracketed patterns
            if self.is_bracketed_pattern(filename) {
                analysis.bracketed_count += 1;
            }

            // Analyze multi-part patterns
            if self.is_multipart_pattern(filename) {
                analysis.multipart_count += 1;
            }

            // Analyze quality patterns
            if let Some(quality) = &movie_info.quality {
                analysis.quality_patterns.insert(quality.clone());
            }

            // Analyze source patterns
            if let Some(source) = &movie_info.source {
                analysis.source_patterns.insert(source.clone());
            }
        }

        analysis.total_files = results.successful_parses.len();
        analysis
    }

    /// Check if filename is Chinese-English bilingual
    fn is_chinese_english_bilingual(&self, filename: &str) -> bool {
        // Look for Chinese characters followed by English
        filename.chars().any(|c| c.is_ascii() == false)
            && filename.chars().any(|c| c.is_ascii_alphabetic())
    }

    /// Check if filename has bracketed pattern
    fn is_bracketed_pattern(&self, filename: &str) -> bool {
        filename.contains('[') && filename.contains(']')
    }

    /// Check if filename is multi-part
    fn is_multipart_pattern(&self, filename: &str) -> bool {
        filename.to_lowercase().contains("part")
            || filename.to_lowercase().contains("cd")
            || filename.to_lowercase().contains("三部曲")
    }
}

/// Results from dynamic testing
#[derive(Debug)]
pub struct DynamicTestResults {
    pub successful_parses: Vec<(String, MovieInfo)>,
    pub failed_parses: Vec<(String, String)>,
    pub total_files: usize,
}

impl DynamicTestResults {
    pub fn new() -> Self {
        Self {
            successful_parses: Vec::new(),
            failed_parses: Vec::new(),
            total_files: 0,
        }
    }

    pub fn add_success(&mut self, filename: &str, movie_info: MovieInfo) {
        self.successful_parses
            .push((filename.to_string(), movie_info));
        self.total_files += 1;
    }

    pub fn add_failure(&mut self, filename: &str, error: String) {
        self.failed_parses.push((filename.to_string(), error));
        self.total_files += 1;
    }

    pub fn success_rate(&self) -> f32 {
        if self.total_files == 0 {
            0.0
        } else {
            self.successful_parses.len() as f32 / self.total_files as f32
        }
    }

    pub fn print_summary(&self) {
        println!("📊 DYNAMIC TEST RESULTS SUMMARY");
        println!("=================================");
        println!(
            "✅ Successfully parsed: {} files",
            self.successful_parses.len()
        );
        println!("❌ Failed to parse: {} files", self.failed_parses.len());
        println!("📁 Total files tested: {} files", self.total_files);
        println!("📈 Success rate: {:.1}%", self.success_rate() * 100.0);

        if !self.failed_parses.is_empty() {
            println!("\n❌ FAILED PARSES (first 10):");
            for (filename, error) in self.failed_parses.iter().take(10) {
                println!("   • {}: {}", filename, error);
            }
            if self.failed_parses.len() > 10 {
                println!("   ... and {} more", self.failed_parses.len() - 10);
            }
        }
    }
}

/// Analysis of pattern distribution
#[derive(Debug)]
pub struct PatternAnalysis {
    pub total_files: usize,
    pub chinese_english_count: usize,
    pub bracketed_count: usize,
    pub multipart_count: usize,
    pub quality_patterns: std::collections::HashSet<String>,
    pub source_patterns: std::collections::HashSet<String>,
}

impl PatternAnalysis {
    pub fn new() -> Self {
        Self {
            total_files: 0,
            chinese_english_count: 0,
            bracketed_count: 0,
            multipart_count: 0,
            quality_patterns: std::collections::HashSet::new(),
            source_patterns: std::collections::HashSet::new(),
        }
    }

    pub fn print_analysis(&self) {
        println!("\n🎯 PATTERN ANALYSIS");
        println!("===================");
        println!(
            "🌏 Chinese-English bilingual: {} files ({:.1}%)",
            self.chinese_english_count,
            self.percentage(self.chinese_english_count)
        );
        println!(
            "📦 Bracketed patterns: {} files ({:.1}%)",
            self.bracketed_count,
            self.percentage(self.bracketed_count)
        );
        println!(
            "🔗 Multi-part movies: {} files ({:.1}%)",
            self.multipart_count,
            self.percentage(self.multipart_count)
        );

        println!("\n🎯 Quality patterns found:");
        for quality in &self.quality_patterns {
            println!("   • {}", quality);
        }

        println!("\n📺 Source patterns found:");
        for source in &self.source_patterns {
            println!("   • {}", source);
        }
    }

    fn percentage(&self, count: usize) -> f32 {
        if self.total_files == 0 {
            0.0
        } else {
            count as f32 * 100.0 / self.total_files as f32
        }
    }
}
