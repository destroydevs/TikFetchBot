FROM rust:latest as builder

RUN apt-get update && \
    apt-get install -y cmake libssl-dev pkg-config gcc && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY src ./src
COPY config.yml ./config.yml

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/TikFetchBot /app/TikFetchBot
COPY --from=builder /app/config.yml /app/config.yml

CMD ["./TikFetchBot"]