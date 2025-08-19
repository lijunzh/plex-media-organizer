use plex_media_organizer::{MovieInfo, MovieParser};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Test to scan entire /Volumes/media/movie directory for edge cases
#[test]
fn test_scan_entire_movie_directory() {
    let movie_dir = Path::new("/Volumes/media/movie");

    if !movie_dir.exists() {
        println!("⚠️  Movie directory /Volumes/media/movie does not exist, skipping test");
        return;
    }

    let parser = MovieParser::new(None);
    let mut results = Vec::new();
    let mut error_cases = Vec::new();
    let mut edge_cases = Vec::new();
    let mut statistics = HashMap::new();

    println!(
        "🔍 Scanning entire movie directory: {}",
        movie_dir.display()
    );
    println!("{}", "=".repeat(80));

    // Scan all subdirectories
    if let Ok(entries) = fs::read_dir(movie_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                println!("📁 Scanning directory: {}", path.display());
                scan_directory(
                    &path,
                    &parser,
                    &mut results,
                    &mut error_cases,
                    &mut edge_cases,
                    &mut statistics,
                );
            }
        }
    }

    // Print summary
    println!("\n\n");
    println!("📊 SCAN SUMMARY");
    println!("{}", "=".repeat(80));
    println!("Total files processed: {}", results.len());
    println!(
        "Successful parses: {}",
        results.iter().filter(|r| r.is_ok()).count()
    );
    println!(
        "Failed parses: {}",
        results.iter().filter(|r| r.is_err()).count()
    );

    if !error_cases.is_empty() {
        println!("\n❌ ERROR CASES ({}):", error_cases.len());
        println!("{}", "-".repeat(40));
        for (filename, error) in error_cases.iter().take(20) {
            println!("📄 {}: {}", filename, error);
        }
        if error_cases.len() > 20 {
            println!("... and {} more error cases", error_cases.len() - 20);
        }
    }

    if !edge_cases.is_empty() {
        println!("\n⚠️  EDGE CASES ({}):", edge_cases.len());
        println!("{}", "-".repeat(40));
        for (filename, details) in edge_cases.iter().take(20) {
            println!("📄 {}: {}", filename, details);
        }
        if edge_cases.len() > 20 {
            println!("... and {} more edge cases", edge_cases.len() - 20);
        }
    }

    // Print statistics
    println!("\n📈 PARSING STATISTICS:");
    println!("{}", "-".repeat(40));
    for (category, count) in &statistics {
        println!("{}: {}", category, count);
    }

    // Assert that we have reasonable success rate
    let success_rate = results.iter().filter(|r| r.is_ok()).count() as f64 / results.len() as f64;
    println!("\n✅ Success rate: {:.1}%", success_rate * 100.0);

    // Don't fail the test, just report findings
    if success_rate < 0.8 {
        println!("⚠️  Warning: Success rate below 80% - may need attention");
    }
}

fn scan_directory(
    dir: &Path,
    parser: &MovieParser,
    results: &mut Vec<Result<MovieInfo, String>>,
    error_cases: &mut Vec<(String, String)>,
    edge_cases: &mut Vec<(String, String)>,
    statistics: &mut HashMap<String, usize>,
) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();

            if path.is_file() {
                if let Some(extension) = path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    if ["mkv", "mp4", "avi", "mov", "wmv", "flv", "webm"].contains(&ext.as_str()) {
                        let filename = path.file_name().unwrap().to_string_lossy().to_string();

                        // Parse the file
                        match parser.parse_filename(&filename) {
                            Ok(result) => {
                                results.push(Ok(result.clone()));

                                // Check for potential edge cases
                                let edge_case = analyze_for_edge_cases(&filename, &result);
                                if let Some(details) = edge_case {
                                    edge_cases.push((filename.clone(), details));
                                }

                                // Update statistics
                                update_statistics(&result, statistics);
                            }
                            Err(e) => {
                                results.push(Err(e.to_string()));
                                error_cases.push((filename, e.to_string()));
                            }
                        }
                    }
                }
            } else if path.is_dir() {
                // Recursively scan subdirectories
                scan_directory(&path, parser, results, error_cases, edge_cases, statistics);
            }
        }
    }
}

fn analyze_for_edge_cases(filename: &str, result: &MovieInfo) -> Option<String> {
    let mut issues = Vec::new();

    // Check for empty or very short titles
    if result.title.is_empty() {
        issues.push("Empty title".to_string());
    } else if result.title.len() < 3 {
        issues.push("Very short title".to_string());
    }

    // Check for titles that are too long
    if result.title.len() > 200 {
        issues.push("Very long title".to_string());
    }

    // Check for titles with excessive technical terms
    let technical_terms = [
        "DDP", "DTS", "AC3", "H264", "H265", "x264", "x265", "WEB-DL", "BluRay", "REMUX",
    ];
    let tech_count = technical_terms
        .iter()
        .filter(|term| result.title.to_uppercase().contains(*term))
        .count();
    if tech_count > 2 {
        issues.push(format!("Contains {} technical terms", tech_count));
    }

    // Check for titles with excessive punctuation
    let punct_count = result
        .title
        .chars()
        .filter(|c| !c.is_alphanumeric() && !c.is_whitespace())
        .count();
    if punct_count > 10 {
        issues.push(format!("Excessive punctuation ({})", punct_count));
    }

    // Check for titles with numbers that might be years but aren't extracted
    let numbers: Vec<&str> = result
        .title
        .split_whitespace()
        .filter(|word| word.chars().all(|c| c.is_ascii_digit()) && word.len() == 4)
        .collect();
    if !numbers.is_empty() && result.year.is_none() {
        issues.push(format!("Contains year-like numbers: {:?}", numbers));
    }

    // Check for titles with mixed languages that might not be handled well
    let has_chinese = result.title.chars().any(|c| {
        let code = c as u32;
        (0x4E00..=0x9FFF).contains(&code)
    });
    let has_japanese = result.title.chars().any(|c| {
        let code = c as u32;
        (0x3040..=0x309F).contains(&code) || (0x30A0..=0x30FF).contains(&code)
    });
    let has_english = result.title.chars().any(|c| c.is_ascii_alphabetic());

    if has_chinese && has_japanese && has_english {
        issues.push("Complex trilingual title".to_string());
    } else if (has_chinese && has_english) || (has_japanese && has_english) {
        issues.push("Bilingual title".to_string());
    }

    // Check for titles with brackets that might not be handled correctly
    if result.title.contains('[') || result.title.contains(']') {
        issues.push("Contains brackets".to_string());
    }

    // Check for titles with dots that might indicate incomplete parsing
    if result.title.contains("..") {
        issues.push("Contains double dots (incomplete parsing)".to_string());
    }

    // Check for titles that are mostly the same as the filename (might indicate poor parsing)
    let filename_clean = filename.replace(['.', '-', '_'], " ").to_lowercase();
    let title_clean = result.title.to_lowercase();
    if filename_clean.len() > 20 && title_clean.len() > 20 {
        let similarity = calculate_similarity(&filename_clean, &title_clean);
        if similarity > 0.8 {
            issues.push(format!(
                "Very similar to filename ({}% similarity)",
                (similarity * 100.0) as i32
            ));
        }
    }

    if issues.is_empty() {
        None
    } else {
        Some(issues.join(", "))
    }
}

fn calculate_similarity(s1: &str, s2: &str) -> f64 {
    let words1: Vec<&str> = s1.split_whitespace().collect();
    let words2: Vec<&str> = s2.split_whitespace().collect();

    let common_words = words1.iter().filter(|w1| words2.contains(w1)).count();
    let total_words = words1.len().max(words2.len());

    if total_words == 0 {
        0.0
    } else {
        common_words as f64 / total_words as f64
    }
}

fn update_statistics(result: &MovieInfo, statistics: &mut HashMap<String, usize>) {
    // Count languages
    if let Some(lang) = &result.language {
        *statistics.entry(format!("Language: {}", lang)).or_insert(0) += 1;
    }

    // Count quality types
    if let Some(quality) = &result.quality {
        *statistics
            .entry(format!("Quality: {}", quality))
            .or_insert(0) += 1;
    }

    // Count source types
    if let Some(source) = &result.source {
        *statistics.entry(format!("Source: {}", source)).or_insert(0) += 1;
    }

    // Count years
    if let Some(year) = result.year {
        *statistics.entry(format!("Year: {}", year)).or_insert(0) += 1;
    }

    // Count title lengths
    let title_len = result.title.len();
    let len_category = match title_len {
        0..=10 => "Title length: 0-10",
        11..=30 => "Title length: 11-30",
        31..=60 => "Title length: 31-60",
        _ => "Title length: 60+",
    };
    *statistics.entry(len_category.to_string()).or_insert(0) += 1;

    // Count original titles
    if result.original_title.is_some() {
        *statistics
            .entry("Has original title".to_string())
            .or_insert(0) += 1;
    } else {
        *statistics
            .entry("No original title".to_string())
            .or_insert(0) += 1;
    }
}
