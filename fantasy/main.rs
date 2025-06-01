#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let bot = Bot::from_env();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&db_url).await.unwrap();

    teloxide::repl(bot, move |msg| {
        let pool = pool.clone();
        async move {
            let text = msg.update.text().unwrap_or("");
            if text == "/slots" {
                let result = play_slots(msg.update.chat_id(), &pool).await;
                msg.answer(result).await?;
            } else if text.starts_with("/buy ") {
                let item = text.trim_start_matches("/buy ").trim();
                let result = buy_item(msg.update.chat_id(), item, &pool).await;
                msg.answer(result).await?;
            } // Add other commands
            Ok(())
        }
    }).await;
}
