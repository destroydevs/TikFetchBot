@echo off
docker build -t tikfetchbot .
docker save tikfetchbot -o TikFetchBot.tar