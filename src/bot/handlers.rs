use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use teloxide::types::InputFile;
use tracing::{error, info};

use crate::tiktok::downloader;
use crate::utils;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub enum Command {
    Start,
    Help,
}

pub async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            let text = msg.text().unwrap_or("");
            let parts: Vec<&str> = text.splitn(2, ' ').collect();

            if parts.len() > 1 && !parts[1].is_empty() {
                let url = decode_deep_link(parts[1]);
                info!("Deep link URL: {}", url);
                process_video(bot, msg, &url).await?;
            } else {
                bot.send_message(
                    msg.chat.id,
                    "👋 Привет! Отправь мне ссылку на TikTok видео, или используй inline режим для скачивания из любого чата.",
                )
                .await?;
            }
        }
        Command::Help => {
            bot.send_message(
                msg.chat.id,
                "Доступные команды:\n/start — начать\n/help — помощь\n\nИли просто отправь ссылку на TikTok видео.",
            )
            .await?;
        }
    }
    Ok(())
}

fn decode_deep_link(encoded: &str) -> String {
    let decoded = encoded.replace("_", "/");
    if decoded.starts_with("http") {
        decoded
    } else {
        format!("https://{}", decoded)
    }
}

pub async fn message_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(text) = &msg.text() {
        if text.starts_with("/start") {
            return Ok(());
        }

        if let Some(url) = utils::extract_tiktok_url(text) {
            process_video(bot, msg, &url).await?;
        } else {
            bot.send_message(
                msg.chat.id,
                "❌ Не нашёл ссылку на TikTok. Отправь мне ссылку на видео.",
            )
            .await?;
        }
    }
    Ok(())
}

async fn process_video(bot: Bot, msg: Message, url: &str) -> ResponseResult<()> {
    info!("Processing TikTok URL: {}", url);

    let processing_msg = bot
        .send_message(msg.chat.id, "⏳ Скачиваю видео...")
        .await?;

    let result = downloader::download_video(url).await;

    bot.delete_message(msg.chat.id, processing_msg.id)
        .await
        .ok();

    match result {
        Ok(video_path) => {
            let file_size = std::fs::metadata(&video_path)
                .map(|m| m.len())
                .unwrap_or(0);

            if file_size > 50_000_000 {
                bot.send_message(
                    msg.chat.id,
                    "❌ Видео слишком большое (лимит 50MB).",
                )
                .await?;
            } else {
                let sending_msg = bot
                    .send_message(msg.chat.id, "📤 Отправляю видео...")
                    .await?;

                let video_info = utils::get_video_dimensions(&video_path);

                let mut req = bot.send_video(msg.chat.id, InputFile::file(&video_path));

                if let Some((w, h)) = video_info {
                    req = req.width(w).height(h);
                }

                match req.await {
                    Ok(_) => {
                        bot.delete_message(msg.chat.id, sending_msg.id)
                            .await
                            .ok();
                    }
                    Err(e) => {
                        error!("Failed to send video: {}", e);
                        bot.edit_message_text(
                            msg.chat.id,
                            sending_msg.id,
                            "❌ Ошибка при отправке видео. Попробуйте позже.",
                        )
                        .await
                        .ok();
                    }
                }
            }

            utils::cleanup_file(&video_path);
        }
        Err(e) => {
            error!("Download failed: {}", e);
            bot.send_message(
                msg.chat.id,
                format!("❌ Ошибка при скачивании: {}", e),
            )
            .await?;
        }
    }

    Ok(())
}
