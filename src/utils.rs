//! Filesystem and string sanitization utilities.

use std::path::Path;

use regex::Regex;
use std::sync::LazyLock;

/// Characters unsafe for file/directory names.
static UNSAFE_CHARS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[<>:"/\\|?*\x00-\x1f]"#).unwrap());

static MULTI_SPACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s{2,}").unwrap());

const MAX_PATH_COMPONENT: usize = 200;

/// Remove unsafe characters from a path component.
pub fn sanitize_name(name: &str) -> String {
    let cleaned = UNSAFE_CHARS.replace_all(name, "");
    let cleaned = MULTI_SPACE.replace_all(&cleaned, " ");
    let mut cleaned = cleaned.trim().trim_end_matches('.').to_string();
    if cleaned.len() > MAX_PATH_COMPONENT {
        cleaned.truncate(MAX_PATH_COMPONENT);
        cleaned = cleaned.trim_end().to_string();
    }
    if cleaned.is_empty() {
        "Unknown".to_string()
    } else {
        cleaned
    }
}

/// Format a file size in bytes to a human-readable string.
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.0} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}

/// Check if a path component is safe (no traversal).
pub fn is_safe_component(component: &str) -> bool {
    !component.is_empty() && component != ".." && component != "." && !component.contains('/')
}

/// Join a path safely, rejecting traversal attempts.
pub fn safe_path_join(base: &Path, component: &str) -> Option<std::path::PathBuf> {
    if !is_safe_component(component) {
        return None;
    }
    Some(base.join(component))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_removes_unsafe_chars() {
        assert_eq!(sanitize_name("Movie: The Sequel?"), "Movie The Sequel");
        assert_eq!(sanitize_name("A/B\\C"), "ABC");
    }

    #[test]
    fn test_sanitize_collapses_spaces() {
        assert_eq!(sanitize_name("Movie   Name"), "Movie Name");
    }

    #[test]
    fn test_sanitize_empty_returns_unknown() {
        assert_eq!(sanitize_name(""), "Unknown");
        assert_eq!(sanitize_name("..."), "Unknown");
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(1500), "1 KB");
        assert_eq!(format_size(1_500_000), "1.4 MB");
        assert_eq!(format_size(1_500_000_000), "1.4 GB");
    }

    #[test]
    fn test_safe_path_join_rejects_traversal() {
        let base = Path::new("/tmp");
        assert!(safe_path_join(base, "..").is_none());
        assert!(safe_path_join(base, ".").is_none());
        assert!(safe_path_join(base, "").is_none());
        assert!(safe_path_join(base, "ok").is_some());
    }
}
