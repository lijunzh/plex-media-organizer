use plex_media_organizer::{filename_parser::FilenameParser, tmdb_client::TmdbClient};

#[tokio::test]
async fn test_i_robot_debug() {
    // Load TMDB API key from environment
    let api_key =
        std::env::var("TMDB_API_KEY").expect("TMDB_API_KEY environment variable required");
    let tmdb_client = TmdbClient::new(api_key);
    let parser = FilenameParser::new();

    // Clear cache to ensure fresh results
    tmdb_client.clear_cache();

    let filename = "I, Robot.mkv";

    println!("🔍 Debugging: {}", filename);

    // Parse filename
    let components = parser.parse(filename);
    match components {
        Ok(parsed) => {
            println!("   📝 Parsed title: '{}'", parsed.title);
            println!("   📅 Parsed year: {:?}", parsed.year);
            println!("   📊 Confidence: {:.3}", parsed.confidence);

            // Test individual search strategies
            println!("\n   🔍 Testing individual search strategies...");

            // Strategy 1: Search without year
            println!("   📋 Strategy 1: Search without year");
            let result_no_year = tmdb_client
                .find_best_match_with_score(&parsed.title, None)
                .await
                .unwrap();
            if let Some(result) = result_no_year {
                println!(
                    "      Found: '{}' (Release: {:?}, Confidence: {:.3})",
                    result.movie.title, result.movie.release_date, result.confidence_score
                );
            } else {
                println!("      No match found");
            }

            // Strategy 2: Search with year 2004
            println!("   📋 Strategy 2: Search with year 2004");
            let result_2004 = tmdb_client
                .find_best_match_with_score(&parsed.title, Some(2004))
                .await
                .unwrap();
            if let Some(result) = result_2004 {
                println!(
                    "      Found: '{}' (Release: {:?}, Confidence: {:.3})",
                    result.movie.title, result.movie.release_date, result.confidence_score
                );
            } else {
                println!("      No match found");
            }

            // Enhanced search
            println!("\n   🔍 Enhanced TMDB search (will try multiple years if needed)...");
            let search_result = tmdb_client
                .enhanced_search(&parsed.title, parsed.year)
                .await;
            match search_result {
                Ok(Some(tmdb_result)) => {
                    println!("   ✅ TMDB Match Found:");
                    println!("      🎬 Title: '{}'", tmdb_result.movie.title);
                    if let Some(original) = &tmdb_result.movie.original_title {
                        println!("      🌍 Original: '{}'", original);
                    }
                    if let Some(date) = &tmdb_result.movie.release_date {
                        println!("      📅 Release: {}", date);
                        // Extract year from release date
                        if let Some(year) =
                            date.split('-').next().and_then(|y| y.parse::<u32>().ok())
                        {
                            println!("      📅 Extracted Year: {}", year);
                        }
                    }
                    println!("      📊 Confidence: {:.3}", tmdb_result.confidence_score);
                }
                Ok(None) => {
                    println!("   ❌ No TMDB match found");
                }
                Err(e) => {
                    println!("   💥 TMDB search error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("   💥 Parse error: {}", e);
        }
    }
}
