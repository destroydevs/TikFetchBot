use crate::data::current_timestamp;
use crate::logger::LOGGER;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{Executor, FromRow, Pool, Postgres, Row};

pub type BotResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub struct Database {
    url: String,
    port: u16,
    password: String,
    username: String,
    database: String,
    pool: Option<Pool<Postgres>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct User {
    pub id: i64,
    pub chat_id: Option<i64>,
    pub name: String,
    pub requests_amount: i64,
    pub timestamp: i64,
    pub register_timestamp: i64,
}

pub enum UserColumn {
    Id,
    Name,
    RequestsAmount,
    Timestamp,
    ChatId,
    RegisterTimestamp,
}

impl Database {
    pub fn new(url: String, port: u16, password: String, username: String, database: String) -> Database {
        Database {
            url,
            port,
            password,
            username,
            database,
            pool: None,
        }
    }

    pub async fn try_connect(&mut self) {

        LOGGER.executing("Connecting to postgres database...");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(format!(
                "postgres://{}:{}@{}:{}/{}",
                self.username,
                self.password,
                self.url,
                self.port,
                self.database,
            ).as_str())
            .await
            .expect("Failed to connect to database \nReason");

        LOGGER.success("Connected to postgres database!");

        self.pool = Option::from(pool)
    }

    pub async fn init(&self) -> BotResult<()> {
        if let Some(pool) = &self.pool {
            sqlx::query(
                r#"
                    CREATE TABLE IF NOT EXISTS users (
                        id BIGINT PRIMARY KEY,
                        chat_id BIGINT,
                        name TEXT NOT NULL,
                        requests_amount BIGINT NOT NULL DEFAULT 0,
                        timestamp BIGINT NOT NULL,
                        register_timestamp BIGINT NOT NULL
                    )
                    "#,
            )
                .execute(pool)
                .await?;
        }

        Ok(())
    }

    pub async fn set_data(
        &self,
        user_id: i64,
        column: UserColumn,
        value: &str,
    ) -> BotResult<()> {
        let query = match column {
            UserColumn::Name => "UPDATE users SET name = $1 WHERE id = $2",
            UserColumn::RequestsAmount => "UPDATE users SET requests_amount = $1 WHERE id = $2",
            UserColumn::Timestamp => "UPDATE users SET timestamp = $1 WHERE id = $2",
            UserColumn::ChatId => "UPDATE users SET chat_id = $1 WHERE id = $2",
            _ => return Err("Cannot modify this column".into()),
        };

        let parsed_value = match column {
            UserColumn::Name => value.to_string(),
            UserColumn::ChatId => value.parse()?,
            _ => value.parse::<i64>()?.to_string(),
        };

        if let Some(pool) = &self.pool {
            sqlx::query(query)
                .bind(parsed_value)
                .bind(user_id)
                .execute(pool)
                .await?;
        }

        Ok(())
    }

    pub async fn has_user(&self, id: i64) -> BotResult<bool> {

        if let Some(pool) = &self.pool {
            let user = sqlx::query("SELECT id FROM users WHERE id = $1")
                .bind(id)
                .fetch_optional(pool)
                .await?;

            Ok(user.is_some())
        } else {
            Ok(false)
        }
    }

    pub async fn add_user(&self, user: User) -> BotResult<()> {
        if let Some(pool) = &self.pool {
            sqlx::query(
                r#"
                    INSERT INTO users (id, chat_id, name, requests_amount, timestamp, register_timestamp)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    "#,
            )
                .bind(user.id)
                .bind(user.chat_id)
                .bind(user.name)
                .bind(user.requests_amount)
                .bind(user.timestamp)
                .bind(user.register_timestamp)
                .execute(pool)
                .await?;
        }
        Ok(())
    }

    pub async fn update_user(&self, user: User) -> BotResult<()> {
        if let Some(pool) = &self.pool {
            sqlx::query(
                r#"
                    UPDATE users SET
                        chat_id = $1,
                        name = $2,
                        requests_amount = $3,
                        timestamp = $4,
                        register_timestamp = $5
                    WHERE id = $6
                    "#,
            )
                .bind(user.chat_id)
                .bind(user.name)
                .bind(user.requests_amount)
                .bind(user.timestamp)
                .bind(user.register_timestamp)
                .bind(user.id)
                .execute(pool)
                .await?;
        }

        Ok(())
    }

    pub async fn remove_user(&self, id: i64) -> BotResult<()> {
        if let Some(pool) = &self.pool {
            sqlx::query("DELETE FROM users WHERE id = $1")
                .bind(id)
                .execute(pool)
                .await?;
        }

        Ok(())
    }

    pub async fn get_data(&self, user_id: i64, column: UserColumn) -> BotResult<Option<String>> {
        if let Some(pool) = &self.pool {
            let row: Option<PgRow> = sqlx::query("SELECT * FROM users WHERE id = $1")
                .bind(user_id)
                .fetch_optional(pool)
                .await?;

            Ok(row.and_then(|row| {
                match column {
                    UserColumn::Id => Some(row.get::<i64, _>("id").to_string()),
                    UserColumn::Name => Some(row.get::<String, _>("name")),
                    UserColumn::RequestsAmount => Some(row.get::<i64, _>("requests_amount").to_string()),
                    UserColumn::Timestamp => Some(row.get::<i64, _>("timestamp").to_string()),
                    UserColumn::ChatId => row.get::<Option<i64>, _>("chat_id").map(|v| v.to_string()),
                    UserColumn::RegisterTimestamp => Some(row.get::<i64, _>("register_timestamp").to_string()),
                }
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_user(&self, id: i64) -> BotResult<Option<User>> {
        if let Some(pool) = &self.pool {
            let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(id)
                .fetch_optional(pool)
                .await?;

            Ok(user)
        } else {
            Ok(None)
        }
    }

    pub async fn increment_requests(&self, id: i64) -> BotResult<()> {
        if let Some(pool) = &self.pool {
            sqlx::query(
                r#"
                    UPDATE users SET
                        requests_amount = requests_amount + 1,
                        timestamp = $1
                    WHERE id = $2
                    "#,
            )
                .bind(current_timestamp())
                .bind(id)
                .execute(pool)
                .await?;
        }
        Ok(())
    }

}