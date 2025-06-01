use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub telegram_id: i64,
    pub username: String,
    pub gold_coins: i64,
    pub level: i64,
    pub class: String,
    pub hp: i64,
    pub energy: i64,
    pub mana: i64,
    pub pets: Vec<String>,
    pub conditions: Vec<String>,
    pub achievements: Vec<String>,
    pub artifacts: Vec<String>,
}
