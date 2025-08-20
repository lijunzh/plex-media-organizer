use plex_media_organizer::config::AppConfig;
use plex_media_organizer::filename_parser::FilenameParser;

#[test]
fn test_parsing_detailed() {
    println!("🔍 DEBUGGING PARSING IN DETAIL");
    println!("===============================");

    // Load configuration
    let config = AppConfig::load().expect("Failed to load config");
    let parser = FilenameParser::with_technical_terms(config.get_all_technical_terms());

    // Test a specific problematic filename
    let test_filename = "The.Lord.of.the.Rings.The.Return.of.the.King.2003.Extended.UHD.BluRay.2160p.Atmos.TrueHD.7.1.HDR.x265.10bit-CHD.mkv";

    println!("📁 Testing filename: {}", test_filename);
    println!("");

    // Parse the filename
    match parser.parse(test_filename) {
        Ok(components) => {
            println!("✅ Parsed successfully:");
            println!("   Title: '{}'", components.title);
            println!("   Year: {:?}", components.year);
            println!("   Quality: {:?}", components.quality);
            println!("   Source: {:?}", components.source);
            println!("   Language: {:?}", components.language);
            println!("");

            // Check if "7 1" is in the title
            if components.title.contains("7 1") {
                println!("❌ PROBLEM: '7 1' is still in the title!");
                println!("   This should have been filtered out as a technical term.");
            } else {
                println!("✅ SUCCESS: '7 1' was correctly filtered out");
            }

            // Check if "CHD" is in the title
            if components.title.contains("CHD") {
                println!("❌ PROBLEM: 'CHD' is still in the title!");
                println!("   This should have been filtered out as a technical term.");
            } else {
                println!("✅ SUCCESS: 'CHD' was correctly filtered out");
            }
        }
        Err(e) => {
            println!("❌ Failed to parse: {}", e);
        }
    }

    println!("");
    println!("🔍 Testing individual token filtering:");
    println!("=====================================");

    // Test individual tokens that should be filtered out
    let test_tokens = ["7", "1", "7.1", "7 1", "CHD", "terminal", "LolHD"];

    for token in &test_tokens {
        // This is a simplified test - we can't directly access the is_metadata_token method
        // but we can check if the token appears in a parsed title
        let test_filename = format!("Test.Movie.{}", token);
        match parser.parse(&test_filename) {
            Ok(components) => {
                if components.title.contains(token) {
                    println!("❌ Token '{}' was NOT filtered out", token);
                } else {
                    println!("✅ Token '{}' was filtered out", token);
                }
            }
            Err(_) => {
                println!("⚠️  Could not test token '{}'", token);
            }
        }
    }
}
