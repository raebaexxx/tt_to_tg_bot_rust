use regex::Regex;
use std::path::Path;
use tracing::info;

pub fn is_tiktok_url(text: &str) -> bool {
    let pattern = Regex::new(
        r"(https?://)?(www\.|vm\.)?tiktok\.com/[@\w]+",
    )
    .unwrap();
    pattern.is_match(text)
}

pub fn extract_tiktok_url(text: &str) -> Option<String> {
    let pattern = Regex::new(
        r"https?://(?:www\.|vm\.)?tiktok\.com/[^\s]+",
    )
    .unwrap();
    pattern.find(text).map(|m| m.as_str().to_string())
}

pub fn cleanup_file(path: &Path) {
    if path.exists() {
        if let Err(e) = std::fs::remove_file(path) {
            info!("Failed to remove temp file: {}", e);
        }
    }
}
