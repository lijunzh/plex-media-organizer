use plex_media_organizer::{MovieParser, filename_parser::FilenameParser};

#[test]
fn test_debug_chinese_parsing() {
    let parser = MovieParser::new(None);
    let test_cases = vec![
        "[金手指].The.Goldfinger.2023.2160p.60fps.WEB-DL.HEVC.10bit.DDP5.1.6Audios-QHstudIo.mp4",
        "武状元苏乞儿.King.of.Beggars.1992.2160p.WEB-DL.H264.AAC.2Audio-OurTV.mp4",
        "Hello.Mr.Billionaire.2018.720p.BluRay.x264-Tv21.mkv",
        "逃学威龙.Fight.Back.to.School.1991.Bluray.1080p.x265.AAC(5.1).2Audios.GREENOTEA.mkv",
        "白蛇2：青蛇劫起..Green.Snake.2021.1080p.WEB-DL.H264.DDP5.1.Atmos-OurTV.mkv",
        "[千王之王2000].The.Tricky.Master.1999.DVDRip.X264.AC3.CD1-tdw9430.avi",
        "一步之遥.Gone.With.The.Bullets.2014.2160p.WEB-DL.H265.10bit.HDR.AAC-PTerWEB.mp4",
        "银河写手.Galaxy.Writer.2024.1080p.WEB-DL.H264.AAC-HHWEB.mkv",
        "前任4.英年早婚.The.Ex-Files.4.Marriage.Plan.2023.120FPS.2160p.WEB-DL.H265.10bit.DDP5.1-OurTV.mkv",
        "二手杰作.World's.Greatest.Dad.2023.1080p.WEB-DL.H264.AAC-GPTHD.mp4",
    ];

    for filename in test_cases {
        println!("🔍 Testing: {}", filename);

        let filename_parser = FilenameParser::new();
        match filename_parser.parse(filename) {
            Ok(components) => {
                println!("📋 Filename Parser Results:");
                println!("  Title: '{}'", components.title);
                println!("  Original Title: '{:?}'", components.original_title);
                println!("  Year: {:?}", components.year);
                println!("  Quality: '{:?}'", components.quality);
                println!("  Source: '{:?}'", components.source);
                println!("  Language: '{:?}'", components.language);
                println!("  Confidence: {:.2}", components.confidence);
            }
            Err(e) => {
                println!("❌ Filename Parser Error: {}", e);
            }
        }

        match parser.parse_filename(filename) {
            Ok(result) => {
                println!("✅ Movie Parser Results:");
                println!("  Title: '{}'", result.title);
                println!("  Original Title: '{:?}'", result.original_title);
                println!("  Year: {:?}", result.year);
                println!("  Quality: {:?}", result.quality);
                println!("  Source: {:?}", result.source);
                println!("  Language: '{:?}'", result.language);

                // Check if original title has Chinese characters
                if let Some(ref original_title) = result.original_title {
                    if !contains_chinese_characters(original_title) {
                        println!(
                            "⚠️  WARNING: Original title has no Chinese characters: '{}'",
                            original_title
                        );
                    }
                }
            }
            Err(e) => {
                println!("❌ Movie Parser Error: {}", e);
            }
        }
        println!("{}", "-".repeat(80));
    }
}

#[test]
fn test_debug_chinese_parsing_with_tmdb_simulation() {
    let parser = MovieParser::new(None);
    let test_cases = vec![
        "[金手指].The.Goldfinger.2023.2160p.60fps.WEB-DL.HEVC.10bit.DDP5.1.6Audios-QHstudIo.mp4",
        "武状元苏乞儿.King.of.Beggars.1992.2160p.WEB-DL.H264.AAC.2Audio-OurTV.mp4",
    ];

    for filename in test_cases {
        println!("🔍 Testing with TMDB simulation: {}", filename);

        // Parse filename first
        let base_info = match parser.parse_filename(filename) {
            Ok(info) => info,
            Err(e) => {
                println!("❌ Filename parsing error: {}", e);
                continue;
            }
        };

        println!("📋 Base info from filename:");
        println!("  Title: '{}'", base_info.title);
        println!("  Original Title: '{:?}'", base_info.original_title);
        println!("  Original Language: '{:?}'", base_info.original_language);

        // Simulate TMDB response (this is what we think TMDB is returning)
        let tmdb_info = plex_media_organizer::types::MovieInfo {
            title: "The Goldfinger".to_string(), // English title from TMDB
            original_title: Some("The Goldfinger".to_string()), // TMDB might return English as original
            original_language: Some("en".to_string()), // This is the problem - TMDB says original language is English
            year: base_info.year,
            part_number: None,
            is_collection: false,
            collection_name: None,
            quality: None,
            source: None,
            language: Some("en".to_string()),
        };

        println!("🎬 Simulated TMDB info:");
        println!("  Title: '{}'", tmdb_info.title);
        println!("  Original Title: '{:?}'", tmdb_info.original_title);
        println!("  Original Language: '{:?}'", tmdb_info.original_language);

        // Since merge_movie_info is private, let's simulate what happens
        // The issue is in the merge logic when TMDB says original_language is "en"
        println!("🔄 Simulating merge logic:");

        // This is what the merge logic does when original_language is "en"
        let tmdb_original_language = &tmdb_info.original_language;
        match tmdb_original_language.as_deref() {
            Some("ja") | Some("zh") | Some("ko") => {
                println!("  TMDB says original language is CJK - would preserve Chinese title");
            }
            Some("en") => {
                println!(
                    "  TMDB says original language is English - this overwrites Chinese title!"
                );
                println!("  This is the problem - TMDB is wrong about the original language");
            }
            _ => {
                println!("  TMDB says original language is other - falls back to filename logic");
            }
        }

        // Check if Chinese title was preserved
        if let Some(ref original_title) = base_info.original_title {
            if contains_chinese_characters(original_title) {
                println!("✅ Chinese title from filename: '{}'", original_title);
                println!("❌ But TMDB says original language is English, so it gets overwritten!");
            } else {
                println!(
                    "❌ Chinese title lost! Original title is now: '{}'",
                    original_title
                );
            }
        } else {
            println!("❌ No original title in base info!");
        }

        println!("{}", "-".repeat(80));
    }
}

#[test]
fn test_debug_specific_problematic_cases() {
    let parser = MovieParser::new(None);
    let test_cases = vec![
        "[金手指].The.Goldfinger.2023.2160p.60fps.WEB-DL.HEVC.10bit.DDP5.1.6Audios-QHstudIo.mp4",
        "武状元苏乞儿.King.of.Beggars.1992.2160p.WEB-DL.H264.AAC.2Audio-OurTV.mp4",
    ];

    for filename in test_cases {
        println!("🔍 Testing specific problematic case: {}", filename);

        // Parse filename first
        let base_info = match parser.parse_filename(filename) {
            Ok(info) => info,
            Err(e) => {
                println!("❌ Filename parsing error: {}", e);
                continue;
            }
        };

        println!("📋 Base info from filename:");
        println!("  Title: '{}'", base_info.title);
        println!("  Original Title: '{:?}'", base_info.original_title);
        println!("  Original Language: '{:?}'", base_info.original_language);

        // Simulate different TMDB responses to see what might be happening
        let tmdb_scenarios = vec![
            (
                "Scenario 1: TMDB says original_language is 'en'",
                Some("en".to_string()),
            ),
            (
                "Scenario 2: TMDB says original_language is 'zh'",
                Some("zh".to_string()),
            ),
            ("Scenario 3: TMDB says original_language is None", None),
        ];

        for (scenario_name, tmdb_original_language) in tmdb_scenarios {
            println!("🎬 {}", scenario_name);

            let tmdb_info = plex_media_organizer::types::MovieInfo {
                title: "The Goldfinger".to_string(),
                original_title: Some("The Goldfinger".to_string()),
                original_language: tmdb_original_language.clone(),
                year: base_info.year,
                part_number: None,
                is_collection: false,
                collection_name: None,
                quality: None,
                source: None,
                language: Some("en".to_string()),
            };

            println!(
                "  TMDB Original Language: '{:?}'",
                tmdb_info.original_language
            );

            // Simulate the merge logic
            let has_cjk_in_filename = base_info
                .original_title
                .as_ref()
                .map(|title| contains_chinese_characters(title))
                .unwrap_or(false);

            match tmdb_original_language.as_deref() {
                Some("ja") | Some("zh") | Some("ko") => {
                    println!("  → TMDB says CJK - would use TMDB's original title");
                }
                Some("en") => {
                    if has_cjk_in_filename {
                        println!(
                            "  → TMDB says English but filename has CJK - would use filename's Chinese title"
                        );
                    } else {
                        println!(
                            "  → TMDB says English and no CJK in filename - would use TMDB's English title"
                        );
                    }
                }
                None => {
                    if has_cjk_in_filename {
                        println!(
                            "  → TMDB has no language info but filename has CJK - would use filename's Chinese title"
                        );
                    } else {
                        println!(
                            "  → TMDB has no language info and no CJK in filename - would use filename's title"
                        );
                    }
                }
                _ => {
                    println!("  → TMDB says other language - would use TMDB's English title");
                }
            }
        }

        println!("{}", "-".repeat(80));
    }
}

fn contains_chinese_characters(text: &str) -> bool {
    text.chars().any(|c| {
        matches!(c,
            '\u{4e00}'..='\u{9fff}' | // CJK Unified Ideographs
            '\u{3400}'..='\u{4dbf}' | // CJK Unified Ideographs Extension A
            '\u{20000}'..='\u{2a6df}' | // CJK Unified Ideographs Extension B
            '\u{2a700}'..='\u{2b73f}' | // CJK Unified Ideographs Extension C
            '\u{2b740}'..='\u{2b81f}' | // CJK Unified Ideographs Extension D
            '\u{2b820}'..='\u{2ceaf}' | // CJK Unified Ideographs Extension E
            '\u{f900}'..='\u{faff}' | // CJK Compatibility Ideographs
            '\u{3300}'..='\u{33ff}' | // CJK Compatibility
            '\u{fe30}'..='\u{fe4f}' | // CJK Compatibility Forms
            '\u{ff00}'..='\u{ffef}'   // Halfwidth and Fullwidth Forms
        )
    })
}
