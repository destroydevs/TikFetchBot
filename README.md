
> Данный бот создан исключительно в образовательных целях для:
> - Изучения работы с teloxide
> - Практики асинхронного программирования (Tokio)
> - Исследования механизмов обработки мультимедийного контента\
> \
> Контент не хранится на моих серверах.\
> Пользователи получают только те данные, которые TikTok предоставляет публично.\
> Проект не монетизируется и не имеет коммерческой выгоды.\
> Используется только публичное API, доступное всем разработчикам.\
> Я не несу ответственности за действия пользователей.

# 🔧 **Key Features**

## ✅ Rust-Powered Core
- Blazing-fast downloads with near-zero latency
- Memory-safe architecture prevents crashes

## ✅ Async Processing
- Parallel downloads via Tokio runtime
- Queue system handles multiple requests smoothly

## ✅ Zero-Config Operation
- Just send a link – no commands needed
- Auto-detects TikTok URLs (all domains supported)

## ✅ No Watermarks
- Original-quality videos (1080p/720p)
- Separate high-bitrate audio extraction

## ✅ Multi-Language Support
- Auto-detects user language (EN/RU)
- Easy to add more languages

## ✅ 24/7 Uptime
- Cloud deployment ready
- Auto-recovery from failures

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

