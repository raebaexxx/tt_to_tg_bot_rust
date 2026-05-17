# TikTok to Telegram Bot

Telegram bot for downloading TikTok videos without watermark, written in Rust.

## Features

- Download TikTok videos without watermark
- Send videos directly to Telegram chat
- Simple and fast
- Long Polling mode

## Requirements

- Rust 1.75+
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) installed on your system
- [ffmpeg](https://ffmpeg.org/) installed on your system
- Telegram Bot Token

## Installation

1. Clone the repository:
```bash
git clone https://github.com/raebaexxx/tt_to_tg_bot_rust.git
cd tt_to_tg_bot_rust
```

2. Install yt-dlp and ffmpeg:
```bash
# Ubuntu/Debian
sudo apt install yt-dlp ffmpeg

# macOS
brew install yt-dlp ffmpeg

# CentOS/Fedora
sudo dnf install yt-dlp ffmpeg
```

3. Create `.env` file:
```bash
cp .env.example .env
```

4. Edit `.env` and add your Telegram bot token:
```
TELEGRAM_BOT_TOKEN=your_bot_token_here
```

5. Build and run:
```bash
cargo run --release
```

## Usage

1. Start a chat with your bot in Telegram
2. Send `/start` to get a welcome message
3. Send a TikTok video link
4. Wait for the bot to download and send the video

## Commands

- `/start` - Start the bot
- `/help` - Show available commands

## Configuration

| Variable | Description | Required |
|----------|-------------|----------|
| `TELEGRAM_BOT_TOKEN` | Telegram bot token from @BotFather | Yes |

## License

MIT
