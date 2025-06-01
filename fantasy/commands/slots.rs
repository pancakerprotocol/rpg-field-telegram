pub async fn play_slots(user_id: i64, pool: &PgPool) -> String {
    let symbols = ["🍒", "🍋", "🍉", "7️⃣"];
    let spin: Vec<_> = (0..3).map(|_| symbols[rand::random::<usize>() % symbols.len()]).collect();
    let win = spin.iter().all(|&x| x == spin[0]);
    let coins = if win { 50 } else { -10 };

    // update coins
    sqlx::query!(
        "UPDATE users SET gold_coins = gold_coins + $1 WHERE telegram_id = $2",
        coins,
        user_id
    ).execute(pool).await.unwrap();

    format!("🎰 {} {} {}\n{}", spin[0], spin[1], spin[2], if win { "You won 50 coins!" } else { "You lost 10 coins." })
}
