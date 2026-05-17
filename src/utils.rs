use regex::Regex;
use std::path::Path;
use std::process::Command;
use tracing::info;

pub fn is_tiktok_url(text: &str) -> bool {
    let pattern = Regex::new(
        r"(https?://)?(www\.|vm\.|vt\.|m\.)?tiktok\.com/[@\w]+",
    )
    .unwrap();
    pattern.is_match(text)
}

pub fn extract_tiktok_url(text: &str) -> Option<String> {
    let pattern = Regex::new(
        r"https?://(?:www\.|vm\.|vt\.|m\.)?tiktok\.com/[^\s]+",
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

pub fn get_video_dimensions(path: &Path) -> Option<(u32, u32)> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-show_entries",
            "stream=width,height",
            "-of",
            "csv=p=0:s=x",
            path.to_str()?,
        ])
        .output()
        .ok()?;

    let dims = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let parts: Vec<&str> = dims.split('x').collect();
    if parts.len() == 2 {
        let w = parts[0].parse().ok()?;
        let h = parts[1].parse().ok()?;
        Some((w, h))
    } else {
        None
    }
}
