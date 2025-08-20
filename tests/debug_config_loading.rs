use plex_media_organizer::config::AppConfig;
use plex_media_organizer::filename_parser::FilenameParser;

#[test]
fn test_config_loading() {
    println!("🔍 DEBUGGING CONFIG LOADING");
    println!("=============================");

    // Test loading the configuration
    match AppConfig::load() {
        Ok(config) => {
            println!("✅ Configuration loaded successfully");

            // Test getting technical terms
            let technical_terms = config.get_all_technical_terms();
            println!("📋 Total technical terms loaded: {}", technical_terms.len());

            // Check for specific terms that should be in the config
            let expected_terms = [
                "CHD", "terminal", "LolHD", "D-Z0N3", "7.1", "7 1", "DD+5.1", "DD+5 1",
            ];
            for term in &expected_terms {
                if technical_terms
                    .iter()
                    .any(|t| t.to_lowercase() == term.to_lowercase())
                {
                    println!("✅ Found term: '{}'", term);
                } else {
                    println!("❌ Missing term: '{}'", term);
                }
            }

            // Test creating filename parser with config
            let parser = FilenameParser::with_technical_terms(technical_terms);
            println!("✅ FilenameParser created successfully");
        }
        Err(e) => {
            println!("❌ Failed to load configuration: {}", e);
        }
    }
}
