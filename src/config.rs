use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub telegram_bot_token: String,
    pub bot_username: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        dotenvy::dotenv().ok();

        let telegram_bot_token =
            env::var("TELEGRAM_BOT_TOKEN").map_err(|_| "TELEGRAM_BOT_TOKEN not set")?;

        let bot_username =
            env::var("BOT_USERNAME").unwrap_or_else(|_| "your_bot_username".to_string());

        Ok(Self {
            telegram_bot_token,
            bot_username,
        })
    }
}
