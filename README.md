### Quick Start Guide

1. **Open Bot in Telegram**  
   [@DestroyGPT_Bot](https://t.me/destroygpt_bot)

2. **Send TikTok URL**  
   `https://vm.tiktok.com/...`  
   `https://www.tiktok.com/...`

3. **Get Content**  
   Video (MP4) + 🎧 Audio (MP3)  
   Processing time: ~1-3 sec

# **Key Features**

## Rust-Powered Core
- Blazing-fast downloads with near-zero latency
- Memory-safe architecture prevents crashes

## Async Processing
- Parallel downloads via Tokio runtime
- Queue system handles multiple requests smoothly

## No Watermarks
- Original-quality videos (1080p/720p)
- Separate high-bitrate audio extraction

## Multi-Language Support
- Auto-detects user language (EN/RU)
- Easy to add more languages

✨ **No Ads** – Clean interface, zero promotions

# ❓ HOW BUILD
First steps - <b><a href="https://www.rust-lang.org/tools/install">Install Rust.</a></b>

## 🔰 Build for Linux:

Clone git repo:
```git clone https://github.com/destroydevs/TikFetchBot.git```

Update packets and download dependencies
> sudo apt update\
> sudo apt install wget\
> sudo apt install gcc\
> sudo apt install -y pkg-config libssl-dev musl-tools\
> sudo apt install cmake\
> sudo apt install -y musl-dev
```
wget https://www.openssl.org/source/openssl-1.1.1w.tar.gz
tar xzf openssl-1.1.1w.tar.gz
cd openssl-1.1.1w

./Configure no-shared no-zlib no-async -fPIC --prefix=/usr/local/musl linux-x86_64
make depend
make -j$(nproc)
```
> sudo make install

Building:
```OPENSSL_DIR=/usr/local/musl cargo build --target x86_64-unknown-linux-musl --release ```

## 🔰 Building for Windows: 
Clone git repo:
```git clone https://github.com/destroydevs/TikFetchBot.git```

Building:
```cargo build --release```

## How to start?
You need create environment `TELOXIDE_TOKEN` in system.\
`TELOXIDE_TOKEN` must containts the bot token.

Bot token you can get here: <a href="https://t.me/BotFather">@BotFather</a>

