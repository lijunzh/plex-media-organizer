//! Enrichment orchestrator — validates parsed metadata against databases.
//!
//! Phase 1: Pass-through enricher that promotes parsed data into
//! enriched models with no network calls. Future phases will add
//! TMDb, MusicBrainz, and web search providers.

use tracing::debug;

use crate::config::AppConfig;
use crate::models::{EnrichedMedia, MediaType, Movie, MusicTrack, ParsedMedia, TvEpisode};

/// Enrichment pipeline.
pub struct Enricher {
    config: AppConfig,
}

impl Enricher {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    /// Enrich parsed metadata.
    ///
    /// Currently a pass-through that wraps parsed data into enriched models.
    /// Future phases will query TMDb, MusicBrainz, and web search.
    pub fn enrich(&self, parsed: ParsedMedia) -> EnrichedMedia {
        let mut enriched = EnrichedMedia::from_parsed(parsed.clone());

        match parsed.media_type {
            MediaType::Movie => self.enrich_movie(&parsed, &mut enriched),
            MediaType::Tv => self.enrich_tv(&parsed, &mut enriched),
            MediaType::Music => self.enrich_music(&parsed, &mut enriched),
            MediaType::Unknown => {
                debug!(
                    "unknown type for {:?}, skipping enrichment",
                    parsed.raw_filename
                );
            }
        }

        // Mark for review if confidence is too low
        if enriched.confidence < self.config.review_threshold {
            enriched.needs_review = true;
            debug!(
                "marking {:?} for review (conf={:.0} < threshold={:.0})",
                parsed.raw_filename, enriched.confidence, self.config.review_threshold
            );
        }

        enriched
    }

    fn enrich_movie(&self, parsed: &ParsedMedia, enriched: &mut EnrichedMedia) {
        // Phase 1: promote parsed data directly (no DB lookup yet)
        enriched.movie = Some(Movie {
            title: parsed.title.clone(),
            year: parsed.year,
            tmdb_id: None,
            original_title: None,
            confidence: parsed.confidence,
        });
        enriched.enrichment_source = Some("parser".to_string());
    }

    fn enrich_tv(&self, parsed: &ParsedMedia, enriched: &mut EnrichedMedia) {
        enriched.tv_episode = Some(TvEpisode {
            show_title: parsed.title.clone(),
            season: parsed.season.unwrap_or(1),
            episode: parsed.episode.unwrap_or(0),
            episode_end: parsed.episode_end,
            episode_title: parsed.episode_title.clone(),
            year: parsed.year,
            tmdb_id: None,
            confidence: parsed.confidence,
        });
        enriched.enrichment_source = Some("parser".to_string());
    }

    fn enrich_music(&self, parsed: &ParsedMedia, enriched: &mut EnrichedMedia) {
        enriched.music_track = Some(MusicTrack {
            artist: parsed.artist.clone().unwrap_or_default(),
            album: parsed.album.clone(),
            track_title: parsed.track_title.clone(),
            track_number: parsed.track_number,
            year: parsed.year,
            confidence: parsed.confidence,
        });
        enriched.enrichment_source = Some("parser".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_enricher() -> Enricher {
        Enricher::new(AppConfig::default())
    }

    #[test]
    fn test_enrich_movie_pass_through() {
        let parsed = ParsedMedia {
            title: "The Matrix".to_string(),
            year: Some(1999),
            media_type: MediaType::Movie,
            confidence: 80.0,
            ..Default::default()
        };
        let enriched = default_enricher().enrich(parsed);
        assert!(enriched.movie.is_some());
        assert_eq!(enriched.movie.unwrap().title, "The Matrix");
        assert_eq!(enriched.media_type, MediaType::Movie);
    }

    #[test]
    fn test_enrich_tv_pass_through() {
        let parsed = ParsedMedia {
            title: "Breaking Bad".to_string(),
            season: Some(1),
            episode: Some(1),
            media_type: MediaType::Tv,
            confidence: 80.0,
            ..Default::default()
        };
        let enriched = default_enricher().enrich(parsed);
        assert!(enriched.tv_episode.is_some());
        let ep = enriched.tv_episode.unwrap();
        assert_eq!(ep.show_title, "Breaking Bad");
        assert_eq!(ep.season, 1);
        assert_eq!(ep.episode, 1);
    }

    #[test]
    fn test_low_confidence_flagged_for_review() {
        let parsed = ParsedMedia {
            title: "Something".to_string(),
            media_type: MediaType::Movie,
            confidence: 30.0,
            ..Default::default()
        };
        let enriched = default_enricher().enrich(parsed);
        assert!(enriched.needs_review);
    }
}
