#!/usr/bin/env bash
IMAGE_NAME=${1:-builder}
mkdir -p build
docker stop temp 2>/dev/null >&2 || true
docker rm temp 2>/dev/null >&2 || true
docker rmi "$IMAGE_NAME" 2>/dev/null >&2 || true
docker build -t "$IMAGE_NAME" .
if [ $? -ne 0 ]; then exit 1; fi
docker create --name temp "$IMAGE_NAME"
if [ $? -ne 0 ]; then exit 1; fi
docker cp temp:/app/target/x86_64-unknown-linux-musl/release/ttdownloader "$(cygpath -w ./build/ttdownloader)"
if [ $? -ne 0 ]; then exit 1; fi
docker rm temp
if [ $? -ne 0 ]; then exit 1; fi
