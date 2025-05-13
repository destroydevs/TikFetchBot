FROM rust:latest as builder

RUN apt-get update && \
    apt-get install -y cmake musl-tools libssl-dev wget gcc pkg-config musl-dev && \
    rustup target add x86_64-unknown-linux-musl && \
    wget https://www.openssl.org/source/openssl-1.1.1w.tar.gz && \
    tar xzf openssl-1.1.1w.tar.gz && \
    cd openssl-1.1.1w && \
    ./Configure no-shared no-zlib no-async -fPIC --prefix=/usr/local/musl linux-x86_64 && \
    make depend && \
    make -j$(nproc) && \
    make install

WORKDIR /app
COPY /src /app/src
COPY Cargo.toml /app/Cargo.toml

RUN OPENSSL_DIR=/usr/local/musl cargo build --target x86_64-unknown-linux-musl --release