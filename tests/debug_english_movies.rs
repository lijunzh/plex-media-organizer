use chrono::Datelike;
use plex_media_organizer::movie_parser::MovieParser;
use plex_media_organizer::tmdb_client::TmdbClient;

#[tokio::test]
async fn test_debug_english_movies_skipped() {
    println!("🔍 DEBUGGING ENGLISH MOVIES BEING SKIPPED");
    println!("=========================================");

    let tmdb_client = TmdbClient::new("900cb3e0c3744c61fa7158686f1339d2".to_string());
    let movie_parser = MovieParser::new(Some(tmdb_client.clone()));

    let problematic_movies = [
        "The.Lord.of.the.Rings.The.Return.of.the.King.2003.Extended.UHD.BluRay.2160p.Atmos.TrueHD.7.1.HDR.x265.10bit-CHD.mkv",
        "Love.and.Other.Drugs.2010.BluRay.720p.DTS.x264-CHD.mkv",
        "The.Lord.of.the.Rings.The.Fellowship.of.the.Ring.2001.Extended.UHD.BluRay.2160p.Atmos.TrueHD.7.1.HDR.x265.10bit-CHD.mkv",
        "The.Lord.of.the.Rings.The.Two.Towers.2002.Extended.UHD.BluRay.2160p.Atmos.TrueHD.7.1.HDR.x265.10bit-CHD.mkv",
        "the.dark.knight.rises.2012.rerip.2160p.uhd.bluray.x265-terminal.mkv",
        "batman.begins.2005.2160p.uhd.bluray.x265-terminal.mkv",
        "chd-sleepy-hollow-dts-bdrip.mkv",
        "Transformers Age of Extinction 2014 BluRay 2160p TrueHD 7.1 Atmos x265 10bit-CHD.mkv",
        "Transformers The Last Knight 2017 BluRay 2160p TrueHD 7.1 Atmos x265 10bit-CHD.mkv",
        "Transformers Dark of the Moon 2011 BluRay 2160p TrueHD 7.1 Atmos x265 10bit-CHD.mkv",
        "3.Idiots.2009.BluRay.1080p.DTS.x264-LolHD.mkv",
        "Transformers 2007 BluRay 2160p TrueHD 7.1 Atmos x265 10bit-CHD.mkv",
        "[DBD-Raws][4K_HDR][ready.player.one][2160P][BDRip][HEVC-10bit][FLAC].mkv",
        "Transformers Revenge of the Fallen 2009 BluRay 2160p TrueHD 7.1 Atmos x265 10bit-CHD.mkv",
        "A.I.Artificial.Intelligence.2001.1080p.BluRay.x264-EbP.mkv",
        "Poirot.Murder.on.the.Orient.Express.2010.720p.BluRay.x264-DON.mkv",
        "Pearl.Harbor.2001.1080p.Bluray.DTS.x264-D-Z0N3.mkv",
        "The.Beasts.(2022).1080p.BluRay.DD+5.1.x264-DON.mkv",
        "Les Misérables.mkv",
        "Free.Guy.2021.2160p.4K.WEB.x265.10bit.AAC5.1-[YTS.MX].mkv",
        "The Man from Earth (2007) (1080p BluRay x265 Silence).mkv",
        "The Witch  Part 1. The Subversion 2018 1080p WEB-DL  H264 AAC-KBTV.mp4",
    ];

    let mut total_tested = 0;
    let mut total_failed = 0;
    let mut total_succeeded = 0;

    for filename in &problematic_movies {
        total_tested += 1;
        println!("\n📁 Testing: {}", filename);
        println!("{}", "─".repeat(80));

        // Test 1: Parse filename
        match movie_parser.parse_filename(filename) {
            Ok(parsed) => {
                println!("✅ Parsed successfully:");
                println!("   Title: '{}'", parsed.title);
                println!("   Year: {:?}", parsed.year);
                println!("   Quality: {:?}", parsed.quality);
                println!("   Source: {:?}", parsed.source);
                println!("   Language: {:?}", parsed.language);

                // Test 2: TMDB search
                let search_result = tmdb_client
                    .find_best_match_with_score(&parsed.title, parsed.year)
                    .await;
                match search_result {
                    Ok(Some(match_result)) => {
                        println!("✅ TMDB Match found:");
                        println!("   TMDB Title: '{}'", match_result.movie.title);
                        println!(
                            "   Original Title: '{:?}'",
                            match_result.movie.original_title
                        );
                        if let Some(release_date) = &match_result.movie.release_date {
                            // Parse the date string to get year
                            if let Ok(date) =
                                chrono::NaiveDate::parse_from_str(release_date, "%Y-%m-%d")
                            {
                                println!("   Year: {}", date.year());
                            } else {
                                println!(
                                    "   Year: Unknown (could not parse date: {})",
                                    release_date
                                );
                            }
                        } else {
                            println!("   Year: Unknown");
                        }
                        println!("   Confidence: {:.2}", match_result.confidence_score);
                        total_succeeded += 1;
                    }
                    Ok(None) => {
                        println!("❌ No TMDB match found!");
                        println!(
                            "   Searched for: '{}' (year: {:?})",
                            parsed.title, parsed.year
                        );

                        // Let's try a direct TMDB search to see what's happening
                        let search_results =
                            tmdb_client.search_movie(&parsed.title, parsed.year).await;
                        match search_results {
                            Ok(results) => {
                                if results.is_empty() {
                                    println!("   Direct search returned 0 results");
                                } else {
                                    println!(
                                        "   Direct search returned {} results:",
                                        results.len()
                                    );
                                    for (i, result) in results.iter().take(3).enumerate() {
                                        let year = if let Some(release_date) = &result.release_date
                                        {
                                            if let Ok(date) = chrono::NaiveDate::parse_from_str(
                                                release_date,
                                                "%Y-%m-%d",
                                            ) {
                                                date.year().to_string()
                                            } else {
                                                "Unknown".to_string()
                                            }
                                        } else {
                                            "Unknown".to_string()
                                        };
                                        println!(
                                            "     {}. '{}' ({}) - {:?}",
                                            i + 1,
                                            result.title,
                                            year,
                                            result.original_title
                                        );
                                    }
                                }
                            }
                            Err(e) => {
                                println!("   Direct search error: {}", e);
                            }
                        }
                        total_failed += 1;
                    }
                    Err(e) => {
                        println!("❌ TMDB search error: {}", e);
                        total_failed += 1;
                    }
                }
            }
            Err(e) => {
                println!("❌ Failed to parse filename: {}", e);
                total_failed += 1;
            }
        }
    }

    println!("\n{}", "=".repeat(80));
    println!("📊 SUMMARY:");
    println!("   Total tested: {}", total_tested);
    println!("   Succeeded: {}", total_succeeded);
    println!("   Failed: {}", total_failed);
    println!(
        "   Success rate: {:.1}%",
        (total_succeeded as f32 / total_tested as f32) * 100.0
    );
    println!("{}", "=".repeat(80));
}
