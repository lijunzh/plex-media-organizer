use plex_media_organizer::{
    core::{MediaProcessor, MediaScanner, MediaOrganizer},
    parsers::MovieParser,
    config::AppConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing refactored Plex Media Organizer...");
    
    // Test creating components
    let config = AppConfig::default();
    let scanner = MediaScanner::new();
    let parser = MovieParser::new();
    let organizer = MediaOrganizer::new(true, None);
    
    println!("✅ Successfully created all components:");
    println!("  - Scanner: {:?}", scanner);
    println!("  - Parser: {:?}", parser);
    println!("  - Organizer: {:?}", organizer);
    
    // Test processor creation
    let processor = MediaProcessor::new()?;
    println!("✅ Successfully created processor: {:?}", processor);
    
    println!("🎉 Refactoring test completed successfully!");
    Ok(())
}