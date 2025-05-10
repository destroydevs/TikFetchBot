use reqwest::{header, Client};
use serde_json::Value;
use crate::data::increment_requests;
use crate::pair::Pair;

// https://github.com/ShaheenJawadi/tiktok-video-downloader
pub async fn parse_tiktok_video(url: &str, id: u64) -> Result<Pair<String, String>, String> {
    let client = Client::new();
    println!("🔍 Fetching video metadata...");

    let api_url = format!("https://www.tikwm.com/api/?url={}", url);
    let response = client.get(&api_url)
        .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .send()
        .await
        .map_err(|_| "Failed to send request".to_string())?;

    let json: Value = response.json()
        .await
        .map_err(|_| "Failed to parse JSON response".to_string())?;

    let video_url = json["data"]["play"]
        .as_str()
        .ok_or("❌ Video URL not found in API response".to_string())?;

    let video_title = json["data"]["title"]
        .as_str()
        .unwrap_or("tiktok_video");

    println!("🔍 Fetching has been finished!");

    // adding +1 to requests_amount on users.json
    increment_requests(id).await.unwrap();

    Ok(Pair::new(video_url.to_string(), video_title.to_string()))
}

pub async fn parse_tiktok_audio(url: &str) -> Result<String, String> {
    let client = Client::new();
    println!("🔍 Fetching audio metadata...");

    let api_url = format!("https://www.tikwm.com/api/?url={}", url);
    let response = client.get(&api_url)
        .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .send()
        .await
        .map_err(|_| "Failed to send request".to_string())?;

    let json: Value = response.json()
        .await
        .map_err(|_| "Failed to parse JSON response".to_string())?;

    let audio_url = json["data"]["music"]
        .as_str()
        .unwrap_or("https://moosic.my.mail.ru/file/7aa9b68114dfa1a4581ce525a1e793b1.mp3");

    println!("🔍 Fetching has been finished!");

    Ok(audio_url.to_string())
}