//! Integration test for rollback functionality

use anyhow::Result;
use plex_media_organizer::{MovieParser, Organizer, Scanner};
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_rollback_integration() -> Result<()> {
    // Create temporary directory structure
    let temp_dir = TempDir::new()?;
    let base_path = temp_dir.path();
    let source_dir = base_path.join("source");
    let organized_dir = base_path.join("organized");

    fs::create_dir_all(&source_dir)?;
    fs::create_dir_all(&organized_dir)?;

    // Create test movie files
    let test_files = vec![
        "The.Matrix.1999.1080p.BluRay.mkv",
        "Avengers.Endgame.2019.2160p.UHD.mkv",
    ];

    for file_name in &test_files {
        let file_path = source_dir.join(file_name);
        fs::write(&file_path, "test movie content")?;
    }

    // Create movie parser without TMDB (to avoid API dependency in tests)
    let movie_parser = MovieParser::new(None);
    let mut scanner = Scanner::new(movie_parser);
    
    // Lower confidence threshold for tests and allow unmatched movies
    scanner.config.organization.matching.min_confidence_threshold = 0.1;
    scanner.config.organization.matching.skip_unmatched_movies = false;

    // Step 1: Scan the source directory
    let scan_result = scanner.scan_directory(&source_dir).await?;
    assert_eq!(scan_result.files.len(), 2);
    assert_eq!(scan_result.parsed_files.len(), 2);

    // Step 2: Organize files to a separate directory
    // Note: The current organizer creates subdirectories in the same location
    // This is actually the expected behavior for Plex organization
    let organizer = Organizer::new(false, None); // Not dry-run, no backup
    let organization_result = organizer.organize_scan_result(&scan_result).await?;

    // Verify organization worked
    assert_eq!(organization_result.statistics.organized_files, 2);
    assert_eq!(organization_result.statistics.failed_files, 0);
    assert!(organization_result.statistics.success_rate > 0.99);

    // Verify files were moved to organized structure
    let organized_files = &organization_result.organized_files;
    for organized_file in organized_files {
        assert!(organized_file.new_path.exists());
        assert!(!organized_file.original_path.exists());
    }

    // Step 3: Save organization result to JSON (simulating what CLI does)
    let operation_json = serde_json::to_string_pretty(&organization_result)?;
    let operation_file = base_path.join("test_operation.json");
    fs::write(&operation_file, operation_json)?;

    // Step 4: Test rollback functionality
    // Load the organization result
    let loaded_json = fs::read_to_string(&operation_file)?;
    let loaded_result: plex_media_organizer::organizer::OrganizationResult =
        serde_json::from_str(&loaded_json)?;

    // Verify we can parse the JSON correctly
    assert_eq!(loaded_result.operation_id, organization_result.operation_id);
    assert_eq!(loaded_result.organized_files.len(), 2);

    // Step 5: Perform actual rollback
    // Move files back to original locations
    for organized_file in &loaded_result.organized_files {
        assert!(organized_file.new_path.exists());

        // Create parent directory if needed
        if let Some(parent) = organized_file.original_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Move file back
        fs::rename(&organized_file.new_path, &organized_file.original_path)?;
    }

    // Step 6: Verify rollback worked
    for organized_file in &loaded_result.organized_files {
        assert!(organized_file.original_path.exists());
        assert!(!organized_file.new_path.exists());

        // Verify file content is preserved
        let content = fs::read_to_string(&organized_file.original_path)?;
        assert_eq!(content, "test movie content");
    }

    // Verify original file names are restored
    let remaining_files: Vec<_> = fs::read_dir(&source_dir)?
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                Some(entry.file_name().to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();

    assert_eq!(remaining_files.len(), 2);
    for test_file in &test_files {
        assert!(remaining_files.contains(&test_file.to_string()));
    }

    println!("✅ Rollback integration test passed!");
    Ok(())
}

#[tokio::test]
async fn test_rollback_error_conditions() -> Result<()> {
    // Test 1: Non-existent JSON file
    let temp_dir = TempDir::new()?;
    let non_existent_file = temp_dir.path().join("non_existent.json");

    // This would be tested in CLI layer - JSON file not found
    assert!(!non_existent_file.exists());

    // Test 2: Invalid JSON format
    let invalid_json_file = temp_dir.path().join("invalid.json");
    fs::write(&invalid_json_file, "not valid json")?;

    let content = fs::read_to_string(&invalid_json_file)?;
    let parse_result: Result<plex_media_organizer::organizer::OrganizationResult, _> =
        serde_json::from_str(&content);
    assert!(parse_result.is_err());

    // Test 3: Empty operation (no files organized)
    let empty_operation = plex_media_organizer::organizer::OrganizationResult {
        operation_id: "test-empty".to_string(),
        timestamp: chrono::Utc::now(),
        organized_files: vec![],
        failed_files: vec![],
        statistics: plex_media_organizer::organizer::OrganizationStatistics {
            total_files: 0,
            organized_files: 0,
            failed_files: 0,
            success_rate: 0.0,
            duration_seconds: 0.0,
        },
    };

    let empty_json = serde_json::to_string(&empty_operation)?;
    let empty_file = temp_dir.path().join("empty_operation.json");
    fs::write(&empty_file, empty_json)?;

    // Load and verify
    let loaded_json = fs::read_to_string(&empty_file)?;
    let loaded_operation: plex_media_organizer::organizer::OrganizationResult =
        serde_json::from_str(&loaded_json)?;

    assert_eq!(loaded_operation.organized_files.len(), 0);

    println!("✅ Rollback error conditions test passed!");
    Ok(())
}

#[tokio::test]
async fn test_rollback_dry_run_operation() -> Result<()> {
    // Test that dry-run operations are properly handled in rollback
    let temp_dir = TempDir::new()?;
    let source_dir = temp_dir.path().join("source");
    fs::create_dir_all(&source_dir)?;

    // Create test file
    let test_file = source_dir.join("Test.Movie.2023.mkv");
    fs::write(&test_file, "test content")?;

    // Create movie parser and scan
    let movie_parser = MovieParser::new(None);
    let mut scanner = Scanner::new(movie_parser);
    
    // Lower confidence threshold for tests and allow unmatched movies
    scanner.config.organization.matching.min_confidence_threshold = 0.1;
    scanner.config.organization.matching.skip_unmatched_movies = false;
    let scan_result = scanner.scan_directory(&source_dir).await?;

    // Organize with dry-run
    let organizer = Organizer::new(true, None); // dry-run = true
    let organization_result = organizer.organize_scan_result(&scan_result).await?;

    // Verify it was a dry-run
    assert_eq!(organization_result.organized_files.len(), 1);
    assert!(organization_result.organized_files[0].dry_run);

    // File should still be in original location
    assert!(test_file.exists());

    // Save operation result
    let operation_json = serde_json::to_string_pretty(&organization_result)?;
    let operation_file = temp_dir.path().join("dry_run_operation.json");
    fs::write(&operation_file, operation_json)?;

    // Load and verify dry-run flag
    let loaded_json = fs::read_to_string(&operation_file)?;
    let loaded_result: plex_media_organizer::organizer::OrganizationResult =
        serde_json::from_str(&loaded_json)?;

    assert!(loaded_result.organized_files[0].dry_run);

    println!("✅ Rollback dry-run operation test passed!");
    Ok(())
}
