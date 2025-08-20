use plex_media_organizer::{filename_parser::FilenameParser, tmdb_client::TmdbClient};

#[tokio::test]
async fn test_current_issues_analysis() {
    // Load TMDB API key from environment
    let api_key =
        std::env::var("TMDB_API_KEY").expect("TMDB_API_KEY environment variable required");
    let tmdb_client = TmdbClient::new(api_key);
    let parser = FilenameParser::new();

    println!("🔍 Analyzing current issues with skipped movies and incorrect renames...\n");

    // Test cases from the user's output - movies that were skipped
    let skipped_movies = vec![
        "Love.and.Other.Drugs.2010.BluRay.720p.DTS.x264-CHD.mkv",
        "chd-sleepy-hollow-dts-bdrip.mkv",
        "Transformers Dark of the Moon 2011 BluRay 2160p TrueHD 7.1 Atmos x265 10bit-CHD.mkv",
        "Transformers 2007 BluRay 2160p TrueHD 7.1 Atmos x265 10bit-CHD.mkv",
        "3.Idiots.2009.BluRay.1080p.DTS.x264-LolHD.mkv",
        "Transformers Revenge of the Fallen 2009 BluRay 2160p TrueHD 7.1 Atmos x265 10bit-CHD.mkv",
        "Transformers The Last Knight 2017 BluRay 2160p TrueHD 7.1 Atmos x265 10bit-CHD.mkv",
        "Transformers Age of Extinction 2014 BluRay 2160p TrueHD 7.1 Atmos x265 10bit-CHD.mkv",
        "[DBD-Raws][4K_HDR][ready.player.one][2160P][BDRip][HEVC-10bit][FLAC].mkv",
        "A.I.Artificial.Intelligence.2001.1080p.BluRay.x264-EbP.mkv",
        "Les Misérables.mkv",
        "Poirot.Murder.on.the.Orient.Express.2010.720p.BluRay.x264-DON.mkv",
        "Pearl.Harbor.2001.1080p.Bluray.DTS.x264-D-Z0N3.mkv",
        "The.Beasts.(2022).1080p.BluRay.DD+5.1.x264-DON.mkv",
        "Star Trek - Jerome Bixby's Sci Fi Legacy.mkv",
        "The Man from Earth - Legacy.mkv",
        "The Story of the Story.mkv",
        "Free.Guy.2021.2160p.4K.WEB.x265.10bit.AAC5.1-[YTS.MX].mkv",
        "The Man from Earth (2007) (1080p BluRay x265 Silence).mkv",
        "The.Lord.of.the.Rings.The.Fellowship.of.the.Ring.2001.Extended.UHD.BluRay.2160p.Atmos.TrueHD.7.1.HDR.x265.10bit-CHD.mkv",
        "The.Lord.of.the.Rings.The.Two.Towers.2002.Extended.UHD.BluRay.2160p.Atmos.TrueHD.7.1.HDR.x265.10bit-CHD.mkv",
        "The.Lord.of.the.Rings.The.Return.of.the.King.2003.Extended.UHD.BluRay.2160p.Atmos.TrueHD.7.1.HDR.x265.10bit-CHD.mkv",
    ];

    // Test cases for movies that were incorrectly renamed
    let incorrectly_renamed = vec![
        (
            "Barbie.2023.2160p.WEB-DL.DDP5.1.Atmos.DV.HDR.H.265-MZABARBiE.mkv",
            "Barbie and Me (2023)",
        ),
        (
            "CODA.2021.1080p.BluRay.x264.DTS-WiKi.mkv",
            "Artists In Agony Hitmen at the Coda Teahouse (2021)",
        ),
        (
            "Joker.2019.2160p.UHD.BluRay.REMUX.HEVC.TrueHD.7.1.Atmos-PTHOME.mkv",
            "Joker Rising 2 The Clown Prince (2019)",
        ),
        (
            "Klaus.2019.Netflix.WEB-DL.1080p.HEVC.DDP-AREY.mkv",
            "Klaus Doldinger's Passport - Live in Leverkusen 2019 (2019)",
        ),
        (
            "Jurassic.World.Rebirth.2025.MULTi.1080p.HDTS-SyncUP.mkv",
            "Jurassic World Rebirth The Making of a New Era (2025)",
        ),
        (
            "The.Intern.2015.1080p.WEB-DL.x264.AC3-JYK.mkv",
            "The Internship Games (2015)",
        ),
        (
            "Apples.2020.1080p.BluRay.x264.DDP5.1-BYRHD.mkv",
            "Crab Apples (2020)",
        ),
        (
            "The.Killer.2023.1080p.NF.WEB-DL.DDP5.1.H.264-playWEB.mkv",
            "The Killers Reading Festival 2023 (2023)",
        ),
        (
            "Moon.2009.1080p.BluRay.DD5.1.x264-RightSiZE.mkv",
            "The Twilight Saga New Moon (2009)",
        ),
        (
            "Jurassic.Park.III.2001.1080p.BluRay.x264.DTS-HDChina.mkv",
            "Jurassic Park III A Visit To ILM (2001)",
        ),
        (
            "The.Avengers.2012.Bluray.2160p.x265.10bit.HDR.3Audio.mUHD-FRDS.mkv",
            "The Avengers A Visual Journey (2012)",
        ),
        (
            "Jurassic.Park.1993.1080P.BluRay.x264.DTS-HDChina.mkv",
            "The Real Jurassic Park (1993)",
        ),
        (
            "Jurassic.Park.Ultimate.Trilogy.1993-2001.1080p.BluRay.x264.DTS-HDChina/Jurassic.Park.III.2001.1080p.BluRay.x264.DTS-HDChina/Jurassic.Park.III.2001.1080p.BluRay.x264.DTS-HDChina.mkv",
            "Jurassic Park III A Visit To ILM (2001)",
        ),
        (
            "Jurassic.Park.Ultimate.Trilogy.1993-2001.1080p.BluRay.x264.DTS-HDChina/Jurassic.Park.1993.1080P.BluRay.x264.DTS-HDChina/Jurassic.Park.1993.1080P.BluRay.x264.DTS-HDChina.mkv",
            "The Real Jurassic Park (1993)",
        ),
        (
            "The.Man.From.Earth.2007.BluRay.iPad.720p.x264.AAC-BYRPAD.mp4",
            "The Man from Earth (2007)",
        ),
        (
            "Star.Wars.Episode.IV.A.New.Hope.1977.2160p.UHD.BluRay.HDR.x265.Atmos.TrueHD.7.1-HDChina.mkv",
            "Star Wars (1977)",
        ),
        (
            "The.Man.From.Earth.2007.BluRay.iPad.720p.x264.AAC-BYRPAD.mp4",
            "The Man from Earth (2007)",
        ),
        (
            "The.Dark.Knight.2008.2160p.UHD.BluRay.X265-IAMABLE.mkv",
            "Batman Unmasked The Psychology of The Dark Knight (2008)",
        ),
        (
            "Moneyball.2011.UHD.2160p.WEB-Rip.DDP.5.1.HEVC-DDR[EtHD].mkv",
            "The Bad Guys 2 (2025)",
        ),
        (
            "Her.2013.1080p.BluRay.DTS.x264-DON.mkv",
            "Her Husband's Betrayal (2013)",
        ),
        (
            "Forrest.Gump.1994.720p.BluRay.DD5.1.x264-LoRD.mkv",
            "Through the Eyes of Forrest Gump (1994)",
        ),
        (
            "The.Great.Gatsby.2013.BluRay.1080p.x265.10bit.2Audio.MNHD-FRDS.mkv",
            "The Great Gatsby Midnight in Manhattan (2000)",
        ),
        ("Warcraft.mkv", "Warcraft III Reign of Chaos (2002)"),
        (
            "Looper.2012.1080p.BluRay.x264.DTS-WiKi.mkv",
            "The Outsider A Looper's Story (2012)",
        ),
        (
            "Frequency.2000.1080p.BluRay.DTS.x264-FANDANGO.mkv",
            "99.9 la frecuencia del terror [99.9 The Frequency of Terror] (1997)",
        ),
    ];

    // Test cases for movies with missing years
    let missing_year = vec!["I, Robot.mkv"];

    println!("📁 ANALYZING SKIPPED MOVIES:");
    println!("{}", "=".repeat(50));

    for filename in &skipped_movies {
        println!("\n📁 Testing: {}", filename);

        // Parse the filename
        let parsed = parser.parse(filename).unwrap();
        println!("   📝 Parsed title: '{}'", parsed.title);
        println!("   📅 Parsed year: {:?}", parsed.year);

        // Try TMDB search with confidence score
        let search_result = tmdb_client
            .find_best_match_with_score(&parsed.title, parsed.year)
            .await;

        match search_result {
            Ok(Some(match_result)) => {
                let best_match = &match_result.movie;
                println!("   ✅ TMDB Match Found:");
                println!("      🎬 Title: '{}'", best_match.title);
                println!(
                    "      🌍 Original: '{}'",
                    best_match.original_title.as_deref().unwrap_or("N/A")
                );
                println!(
                    "      📅 Release: {}",
                    best_match.release_date.as_deref().unwrap_or("N/A")
                );
                println!("      📊 Confidence: {:.3}", match_result.confidence_score);

                if match_result.confidence_score >= 0.7 {
                    println!("      ✅ ABOVE THRESHOLD (0.7) - Would be organized");
                } else {
                    println!("      ❌ BELOW THRESHOLD (0.7) - Would be skipped");
                }
            }
            _ => {
                println!("   ❌ No TMDB match found - Would be skipped");
            }
        }
    }

    println!("\n\n📁 ANALYZING INCORRECTLY RENAMED MOVIES:");
    println!("{}", "=".repeat(50));

    for (filename, incorrect_title) in &incorrectly_renamed {
        println!("\n📁 Testing: {}", filename);

        // Parse the filename
        let parsed = parser.parse(filename).unwrap();
        println!("   📝 Parsed title: '{}'", parsed.title);
        println!("   📅 Parsed year: {:?}", parsed.year);

        // Try TMDB search with confidence score
        let search_result = tmdb_client
            .find_best_match_with_score(&parsed.title, parsed.year)
            .await;

        match search_result {
            Ok(Some(match_result)) => {
                let best_match = &match_result.movie;
                println!("   ✅ TMDB Match Found:");
                println!("      🎬 Title: '{}'", best_match.title);
                println!(
                    "      🌍 Original: '{}'",
                    best_match.original_title.as_deref().unwrap_or("N/A")
                );
                println!(
                    "      📅 Release: {}",
                    best_match.release_date.as_deref().unwrap_or("N/A")
                );
                println!("      📊 Confidence: {:.3}", match_result.confidence_score);
                println!(
                    "      ❌ INCORRECT RENAME: Expected correct title, got '{}'",
                    incorrect_title
                );
            }
            _ => {
                println!("   ❌ No TMDB match found");
            }
        }
    }

    println!("\n\n📁 ANALYZING MOVIES WITH MISSING YEARS:");
    println!("{}", "=".repeat(50));

    for filename in &missing_year {
        println!("\n📁 Testing: {}", filename);

        // Parse the filename
        let parsed = parser.parse(filename).unwrap();
        println!("   📝 Parsed title: '{}'", parsed.title);
        println!("   📅 Parsed year: {:?}", parsed.year);

        if parsed.year.is_none() {
            println!("   ❌ NO YEAR PARSED - Will be organized to 'Unknown Year'");

            // Try TMDB search without year
            let search_result = tmdb_client
                .find_best_match_with_score(&parsed.title, None)
                .await;

            match search_result {
                Ok(Some(match_result)) => {
                    let best_match = &match_result.movie;
                    println!("   ✅ TMDB Match Found (without year):");
                    println!("      🎬 Title: '{}'", best_match.title);
                    println!(
                        "      🌍 Original: '{}'",
                        best_match.original_title.as_deref().unwrap_or("N/A")
                    );
                    println!(
                        "      📅 Release: {}",
                        best_match.release_date.as_deref().unwrap_or("N/A")
                    );
                    println!("      📊 Confidence: {:.3}", match_result.confidence_score);
                }
                _ => {
                    println!("   ❌ No TMDB match found even without year");
                }
            }
        }
    }

    println!("\n\n🔍 SUMMARY OF ISSUES:");
    println!("{}", "=".repeat(50));
    println!(
        "1. SKIPPED MOVIES: {} movies were skipped",
        skipped_movies.len()
    );
    println!(
        "2. INCORRECT RENAMES: {} movies were incorrectly renamed",
        incorrectly_renamed.len()
    );
    println!(
        "3. MISSING YEARS: {} movies have missing years",
        missing_year.len()
    );
    println!("\n💡 RECOMMENDATIONS:");
    println!("- Review technical terms filtering for better title extraction");
    println!("- Improve TMDB search accuracy to avoid incorrect matches");
    println!("- Add better year extraction for movies without explicit years");
}
