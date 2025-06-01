use sqlx::PgPool;
use crate::models::user::User;

pub async fn get_stats(telegram_id: i64, pool: &PgPool) -> Result<String, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, telegram_id, username, gold_coins, level, class, stats, nsfw_enabled
         FROM users WHERE telegram_id = $1",
        telegram_id
    )
    .fetch_one(pool)
    .await?;

    let stats_map: serde_json::Value = user.stats;
    let stats_text = if stats_map.is_object() {
        stats_map.as_object().unwrap().iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        "No stats recorded yet.".to_string()
    };

    Ok(format!(
        "🧙 Stats for {} ({}):\n\
        Level: {}\n\
        Class: {}\n\
        Gold: 💰 {}\n\
        NSFW: {}\n\
        {}\n",
        user.username,
        user.telegram_id,
        user.level,
        user.class,
        user.gold_coins,
        if user.nsfw_enabled { "Enabled" } else { "Disabled" },
        stats_text
    ))
}
