//! Core processing and orchestration logic
//!
//! This module contains the main processing pipeline and workflow coordination
//! for the media organization system.

pub mod processor;
pub mod scanner;
pub mod organizer;

// Re-export core components
pub use processor::MediaProcessor;
pub use scanner::MediaScanner;
pub use organizer::MediaOrganizer;