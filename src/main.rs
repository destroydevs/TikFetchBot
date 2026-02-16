//   Copyright 2025 Evgeny K.
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

use crate::logger::LOGGER;
use colored::Colorize;
use config::Config;
use reqwest::Client;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::sync::Arc;
use colored::control::set_override;
use teloxide::Bot;
use tokio::fs::write;

mod media_fetcher;
mod bot;
mod data;
mod parser;
mod logger;

const CONFIG: &[u8] = include_bytes!("../config.yml");

#[tokio::main]
async fn main() {
    let token = fetch_token();

    load_config_file().await;

    LOGGER.executing("Starting bot...");
    let teloxide_bot = Bot::new(token);

    let client = Client::new();

    // start bot
    LOGGER.success("Bot started successfully!");
    bot::start(teloxide_bot, Arc::new(client)).await;
}

fn fetch_token() -> Result<String, <String>> {
    LOGGER.executing("Fetching token from environment variables...");

    let env_tocken = env::var("TELOXIDE_TOKEN");
    if env_tocken.is_ok() {
        LOGGER.success("Token fetched successfully!");
        return Ok(env_tocken.unwrap());
    }

    Err("TELOXIDE_TOKEN environment variable not set".into())
}

fn get_config_file_path() -> String {
    let dir = env::current_dir().unwrap().to_str().unwrap().to_string();

    let mut work_file: String = "config.yml".to_string();

    if cfg!(windows) {
        // dos/windows systems
        work_file = format!("{}\\config\\config.yml", dir);
    } else {
        // unix systems
        work_file = format!("{}/config/config.yml", dir);
    }

    work_file
}

async fn load_config_file() {
    LOGGER.executing("Loading config.yml file...");
    let work_file = get_config_file_path();

    let file = File::open(&work_file);
    if file.is_err() {
        create_dir_all(work_file.as_str().replace("config.yml", "")).unwrap();
        write(work_file, CONFIG)
            .await
            .expect("Could not write to config.yml");
        LOGGER.success("config.yml file created successfully!");
    }
    LOGGER.success("config.yml file loaded successfully!");
}