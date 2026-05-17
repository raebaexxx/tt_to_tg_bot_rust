use teloxide::prelude::*;
use teloxide::types::{InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText, InlineKeyboardMarkup, InlineKeyboardButton};
use tracing::{error, info};
use std::time::Duration;

mod bot;
mod config;
mod tiktok;
mod utils;

use bot::handlers::{command_handler, message_handler, Command};

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
    let bot_username = config.bot_username.clone();

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
                    let bot_username = bot_username.clone();
                    async move {
                        inline_handler(bot, query, &bot_username).await
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

async fn inline_handler(bot: Bot, query: InlineQuery, bot_username: &str) -> ResponseResult<()> {
    if let Some(url) = utils::extract_tiktok_url(&query.query) {
        let encoded_url = url
            .replace("https://", "")
            .replace("http://", "")
            .replace("/", "_");
        let start_url = format!("https://t.me/{}?start={}", bot_username, encoded_url);
        let parsed_url = reqwest::Url::parse(&start_url).unwrap();

        let keyboard = InlineKeyboardMarkup::new(vec![vec![
            InlineKeyboardButton::url("▶️ Скачать видео".to_string(), parsed_url),
        ]]);

        let result = InlineQueryResultArticle::new(
            "download",
            "Скачать TikTok видео",
            InputMessageContent::Text(InputMessageContentText::new("Нажми кнопку ниже, чтобы скачать видео без водяного знака")),
        )
        .description("Нажми кнопку для скачивания")
        .reply_markup(keyboard);

        bot.answer_inline_query(query.id, vec![InlineQueryResult::Article(result)])
            .await?;
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
