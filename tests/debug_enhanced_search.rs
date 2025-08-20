use plex_media_organizer::tmdb_client::TmdbClient;

#[tokio::test]
async fn test_enhanced_search_logic() {
    // Load TMDB API key from environment
    let api_key =
        std::env::var("TMDB_API_KEY").expect("TMDB_API_KEY environment variable required");
    let tmdb_client = TmdbClient::new(api_key);

    // Clear cache to ensure fresh results
    tmdb_client.clear_cache();

    let title = "I, Robot";
    let year = None; // No year provided

    println!(
        "🔍 Testing enhanced search for '{}' with year {:?}",
        title, year
    );

    // Test individual year searches first
    println!("📋 Testing year 2004 specifically:");
    let result_2004 = tmdb_client
        .find_best_match_with_score(title, Some(2004))
        .await
        .unwrap();
    if let Some(result) = result_2004 {
        println!(
            "   Found: '{}' (Release: {:?}, Confidence: {:.3})",
            result.movie.title, result.movie.release_date, result.confidence_score
        );
    }

    println!("📋 Testing year 2021 specifically:");
    let result_2021 = tmdb_client
        .find_best_match_with_score(title, Some(2021))
        .await
        .unwrap();
    if let Some(result) = result_2021 {
        println!(
            "   Found: '{}' (Release: {:?}, Confidence: {:.3})",
            result.movie.title, result.movie.release_date, result.confidence_score
        );
    }

    // Test enhanced search
    let result = tmdb_client.enhanced_search(title, year).await.unwrap();

    match result {
        Some(tmdb_result) => {
            println!("✅ Enhanced search found:");
            println!("   🎬 Title: '{}'", tmdb_result.movie.title);
            if let Some(original) = &tmdb_result.movie.original_title {
                println!("   🌍 Original: '{}'", original);
            }
            if let Some(date) = &tmdb_result.movie.release_date {
                println!("   📅 Release: '{}'", date);
            } else {
                println!("   📅 Release: None");
            }
            println!("   📊 Confidence: {:.3}", tmdb_result.confidence_score);
        }
        None => {
            println!("❌ Enhanced search found no match");
        }
    }
}
