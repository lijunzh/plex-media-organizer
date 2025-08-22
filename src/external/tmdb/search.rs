//! TMDB search algorithms and result processing

use crate::types::{TmdbMatchResult, TmdbMovie};

/// Search algorithms for TMDB movie matching
#[derive(Clone, Debug)]
pub struct TmdbSearchEngine {
    /// Minimum similarity score for a match
    min_similarity: f32,
    /// Weight for title similarity
    title_weight: f32,
    /// Weight for year similarity
    year_weight: f32,
    /// Weight for popularity
    popularity_weight: f32,
}

impl Default for TmdbSearchEngine {
    fn default() -> Self {
        Self {
            min_similarity: 0.7,
            title_weight: 0.6,
            year_weight: 0.3,
            popularity_weight: 0.1,
        }
    }
}

impl TmdbSearchEngine {
    /// Create a new search engine with custom weights
    pub fn new(
        min_similarity: f32,
        title_weight: f32,
        year_weight: f32,
        popularity_weight: f32,
    ) -> Self {
        Self {
            min_similarity,
            title_weight,
            year_weight,
            popularity_weight,
        }
    }

    /// Find the best match for a movie title and year
    pub fn find_best_match(
        &self,
        title: &str,
        year: Option<u32>,
        candidates: &[TmdbMovie],
    ) -> Option<TmdbMatchResult> {
        if candidates.is_empty() {
            return None;
        }

        let mut best_match: Option<(TmdbMovie, f32)> = None;
        let mut best_score = 0.0;

        for candidate in candidates {
            let score = self.calculate_similarity_score(title, year, candidate);

            if score > best_score && score >= self.min_similarity {
                best_score = score;
                best_match = Some((candidate.clone(), score));
            }
        }

        best_match.map(|(movie, score)| TmdbMatchResult {
            movie,
            confidence_score: score,
        })
    }

    /// Calculate similarity score between search query and candidate movie
    fn calculate_similarity_score(
        &self,
        title: &str,
        year: Option<u32>,
        candidate: &TmdbMovie,
    ) -> f32 {
        let title_score = self.calculate_title_similarity(title, &candidate.title);
        let year_score = self.calculate_year_similarity(year, candidate.release_date.as_deref());
        let popularity_score = self.calculate_popularity_score(candidate.popularity);

        title_score * self.title_weight
            + year_score * self.year_weight
            + popularity_score * self.popularity_weight
    }

    /// Calculate title similarity using fuzzy matching
    fn calculate_title_similarity(&self, query: &str, candidate: &str) -> f32 {
        let query_clean = self.clean_title_for_comparison(query);
        let candidate_clean = self.clean_title_for_comparison(candidate);

        if query_clean == candidate_clean {
            return 1.0;
        }

        // Simple fuzzy matching - can be enhanced with more sophisticated algorithms
        let query_words: Vec<&str> = query_clean.split_whitespace().collect();
        let candidate_words: Vec<&str> = candidate_clean.split_whitespace().collect();

        let mut matches = 0;
        for query_word in &query_words {
            for candidate_word in &candidate_words {
                if query_word == candidate_word {
                    matches += 1;
                    break;
                }
            }
        }

        if query_words.is_empty() || candidate_words.is_empty() {
            return 0.0;
        }

        matches as f32 / query_words.len().max(candidate_words.len()) as f32
    }

    /// Calculate year similarity
    fn calculate_year_similarity(
        &self,
        query_year: Option<u32>,
        release_date: Option<&str>,
    ) -> f32 {
        match (query_year, release_date) {
            (Some(query), Some(date)) => {
                if let Some(year) = self.extract_year_from_date(date) {
                    if query == year {
                        return 1.0;
                    }
                    let diff = (query as i32 - year as i32).abs();
                    if diff <= 1 {
                        return 0.9;
                    } else if diff <= 2 {
                        return 0.7;
                    } else if diff <= 5 {
                        return 0.5;
                    }
                }
                0.0
            }
            _ => 0.5, // Partial score when year information is missing
        }
    }

    /// Calculate popularity score
    fn calculate_popularity_score(&self, popularity: Option<f32>) -> f32 {
        match popularity {
            Some(pop) => {
                if pop > 100.0 {
                    1.0
                } else if pop > 50.0 {
                    0.8
                } else if pop > 20.0 {
                    0.6
                } else if pop > 10.0 {
                    0.4
                } else {
                    0.2
                }
            }
            None => 0.5, // Neutral score when popularity is unknown
        }
    }

    /// Clean title for comparison
    fn clean_title_for_comparison(&self, title: &str) -> String {
        title
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
    }

    /// Extract year from date string
    fn extract_year_from_date(&self, date: &str) -> Option<u32> {
        if date.len() >= 4 {
            date[..4].parse::<u32>().ok()
        } else {
            None
        }
    }

    /// Calculate confidence level based on similarity score
    #[allow(dead_code)]
    fn calculate_confidence(&self, score: f32) -> String {
        if score >= 0.9 {
            "Very High".to_string()
        } else if score >= 0.8 {
            "High".to_string()
        } else if score >= 0.7 {
            "Medium".to_string()
        } else if score >= 0.6 {
            "Low".to_string()
        } else {
            "Very Low".to_string()
        }
    }

    /// Generate title variations for better matching
    pub fn generate_title_variations(&self, title: &str) -> Vec<String> {
        let mut variations = vec![title.to_string()];

        // Remove common prefixes
        let prefixes_to_remove = ["The ", "A ", "An "];
        for prefix in &prefixes_to_remove {
            if let Some(stripped) = title.strip_prefix(prefix) {
                variations.push(stripped.to_string());
            }
        }

        // Add common prefixes if not present
        if !title.starts_with("The ") && !title.starts_with("A ") && !title.starts_with("An ") {
            variations.push(format!("The {}", title));
        }

        // Remove punctuation and special characters
        let clean_title = title
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>();
        if clean_title != title {
            variations.push(clean_title);
        }

        variations
    }
}
