use std::path::PathBuf;
use std::process::Command;
use tempfile::NamedTempFile;
use tracing::{error, info};

use crate::utils::is_tiktok_url;

pub async fn download_video(url: &str) -> Result<PathBuf, String> {
    if !is_tiktok_url(url) {
        return Err("Invalid TikTok URL".to_string());
    }

    info!("Downloading video from TikTok: {}", url);

    let temp_file = NamedTempFile::new()
        .map_err(|e| format!("Failed to create temp file: {}", e))?;

    let output_path = temp_file
        .path()
        .to_str()
        .ok_or("Invalid temp file path")?
        .to_string();

    let output = Command::new("yt-dlp")
        .args([
            "-o",
            &format!("{}.mp4", output_path),
            "--no-playlist",
            "--extractor-args",
            "tiktok:format=play",
            url,
        ])
        .output()
        .map_err(|e| format!("Failed to execute yt-dlp: {}. Make sure yt-dlp is installed.", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("yt-dlp failed: {}", stderr);
        return Err(format!("yt-dlp error: {}", stderr));
    }

    info!("Video downloaded successfully");

    Ok(PathBuf::from(format!("{}.mp4", output_path)))
}
