use crate::data::increment_requests;
use reqwest::{header, Client};
use serde_json::Value;

// https://github.com/ShaheenJawadi/tiktok-video-downloader
pub async fn parse_tiktok_content(url: &str, id: u64) -> Result<Box<dyn Media>, String> {
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

    let images_url: Vec<String> = json["data"]["images"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect::<Vec<String>>()
        })
        .unwrap_or_default();

    let video_title = json["data"]["title"]
        .as_str()
        .unwrap_or("tiktok_video");

    let audio_url = json["data"]["music"]
        .as_str()
        .unwrap_or("https://moosic.my.mail.ru/file/7aa9b68114dfa1a4581ce525a1e793b1.mp3");

    // adding +1 to requests_amount on users.json
    increment_requests(id).await.unwrap();

    if !images_url.is_empty() {

        println!("🔍 Fetching has been finished!");
        return Ok(Box::new(
            PhotoContent {
                title: video_title.to_string(),
                photo_urls: images_url,
                audio_url: audio_url.to_string(),
            }
        ))
    }

    let video_url = json["data"]["play"]
        .as_str()
        .ok_or("Video URL not found in API response".to_string())?;

    println!("🔍 Fetching has been finished!");

    Ok(
        Box::from(VideoContent {
            title: video_title.to_string(),
            video_url: video_url.to_string(),
            audio_url: audio_url.to_string(),
        })
    )
}

pub enum ContentType {
    Photo,
    Video
}

impl PartialEq for ContentType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ContentType::Photo, ContentType::Photo) => true,
            (ContentType::Video, ContentType::Video) => true,
            _ => false,
        }
    }
}

pub trait Media: Send + Sync {
    fn get_title(&self) -> String;
    fn get_content_type(&self) -> ContentType;
    fn get_content_urls(&self) -> Vec<String>;
    fn get_audio_url(&self) -> String;
}

struct VideoContent {
    title: String,
    video_url: String,
    audio_url: String,
}

struct PhotoContent {
    title: String,
    photo_urls: Vec<String>,
    audio_url: String,
}

impl Media for VideoContent {
    fn get_title(&self) -> String {
        self.title.clone()
    }
    fn get_content_type(&self) -> ContentType {
        ContentType::Video
    }
    fn get_content_urls(&self) -> Vec<String> {
        vec![self.video_url.clone()]
    }

    fn get_audio_url(&self) -> String {
        self.audio_url.clone()
    }
}

impl Media for PhotoContent {
    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_content_type(&self) -> ContentType {
        ContentType::Photo
    }
    fn get_content_urls(&self) -> Vec<String> {
        self.photo_urls.iter().map(|p| p.clone()).collect()
    }

    fn get_audio_url(&self) -> String {
        self.audio_url.clone()
    }
}