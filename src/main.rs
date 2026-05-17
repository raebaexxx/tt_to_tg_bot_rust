use teloxide::prelude::*;
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

    let bot = Bot::with_client(config.telegram_bot_token, client);

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
        );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    info!("Bot stopped");
}
