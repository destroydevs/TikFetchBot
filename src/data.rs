use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;

const FILE_PATH: &str = "users.json";

pub type BotResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

lazy_static! {
    static ref DATA_MUTEX: Mutex<()> = Mutex::new(());
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u64,
    pub chat_id: Option<i64>,
    pub name: String,
    pub requests_amount: u64,
    pub timestamp: u64,
    pub register_timestamp: u64,
}

pub enum UserColumn {
    Id,
    Name,
    RequestsAmount,
    Timestamp,
    ChatId,
    RegisterTimestamp
}

async fn read_users() -> BotResult<HashMap<u64, User>> {
    let path = Path::new(FILE_PATH);
    if !path.exists() {
        return Ok(HashMap::new());
    }

    let content = tokio::fs::read_to_string(path).await?;
    let users = serde_json::from_str(&content)?;
    Ok(users)
}

async fn write_users(users: &HashMap<u64, User>) -> BotResult<()> {
    let path = Path::new(FILE_PATH);
    let content = serde_json::to_string_pretty(users)?;
    tokio::fs::write(path, content).await?;
    Ok(())
}

pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

#[allow(dead_code)]
pub async fn set_data(
    user_id: u64,
    column: UserColumn,
    value: &str,
) -> BotResult<()> {
    let _lock = DATA_MUTEX.lock().await;
    let mut users = read_users().await?;

    let user = users.get_mut(&user_id)
        .ok_or("User not found")?;

    match column {
        UserColumn::Name => user.name = value.to_string(),
        UserColumn::RequestsAmount => user.requests_amount = value.parse()?,
        UserColumn::Timestamp => user.timestamp = value.parse()?,
        UserColumn::Id => return Err("Cannot modify user ID".into()),
        UserColumn::RegisterTimestamp => return Err("Cannot modify user register timestamp".into()),
        UserColumn::ChatId => user.chat_id = value.parse().ok(),
    }

    write_users(&users).await?;
    Ok(())
}

pub async fn has_user(id: u64) -> BotResult<bool> {
    let _lock = DATA_MUTEX.lock().await;
    let users = read_users().await?;
    Ok(users.contains_key(&id))
}

pub async fn add_user(user: User) -> BotResult<()> {
    let _lock = DATA_MUTEX.lock().await;
    let mut users = read_users().await?;

    if users.contains_key(&user.id) {
        return Err("User already exists".into());
    }

    users.insert(user.id, user);
    write_users(&users).await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn update_user(user: User) -> BotResult<()> {
    let _lock = DATA_MUTEX.lock().await;
    let mut users = read_users().await?;

    if !users.contains_key(&user.id) {
        return Err("User not found".into());
    }

    users.insert(user.id, user);
    write_users(&users).await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn remove_user(id: u64) -> BotResult<()> {
    let _lock = DATA_MUTEX.lock().await;
    let mut users = read_users().await?;

    if users.remove(&id).is_none() {
        return Err("User not found".into());
    }

    write_users(&users).await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn get_data(user_id: u64, column: UserColumn) -> BotResult<Option<String>> {
    let _lock = DATA_MUTEX.lock().await;
    let users = read_users().await?;

    Ok(users.get(&user_id).and_then(|user| {
        match column {
            UserColumn::Id => Some(user.id.to_string()),
            UserColumn::Name => Some(user.name.clone()),
            UserColumn::RequestsAmount => Some(user.requests_amount.to_string()),
            UserColumn::Timestamp => Some(user.timestamp.to_string()),
            UserColumn::ChatId => user.chat_id.map(|id| id.to_string()),
            UserColumn::RegisterTimestamp => Some(user.register_timestamp.to_string()),
        }
    }))
}

pub async fn get_user(id: u64) -> BotResult<Option<User>> {
    let _lock = DATA_MUTEX.lock().await;
    let users = read_users().await?;
    Ok(users.get(&id).cloned())
}

pub async fn increment_requests(id: u64) -> BotResult<()> {
    let _lock = DATA_MUTEX.lock().await;
    let mut users = read_users().await?;

    if let Some(user) = users.get_mut(&id) {
        user.requests_amount += 1;
        user.timestamp = current_timestamp();
        write_users(&users).await?;
        Ok(())
    } else {
        Err("User not found".into())
    }
}