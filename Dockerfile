FROM rust:latest as builder
RUN apt-get update && apt-get install -y cmake libssl-dev pkg-config gcc
WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY --from=builder /app/target/release/TikFetchBot /app/TikFetchBot
COPY --from=builder /app/config.yml /app/config.yml
CMD ["./TikFetchBot"]