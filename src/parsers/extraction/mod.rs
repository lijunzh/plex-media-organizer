//! Title extraction logic for filename parsing

mod non_english;
mod technical_terms;
pub mod title;

pub use non_english::NonEnglishProcessor;
pub use technical_terms::TechnicalTermsFilter;
pub use title::TitleExtractor;

/// Title extraction result
#[derive(Debug, Clone)]
pub struct ExtractionResult {
    pub title: String,
    pub original_title: Option<String>,
    pub confidence: f32,
    pub warnings: Vec<String>,
}

/// Unified title extraction for all extraction types
#[derive(Clone, Debug)]
pub struct UnifiedTitleExtractor {
    title_extractor: TitleExtractor,
    non_english_processor: NonEnglishProcessor,
    technical_filter: TechnicalTermsFilter,
}

impl Default for UnifiedTitleExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl UnifiedTitleExtractor {
    pub fn new() -> Self {
        Self {
            title_extractor: TitleExtractor::new(),
            non_english_processor: NonEnglishProcessor::new(),
            technical_filter: TechnicalTermsFilter::new(),
        }
    }

    pub fn with_technical_terms(technical_terms: Vec<String>) -> Self {
        Self {
            title_extractor: TitleExtractor::new(),
            non_english_processor: NonEnglishProcessor::new(),
            technical_filter: TechnicalTermsFilter::with_terms(technical_terms),
        }
    }

    /// Extract title from filename
    pub fn extract_title(&self, filename: &str) -> Result<ExtractionResult, anyhow::Error> {
        // First, process non-English titles if present
        let non_english_result = self.non_english_processor.process(filename);

        // Extract main title
        let title_result = self.title_extractor.extract(filename)?;

        // Filter technical terms
        let filtered_title = self.technical_filter.filter(&title_result.title);

        Ok(ExtractionResult {
            title: filtered_title,
            original_title: non_english_result.original_title,
            confidence: title_result.confidence,
            warnings: title_result.warnings,
        })
    }

    /// Get title extractor
    pub fn title_extractor(&self) -> &TitleExtractor {
        &self.title_extractor
    }

    /// Get non-English processor
    pub fn non_english_processor(&self) -> &NonEnglishProcessor {
        &self.non_english_processor
    }

    /// Get technical terms filter
    pub fn technical_filter(&self) -> &TechnicalTermsFilter {
        &self.technical_filter
    }
}
