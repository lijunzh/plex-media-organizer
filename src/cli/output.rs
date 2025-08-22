//! CLI output and progress reporting

use indicatif::{ProgressBar, ProgressStyle};

/// Progress reporter for CLI operations
pub struct ProgressReporter {
    progress_bar: Option<ProgressBar>,
    verbose: bool,
}

impl ProgressReporter {
    /// Create a new progress reporter
    pub fn new(verbose: bool) -> Self {
        Self {
            progress_bar: None,
            verbose,
        }
    }

    /// Start a progress bar with the given message
    pub fn start_progress(&mut self, message: &str, total: u64) {
        if !self.verbose {
            let pb = ProgressBar::new(total);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
                    .unwrap()
                    .progress_chars("#>-"),
            );
            pb.set_message(message.to_string());
            self.progress_bar = Some(pb);
        } else {
            println!("🔄 {}", message);
        }
    }

    /// Update progress
    pub fn update_progress(&self, current: u64) {
        if let Some(ref pb) = self.progress_bar {
            pb.set_position(current);
        }
    }

    /// Update progress message
    pub fn update_message(&self, message: &str) {
        if let Some(ref pb) = self.progress_bar {
            pb.set_message(message.to_string());
        } else if self.verbose {
            println!("📝 {}", message);
        }
    }

    /// Finish progress
    pub fn finish_progress(&mut self, message: &str) {
        if let Some(pb) = self.progress_bar.take() {
            pb.finish_with_message(message.to_string());
        } else if self.verbose {
            println!("✅ {}", message);
        }
    }

    /// Print a message (respects verbose setting)
    pub fn print_message(&self, message: &str) {
        if self.verbose {
            println!("{}", message);
        }
    }

    /// Print a success message
    pub fn print_success(&self, message: &str) {
        if self.verbose {
            println!("✅ {}", message);
        }
    }

    /// Print a warning message
    pub fn print_warning(&self, message: &str) {
        if self.verbose {
            println!("⚠️  {}", message);
        }
    }

    /// Print an error message
    pub fn print_error(&self, message: &str) {
        if self.verbose {
            println!("❌ {}", message);
        }
    }
}

/// Print a formatted section header
pub fn print_section_header(title: &str) {
    println!("\n🎬 {}", title);
    println!("{}", "=".repeat(title.len() + 4));
}

/// Print a formatted subsection header
pub fn print_subsection_header(title: &str) {
    println!("\n📋 {}", title);
    println!("{}", "-".repeat(title.len() + 4));
}

/// Print a summary table
pub fn print_summary_table(items: &[(&str, &str)]) {
    let max_key_len = items.iter().map(|(key, _)| key.len()).max().unwrap_or(0);

    for (key, value) in items {
        println!("{:<width$} : {}", key, value, width = max_key_len);
    }
}
