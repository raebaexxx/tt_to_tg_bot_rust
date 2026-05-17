# TikTok to Telegram Bot

Telegram bot for downloading TikTok videos without watermark, written in Rust.

## Features

- Download TikTok videos without watermark
- Send videos directly to Telegram chat
- Inline mode — download videos from any chat
- iOS compatible video format
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

4. Edit `.env` and add your Telegram bot token and username:
```
TELEGRAM_BOT_TOKEN=your_bot_token_here
BOT_USERNAME=your_bot_username
```

5. Build and run:
```bash
cargo run --release
```

## Usage

### Direct mode
1. Open chat with your bot
2. Send `/start` to get a welcome message
3. Send a TikTok video link
4. Wait for the bot to download and send the video

### Inline mode
1. In any chat, type `@your_bot_username https://vt.tiktok.com/...`
2. Click the result button
3. Bot opens in chat and automatically downloads the video

## Commands

- `/start` — Start the bot
- `/help` — Show available commands

## Configuration

| Variable | Description | Required |
|----------|-------------|----------|
| `TELEGRAM_BOT_TOKEN` | Telegram bot token from @BotFather | Yes |
| `BOT_USERNAME` | Bot username (e.g. `my_tiktok_bot`) | Yes (for inline mode) |

## Running in Background

To keep the bot running after disconnecting from SSH:

### Option 1: nohup (simplest)
```bash
nohup cargo run --release > bot.log 2>&1 &
```
- View logs: `tail -f bot.log`
- Stop bot: `pkill tt_to_tg_bot`

### Option 2: screen
```bash
screen -S bot
cargo run --release
# Press Ctrl+A, then D to detach
```
- Reattach: `screen -r bot`

### Option 3: tmux
```bash
tmux new -s bot
cargo run --release
# Press Ctrl+B, then D to detach
```
- Reattach: `tmux attach -t bot`

## License

MIT
