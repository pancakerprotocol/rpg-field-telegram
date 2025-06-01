use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub telegram_id: i64,
    pub username: String,
    pub gold_coins: i32,
    pub level: i32,
    pub class: String,
    pub stats: Value, // JSONB in Postgres
    pub nsfw_enabled: bool,
}
