pub async fn toggle_nsfw(user_id: i64, enable: bool, pool: &PgPool) {
    sqlx::query!(
        "UPDATE users SET nsfw_enabled = $1 WHERE telegram_id = $2",
        enable,
        user_id
    ).execute(pool).await.unwrap();
}
