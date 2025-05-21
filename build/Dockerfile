FROM alpine:latest
LABEL authors="destr"

WORKDIR /app
COPY .env /app/.env
COPY TikFetchBot /app/TikFetchBot

ENTRYPOINT ["./TikFetchBot"]