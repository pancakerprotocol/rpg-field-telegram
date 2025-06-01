use teloxide::{prelude::*, utils::command::BotCommands};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;

mod db;
mod commands;
mod models;

use commands::{
    slots::play_slots,
    shop::buy_item,
    stats::get_stats,
};

use db::get_or_create_user;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
enum Command {
    #[command(description = "Play the slot machine.")]
    Slots,
    #[command(description = "Buy an item from the shop. Usage: /buy <item_name>")]
    Buy(String),
    #[command(description = "View your character stats.")]
    Stats,
    #[command(description = "Show this help message.")]
    Help,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let bot = Bot::from_env();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let pool = PgPool::connect(&db_url).await.expect("Failed to connect to DB");

    teloxide::commands_repl(bot, move |cx, command| {
        let pool = pool.clone();
        async move {
            let user_id = cx.update.chat.id.0;
            let username = cx.update.from().map(|u| u.username.clone().unwrap_or("Unknown".into())).unwrap_or("Unknown".into());

            // Ensure user exists
            let user = get_or_create_user(user_id, &username, &pool).await.unwrap();

            match command {
                Command::Slots => {
                    let response = play_slots(user.telegram_id, &pool).await;
                    cx.answer(response).await?
                }
                Command::Buy(item_name) => {
                    let response = buy_item(user.telegram_id, &item_name, &pool).await;
                    cx.answer(response).await?
                }
                Command::Stats => {
                    let response = get_stats(user.telegram_id, &pool).await.unwrap_or_else(|e| format!("Error getting stats: {}", e));
                    cx.answer(response).await?
                }
                Command::Help => {
                    cx.answer(Command::descriptions()).await?
                }
            }

            Ok(())
        }
    }, Command::ty()).await;
}
