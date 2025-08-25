//! Media-specific functionality and metadata extraction
//!
//! This module contains media file analysis, metadata extraction, and
//! media-specific processing logic.

pub mod extractor;
pub mod types;

// Re-export media components
pub use extractor::MetadataExtractor;
pub use types::*;