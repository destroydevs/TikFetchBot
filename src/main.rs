use std::fs::File;
use std::io::Read;
use colored::Colorize;
use teloxide::Bot;

mod fetcher;
mod bot;
mod data;
mod parser;
mod pair;

#[tokio::main]
async fn main() {
    println!("{}", "Fetching .env file...".bright_blue());
    let mut file = File::open(".env").unwrap();
    let mut bytes = [0u8;512];

    let size = file.read(&mut bytes).unwrap();

    let slice = &bytes[..size];

    let string = std::str::from_utf8(slice).unwrap();
    println!("{}", ".env successfully fetched!".bright_blue());

    println!("{}", "Starting bot...".bright_blue());
    let teloxide_bot = Bot::new(string);

    // start bot
    println!("{}", "Bot started successfully!".bright_blue());
    bot::start(teloxide_bot).await;
}
