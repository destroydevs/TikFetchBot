use crate::database::Database;
use crate::logger::LOGGER;
use colored::Colorize;
use config::Config;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::sync::Arc;
use colored::control::set_override;
use teloxide::Bot;
use tokio::fs::write;

mod fetcher;
mod bot;
mod data;
mod parser;
mod database;
mod logger;

const CONFIG: &[u8] = include_bytes!("../config.yml");

#[tokio::main]
async fn main() {
    let token = fetch_token();
    load_config_file().await;

    LOGGER.executing("Starting bot...");
    let teloxide_bot = Bot::new(token);

    let db = Arc::new(connect_to_db().await);

    db.init().await.expect("Database initialization failed");

    // start bot
    LOGGER.success("Bot started successfully!");
    bot::start(teloxide_bot, Arc::clone(&db)).await;
}

fn fetch_token() -> String {
    LOGGER.executing("Fetching .env file...");
    let mut file = File::open(".env").unwrap();
    let mut bytes = [0u8;512];

    let size = file.read(&mut bytes).unwrap();

    let slice = &bytes[..size];

    let token = std::str::from_utf8(slice).unwrap();
    LOGGER.success(".env successfully fetched!");
    token.to_string()
}

async fn connect_to_db() -> Database {
    let source = config::File::with_name(get_config_file_path().as_str());

    let config = Config::builder()
        .add_source(source)
        .build()
        .expect("Configuration error");

    let url = config.get_string("database.url").unwrap();
    let port = config.get_int("database.port").unwrap();
    let password = config.get_string("database.password").unwrap();
    let username = config.get_string("database.user").unwrap();
    let database = config.get_string("database.database").unwrap();

    let mut db = Database::new(
        url,
        port as u16,
        password,
        username,
        database
    );

    db.try_connect().await;

    db
}

fn get_config_file_path() -> String {
    let dir = env::current_dir().unwrap().to_str().unwrap().to_string();

    let mut work_file: String = "config.yml".to_string();

    if cfg!(windows) {
        work_file = format!("{}\\config\\config.yml", dir);
    } else {
        work_file = format!("{}/config/config.yml", dir);
    }

    work_file
}

async fn load_config_file() {
    let work_file = get_config_file_path();

    let file = File::open(&work_file);
    if file.is_err() {
        create_dir_all(work_file.as_str().replace("config.yml", "")).unwrap();
        write(work_file, CONFIG)
            .await
            .expect("Could not write to config.yml");
    }
}