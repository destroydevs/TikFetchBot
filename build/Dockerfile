FROM alpine:latest
LABEL authors="destr"

WORKDIR /app
COPY .env /app/.env
COPY ttdownloader /app/ttdownloader

ENTRYPOINT ["./ttdownloader"]