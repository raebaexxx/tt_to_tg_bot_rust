# TikTok to Telegram Bot

[![CI](https://github.com/raebaexxx/tt_to_tg_bot_rust/actions/workflows/ci.yml/badge.svg)](https://github.com/raebaexxx/tt_to_tg_bot_rust/actions/workflows/ci.yml)
[![Build & Release](https://github.com/raebaexxx/tt_to_tg_bot_rust/actions/workflows/release.yml/badge.svg)](https://github.com/raebaexxx/tt_to_tg_bot_rust/actions/workflows/release.yml)

Telegram bot for downloading TikTok videos without watermark, written in Rust.

## Features

- Download TikTok videos without watermark
- Send videos directly to Telegram chat
- Inline mode — download videos from any chat
- iOS compatible video format
- Long Polling mode

## Quick Start (pre-built binary)

Download the latest binary from [Releases](https://github.com/raebaexxx/tt_to_tg_bot_rust/releases/tag/latest):

```bash
# Download
curl -L -o tt_to_tg_bot-x86_64-linux-musl \
  https://github.com/raebaexxx/tt_to_tg_bot_rust/releases/latest/download/tt_to_tg_bot-x86_64-linux-musl

chmod +x tt_to_tg_bot-x86_64-linux-musl

# Create config
echo "TELEGRAM_BOT_TOKEN=your_token_here" > .env

# Run
./tt_to_tg_bot-x86_64-linux-musl
```

## Requirements

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) installed on your system
- [ffmpeg](https://ffmpeg.org/) installed on your system
- Telegram Bot Token

## Installation (from source)

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

## Running in Background

### Option 1: nohup (simplest)
```bash
nohup ./tt_to_tg_bot-x86_64-linux-musl > bot.log 2>&1 &
```
- View logs: `tail -f bot.log`
- Stop bot: `pkill tt_to_tg_bot-x86_64-linux-musl`

### Option 2: screen
```bash
screen -S bot
./tt_to_tg_bot-x86_64-linux-musl
# Press Ctrl+A, then D to detach
```
- Reattach: `screen -r bot`

### Option 3: tmux
```bash
tmux new -s bot
./tt_to_tg_bot-x86_64-linux-musl
# Press Ctrl+B, then D to detach
```
- Reattach: `tmux attach -t bot`

### Option 4: systemd service
```bash
sudo tee /etc/systemd/system/tt_to_tg_bot.service << 'EOF'
[Unit]
Description=TikTok to Telegram Bot
After=network.target

[Service]
Type=simple
User=your_user
WorkingDirectory=/path/to/bot
ExecStart=/path/to/bot/tt_to_tg_bot-x86_64-linux-musl
Restart=always
RestartSec=5
EnvironmentFile=/path/to/bot/.env

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable --now tt_to_tg_bot
```

## License

MIT
