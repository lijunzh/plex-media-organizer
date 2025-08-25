//! Basic parsing tests for the new TMDB-first strategy

use plex_media_organizer::{AppConfig, UnifiedMovieParser};

#[tokio::test]
async fn test_basic_english_movie() {
    let config = AppConfig::default();
    let parser = UnifiedMovieParser::new(config);

    let result = parser
        .parse_async("The.Batman.2022.2160p.Remux.HEVC.DoVi.TrueHD.7.1-3L.mkv")
        .await
        .unwrap();

    println!("DEBUG: Parsed title: '{}'", result.data.title);
    assert!(!result.data.title.is_empty());
    assert_eq!(result.data.year, Some(2022));
    assert!(result.confidence > 0.0);
}

#[tokio::test]
async fn test_chinese_bilingual_movie() {
    let config = AppConfig::default();
    let parser = UnifiedMovieParser::new(config);

    let result = parser
        .parse_async("钢铁侠.Iron.Man.2008.BluRay.2160p.x265.10bit.HDR.4Audio.mUHD-FRDS.mkv")
        .await
        .unwrap();

    println!("DEBUG: Parsed title: '{}'", result.data.title);
    assert!(!result.data.title.is_empty());
    assert_eq!(result.data.year, Some(2008));
    assert!(result.confidence > 0.0);
}

#[tokio::test]
async fn test_technical_terms_filtering() {
    let config = AppConfig::default();
    let parser = UnifiedMovieParser::new(config);

    let result = parser
        .parse_async("Constantine.2005.1080p.BluRay.x264.DTS-3L.mkv")
        .await
        .unwrap();

    println!("DEBUG: Parsed title: '{}'", result.data.title);
    assert!(!result.data.title.is_empty());
    assert_eq!(result.data.year, Some(2005));
    assert!(result.confidence > 0.0);
}

#[tokio::test]
async fn test_no_year_movie() {
    let config = AppConfig::default();
    let parser = UnifiedMovieParser::new(config);

    let result = parser
        .parse_async("The.Matrix.1080p.BluRay.x264.mkv")
        .await
        .unwrap();

    println!("DEBUG: Parsed title: '{}'", result.data.title);
    assert!(!result.data.title.is_empty());
    assert_eq!(result.data.year, None);
    assert!(result.confidence > 0.0);
}

#[tokio::test]
async fn test_chinese_movie_with_english_subtitle() {
    let config = AppConfig::default();
    let parser = UnifiedMovieParser::new(config);

    let result = parser
        .parse_async("武状元苏乞儿.King.of.Beggars.1992.1080p.BluRay.x264.DTS.2Audio.OurTV.mkv")
        .await
        .unwrap();

    println!("DEBUG: Parsed title: '{}'", result.data.title);
    assert!(!result.data.title.is_empty());
    assert_eq!(result.data.year, Some(1992));
    assert!(result.confidence > 0.0);
}
