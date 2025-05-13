@echo off
IF "%1"=="" (
  SET IMAGE_NAME=builder
) ELSE (
  SET IMAGE_NAME=%1
)
IF NOT EXIST build (
  mkdir build
)
docker stop temp 2>nul
docker rm temp 2>nul
docker rmi %IMAGE_NAME% 2>nul
docker build -t %IMAGE_NAME% .
IF %ERRORLEVEL% NEQ 0 exit /b 1
docker create --name temp %IMAGE_NAME%
IF %ERRORLEVEL% NEQ 0 exit /b 1
docker cp temp:/app/target/x86_64-unknown-linux-musl/release/ttdownloader .\build\ttdownloader
IF %ERRORLEVEL% NEQ 0 exit /b 1
docker rm temp
IF %ERRORLEVEL% NEQ 0 exit /b 1
