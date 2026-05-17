use teloxide::prelude::*;
use teloxide::types::{InlineQueryResult, InlineQueryResultArticle, InlineQueryResultCachedVideo, InputMessageContent, InputMessageContentText, InlineKeyboardMarkup, InlineKeyboardButton, InputFile};
use tracing::{error, info};
use std::time::Duration;
use std::sync::Arc;
use tokio::sync::Mutex;

mod bot;
mod config;
mod tiktok;
mod utils;

use bot::handlers::{command_handler, message_handler, Command};

type FileIdCache = Arc<Mutex<std::collections::HashMap<String, String>>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("tt_to_tg_bot=info".parse().unwrap()),
        )
        .init();

    let config = match config::Config::from_env() {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(300))
        .build()
        .expect("Failed to create HTTP client");

    let bot = Bot::with_client(config.telegram_bot_token.clone(), client);
    let file_id_cache: FileIdCache = Arc::new(Mutex::new(std::collections::HashMap::new()));

    info!("Starting TikTok to Telegram bot...");

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(command_handler),
        )
        .branch(
            Update::filter_message()
                .endpoint(|bot: Bot, msg: Message| async move {
                    message_handler(bot, msg).await
                }),
        )
        .branch(
            Update::filter_inline_query()
                .endpoint(move |bot: Bot, query: InlineQuery| {
                    let cache = file_id_cache.clone();
                    async move {
                        inline_handler(bot, query, "", cache).await
                    }
                }),
        );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    info!("Bot stopped");
}

async fn inline_handler(
    bot: Bot,
    query: InlineQuery,
    _bot_username: &str,
    cache: FileIdCache,
) -> ResponseResult<()> {
    if let Some(url) = utils::extract_tiktok_url(&query.query) {
        let cache_key = url.clone();

        if let Some(file_id) = cache.lock().await.get(&cache_key).cloned() {
            info!("Using cached file_id for: {}", url);

            let result = InlineQueryResultCachedVideo::new(
                "cached_video",
                file_id,
                "TikTok видео",
            )
            .caption("Видео без водяного знака");

            bot.answer_inline_query(query.id, vec![InlineQueryResult::CachedVideo(result)])
                .await?;
            return Ok(());
        }

        info!("Downloading video for inline query: {}", url);

        match tiktok::downloader::download_video(&url).await {
            Ok(video_path) => {
                let file_size = std::fs::metadata(&video_path)
                    .map(|m| m.len())
                    .unwrap_or(0);

                if file_size > 50_000_000 {
                    let result = InlineQueryResultArticle::new(
                        "too_large",
                        "Видео слишком большое",
                        InputMessageContent::Text(InputMessageContentText::new("❌ Видео больше 50MB и не может быть отправлено")),
                    );
                    bot.answer_inline_query(query.id, vec![InlineQueryResult::Article(result)])
                        .await?;
                    utils::cleanup_file(&video_path);
                    return Ok(());
                }

                let video_info = utils::get_video_dimensions(&video_path);

                let mut send_req = bot.send_video(query.from.id, InputFile::file(&video_path));

                if let Some((w, h)) = video_info {
                    send_req = send_req.width(w).height(h);
                }

                match send_req.await {
                    Ok(msg) => {
                        if let Some(video) = msg.video() {
                            let file_id = video.file.id.clone();
                            info!("Got file_id: {}", file_id);
                            cache.lock().await.insert(cache_key, file_id);
                        }

                        let keyboard = InlineKeyboardMarkup::new(vec![vec![
                            InlineKeyboardButton::switch_inline_query_current_chat(
                                "🔍 Скачать ещё".to_string(),
                                "".to_string(),
                            ),
                        ]]);

                        let result = InlineQueryResultArticle::new(
                            "success",
                            "Видео отправлено",
                            InputMessageContent::Text(InputMessageContentText::new("✅ Видео отправлено в этот чат")),
                        )
                        .reply_markup(keyboard);

                        bot.answer_inline_query(query.id, vec![InlineQueryResult::Article(result)])
                            .await?;
                    }
                    Err(e) => {
                        error!("Failed to send video: {}", e);
                        let result = InlineQueryResultArticle::new(
                            "error",
                            "Ошибка отправки",
                            InputMessageContent::Text(InputMessageContentText::new("❌ Ошибка при отправке видео")),
                        );
                        bot.answer_inline_query(query.id, vec![InlineQueryResult::Article(result)])
                            .await?;
                    }
                }

                utils::cleanup_file(&video_path);
            }
            Err(e) => {
                error!("Download failed: {}", e);
                let result = InlineQueryResultArticle::new(
                    "download_error",
                    "Ошибка скачивания",
                    InputMessageContent::Text(InputMessageContentText::new(&format!("❌ Ошибка: {}", e))),
                );
                bot.answer_inline_query(query.id, vec![InlineQueryResult::Article(result)])
                    .await?;
            }
        }
    } else if !query.query.is_empty() {
        let result = InlineQueryResultArticle::new(
            "no_url",
            "Не нашёл TikTok ссылку",
            InputMessageContent::Text(InputMessageContentText::new("Введи ссылку на TikTok видео после упоминания бота")),
        )
        .description("Попробуй ввести ссылку на TikTok");

        bot.answer_inline_query(query.id, vec![InlineQueryResult::Article(result)])
            .await?;
    } else {
        let result = InlineQueryResultArticle::new(
            "help",
            "Скачать TikTok видео",
            InputMessageContent::Text(InputMessageContentText::new("Введи ссылку на TikTok видео после упоминания бота, например: @bot https://vt.tiktok.com/...")),
        )
        .description("Нажми и введи ссылку на TikTok");

        bot.answer_inline_query(query.id, vec![InlineQueryResult::Article(result)])
            .await?;
    }

    Ok(())
}
