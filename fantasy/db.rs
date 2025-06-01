pub async fn get_or_create_user(telegram_id: i64, username: &str, pool: &PgPool) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE telegram_id = $1", telegram_id)
        .fetch_optional(pool)
        .await?;

    if let Some(u) = user {
        Ok(u)
    } else {
        let id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO users (id, telegram_id, username) VALUES ($1, $2, $3)",
            id,
            telegram_id,
            username
        ).execute(pool).await?;
        Ok(User {
            id,
            telegram_id,
            username: username.to_string(),
            gold_coins: 100,
            level: 1,
            class: "warrior".into(),
            stats: serde_json::json!({}),
            nsfw_enabled: false
        })
    }
}
