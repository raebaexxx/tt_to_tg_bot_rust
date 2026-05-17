use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub telegram_bot_token: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        dotenvy::dotenv().ok();

        let telegram_bot_token =
            env::var("TELEGRAM_BOT_TOKEN").map_err(|_| "TELEGRAM_BOT_TOKEN not set")?;

        Ok(Self {
            telegram_bot_token,
        })
    }
}
