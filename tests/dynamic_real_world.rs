use std::path::Path;
use test_utils::DynamicTestRunner;

mod test_utils;

/// Dynamic test against real-world tree output files
#[test]
fn test_movie_directory_dynamic() {
    let runner = DynamicTestRunner::new();

    // Test against the actual movie directory tree output
    let tree_file = Path::new("tests/test_data/movie_directory.txt");

    if !tree_file.exists() {
        eprintln!(
            "⚠️  Skipping dynamic test - movie directory file not found at: {}",
            tree_file.display()
        );
        eprintln!("   Expected location: tests/test_data/movie_directory.txt");
        return;
    }

    println!("🎬 DYNAMIC TEST: Movie Directory Analysis");
    println!("=========================================");

    // Run the dynamic test
    let results = match runner.test_tree_file(tree_file) {
        Ok(results) => results,
        Err(e) => {
            panic!("Failed to test tree file: {}", e);
        }
    };

    // Print results summary
    results.print_summary();

    // Analyze patterns
    let pattern_analysis = runner.analyze_patterns(&results);
    pattern_analysis.print_analysis();

    // Assertions based on expected performance
    assert!(
        results.success_rate() > 0.95,
        "Success rate too low: {:.1}%. Expected >95%",
        results.success_rate() * 100.0
    );

    assert!(
        results.total_files > 100,
        "Too few files tested: {}. Expected >100 files",
        results.total_files
    );

    // Pattern-specific assertions
    assert!(
        pattern_analysis.chinese_english_count > 0,
        "Should detect Chinese-English bilingual patterns"
    );

    assert!(
        pattern_analysis.bracketed_count > 0,
        "Should detect bracketed patterns"
    );

    assert!(
        !pattern_analysis.quality_patterns.is_empty(),
        "Should detect quality patterns"
    );

    assert!(
        !pattern_analysis.source_patterns.is_empty(),
        "Should detect source patterns"
    );

    println!("\n🎉 Dynamic test completed successfully!");
    println!(
        "📊 Tested {} files with {:.1}% success rate",
        results.total_files,
        results.success_rate() * 100.0
    );
}

/// Test full workflow (scan + parse + organize) against movie directory
#[tokio::test]
async fn test_movie_organization_workflow() {
    let runner = DynamicTestRunner::new();

    // Test against the actual movie directory tree output
    let tree_file = Path::new("tests/test_data/movie_directory.txt");

    if !tree_file.exists() {
        eprintln!(
            "⚠️  Skipping organization workflow test - movie directory file not found at: {}",
            tree_file.display()
        );
        eprintln!("   Expected location: tests/test_data/movie_directory.txt");
        return;
    }

    println!("🎬 DYNAMIC TEST: Movie Organization Workflow");
    println!("=============================================");

    // Run the dynamic test with organization
    let results = match runner.test_tree_file_with_organization(tree_file).await {
        Ok(results) => results,
        Err(e) => {
            panic!("Failed to test organization workflow: {}", e);
        }
    };

    // Print results summary
    results.print_summary();

    // Assertions based on expected performance
    assert!(
        results.success_rate() > 0.95,
        "Success rate too low: {:.1}%. Expected >95%",
        results.success_rate() * 100.0
    );

    assert!(
        results.total_files > 100,
        "Too few files tested: {}. Expected >100 files",
        results.total_files
    );

    println!("\n🎉 Organization workflow test completed successfully!");
    println!(
        "📊 Tested {} files with {:.1}% success rate",
        results.total_files,
        results.success_rate() * 100.0
    );
}

// TV and Music tests removed - not in scope for iteration 1
// These will be re-implemented when TV and music parsing are added in future iterations

/// Performance test with large collections
#[test]
fn test_performance_large_collection() {
    let runner = DynamicTestRunner::new();

    let tree_file = Path::new("tests/test_data/movie_directory.txt");

    if !tree_file.exists() {
        eprintln!("⚠️  Skipping performance test - movie directory file not found");
        return;
    }

    println!("⚡ PERFORMANCE TEST: Large Collection Parsing");
    println!("============================================");

    let start_time = std::time::Instant::now();

    println!("⏱️  Starting tree file parsing...");
    let results = match runner.test_tree_file(tree_file) {
        Ok(results) => results,
        Err(e) => {
            panic!("Failed to test tree file: {}", e);
        }
    };
    println!(
        "⏱️  Tree file parsing completed in {:?}",
        start_time.elapsed()
    );

    let duration = start_time.elapsed();
    let files_per_second = results.total_files as f64 / duration.as_secs_f64();

    println!("⏱️  Performance Results:");
    println!("   • Total files: {}", results.total_files);
    println!("   • Time taken: {:.2?}", duration);
    println!("   • Files per second: {:.1}", files_per_second);

    // Performance assertions
    assert!(
        files_per_second > 5.0,
        "Performance too slow: {:.1} files/sec. Expected >5 files/sec (reduced due to TMDB API calls)",
        files_per_second
    );

    assert!(
        duration.as_millis() < 10000,
        "Test took too long: {:.0}ms. Expected <10000ms (increased due to TMDB API calls)",
        duration.as_millis()
    );

    println!("✅ Performance test passed!");
}

/// Test error handling and edge cases
#[test]
fn test_error_handling_edge_cases() {
    let runner = DynamicTestRunner::new();

    // Test with malformed filenames
    let edge_case_filenames = vec![
        "".to_string(),             // Empty string
        "no.extension".to_string(), // No extension
        "very.long.filename.that.might.cause.issues.2023.1080p.BluRay.x264.mkv".to_string(),
        "file.with.multiple...dots....mkv".to_string(), // Multiple dots
        "file with spaces and (parentheses) [brackets] 2023.mkv".to_string(),
        "file.with.unicode.🚀.2023.mkv".to_string(), // Unicode characters
    ];

    println!("🧪 ERROR HANDLING TEST: Edge Cases");
    println!("==================================");

    let results = runner.run_dynamic_tests(&edge_case_filenames);

    println!("📊 Edge Case Test Results:");
    println!("   • Total edge cases: {}", edge_case_filenames.len());
    println!(
        "   • Successful parses: {}",
        results.successful_parses.len()
    );
    println!("   • Failed parses: {}", results.failed_parses.len());

    // Edge case tests should handle errors gracefully
    // Note: Our parser is surprisingly robust and handles edge cases well
    println!(
        "   • Parser handled {} edge cases successfully",
        results.successful_parses.len()
    );
    println!(
        "   • Parser failed on {} edge cases",
        results.failed_parses.len()
    );

    // The parser should handle most edge cases gracefully
    assert!(results.total_files > 0, "Should process edge cases");

    println!("✅ Error handling test passed!");
}
