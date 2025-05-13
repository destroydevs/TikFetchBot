@echo off
docker build -t ttdownloader .
docker save ttdownloader -o ttdownloader.tar
