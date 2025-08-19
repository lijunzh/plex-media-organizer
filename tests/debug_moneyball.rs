use plex_media_organizer::{MovieParser, filename_parser::FilenameParser};

#[test]
fn test_debug_moneyball_parsing() {
    let parser = MovieParser::new(None);
    let filename = "Moneyball.2011.UHD.2160p.WEB-Rip.DDP.5.1.HEVC-DDR[EtHD].mkv";

    println!("🔍 Debugging filename: {}", filename);

    // Test the filename parser directly
    let filename_parser = FilenameParser::new();

    match filename_parser.parse(filename) {
        Ok(components) => {
            println!("📋 Filename Parser Results:");
            println!("  Title: '{}'", components.title);
            println!("  Original Title: '{:?}'", components.original_title);
            println!("  Year: {:?}", components.year);
            println!("  Quality: '{:?}'", components.quality);
            println!("  Source: '{:?}'", components.source);
            println!("  Audio: '{:?}'", components.audio);
            println!("  Codec: '{:?}'", components.codec);
            println!("  Group: '{:?}'", components.group);
            println!("  Language: '{:?}'", components.language);
            println!("  Confidence: {:.2}", components.confidence);
        }
        Err(e) => {
            println!("❌ Filename parser failed: {}", e);
        }
    }

    match parser.parse_filename(filename) {
        Ok(result) => {
            println!("\n✅ Movie Parser Results:");
            println!("Title: '{}'", result.title);
            println!("Original Title: '{:?}'", result.original_title);
            println!("Year: {:?}", result.year);
            println!("Quality: {:?}", result.quality);
            println!("Source: {:?}", result.source);
            println!("Language: {:?}", result.language);

            // Check if title is empty or very short
            if result.title.is_empty() {
                println!("❌ ERROR: Empty title!");
            } else if result.title.len() < 5 {
                println!("⚠️  WARNING: Very short title: '{}'", result.title);
            }

            // Check if title contains technical terms
            let technical_terms = [
                "DDP", "DTS", "AC3", "H264", "H265", "x264", "x265", "WEB-Rip", "BluRay", "REMUX",
                "EtHD",
            ];
            for term in &technical_terms {
                if result.title.to_uppercase().contains(term) {
                    println!(
                        "⚠️  WARNING: Title contains technical term '{}': '{}'",
                        term, result.title
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ Parsing failed: {}", e);
        }
    }
}

#[test]
fn test_debug_matrix_parsing() {
    let parser = MovieParser::new(None);
    let filename = "The Matrix.mkv";

    println!("🔍 Debugging Matrix filename: {}", filename);

    // Test the filename parser directly
    let filename_parser = FilenameParser::new();

    match filename_parser.parse(filename) {
        Ok(components) => {
            println!("📋 Filename Parser Results:");
            println!("  Title: '{}'", components.title);
            println!("  Original Title: '{:?}'", components.original_title);
            println!("  Year: {:?}", components.year);
            println!("  Quality: '{:?}'", components.quality);
            println!("  Source: '{:?}'", components.source);
            println!("  Audio: '{:?}'", components.audio);
            println!("  Codec: '{:?}'", components.codec);
            println!("  Group: '{:?}'", components.group);
            println!("  Language: '{:?}'", components.language);
            println!("  Confidence: {:.2}", components.confidence);
        }
        Err(e) => {
            println!("❌ Filename parser failed: {}", e);
        }
    }

    match parser.parse_filename(filename) {
        Ok(result) => {
            println!("\n✅ Movie Parser Results:");
            println!("Title: '{}'", result.title);
            println!("Original Title: '{:?}'", result.original_title);
            println!("Year: {:?}", result.year);
            println!("Quality: {:?}", result.quality);
            println!("Source: {:?}", result.source);
            println!("Language: {:?}", result.language);
        }
        Err(e) => {
            println!("❌ Parsing failed: {}", e);
        }
    }
}
