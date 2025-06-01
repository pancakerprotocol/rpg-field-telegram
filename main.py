use std::sync::Arc;
use rand::seq::SliceRandom;
use teloxide::{prelude::*, utils::command::BotCommands};
use sqlx::{SqlitePool, Row};

const START_COINS: i32 = 100;
const SPIN_COST: i32 = 10;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
enum Command {
    #[command(description = "Start playing.")]
    Start,
    #[command(description = "Check your balance.")]
    Balance,
    #[command(description = "Spin the slot machine.")]
    Spin,
    #[command(description = "Show this help message.")]
    Help,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting Telegram Slots Bot with SQLite...");

    // Connect to SQLite DB (file: slots.db)
    let db_url = "sqlite://slots.db";
    let pool = SqlitePool::connect(db_url).await.expect("Failed to connect to DB");

    // Create table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            user_id INTEGER PRIMARY KEY,
            coins INTEGER NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create users table");

    let bot = Bot::from_env();
    let pool = Arc::new(pool);

    Dispatcher::builder(bot, move |cx: UpdateWithCx<AutoSend<Bot>, Message>| {
        let pool = pool.clone();
        async move {
            if let Some(text) = cx.update.text() {
                if let Ok(cmd) = Command::parse(text, "slots_bot") {
                    handle_command(cx, cmd, pool).await;
                }
            }
        }
    })
    .build()
    .dispatch()
    .await;
}

async fn get_coins(pool: &SqlitePool, user_id: i64) -> i32 {
    let row = sqlx::query("SELECT coins FROM users WHERE user_id = ?")
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .expect("DB query failed");

    if let Some(row) = row {
        row.get("coins")
    } else {
        START_COINS
    }
}

async fn set_coins(pool: &SqlitePool, user_id: i64, coins: i32) {
    sqlx::query(
        r#"
        INSERT INTO users (user_id, coins)
        VALUES (?, ?)
        ON CONFLICT(user_id) DO UPDATE SET coins = excluded.coins
        "#,
    )
    .bind(user_id)
    .bind(coins)
    .execute(pool)
    .await
    .expect("DB insert/update failed");
}

async fn handle_command(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
    pool: Arc<SqlitePool>,
) {
    let user_id = cx.update.from().unwrap().id.0 as i64;

    match command {
        Command::Start => {
            let coins = get_coins(&pool, user_id).await;
            if coins == START_COINS {
                // New user, insert
                set_coins(&pool, user_id, START_COINS).await;
            }
            cx.reply_to(format!("🎰 Welcome! You have {} coins.", coins))
                .await
                .unwrap();
        }
        Command::Balance => {
            let coins = get_coins(&pool, user_id).await;
            cx.reply_to(format!("💰 You have {} coins.", coins)).await.unwrap();
        }
        Command::Spin => {
            let mut coins = get_coins(&pool, user_id).await;

            if coins < SPIN_COST {
                cx.reply_to("❌ Not enough coins to spin!").await.unwrap();
                return;
            }
            coins -= SPIN_COST;

            let symbols = ["🍒", "🍋", "🔔", "💎", "7️⃣"];
            let mut rng = rand::thread_rng();
            let spin: Vec<_> = (0..3)
                .map(|_| *symbols.choose(&mut rng).unwrap())
                .collect();

            let result = format!("🎰 {} {} {}", spin[0], spin[1], spin[2]);

            let winnings_msg;
            if spin.iter().all(|&s| s == spin[0]) {
                coins += 50;
                winnings_msg = "🎉 Jackpot! You won 50 coins!";
            } else if spin[0] == spin[1] || spin[1] == spin[2] || spin[0] == spin[2] {
                coins += 20;
                winnings_msg = "🥳 You got a pair! You win 20 coins!";
            } else {
                winnings_msg = "🙃 No match. Better luck next time!";
            }

            set_coins(&pool, user_id, coins).await;

            cx.reply_to(format!("{result}\n{winnings_msg}\nBalance: {} coins", coins))
                .await
                .unwrap();
        }
        Command::Help => {
            cx.reply_to(Command::descriptions()).await.unwrap();
        }
    }
}
