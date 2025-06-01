pub async fn start_adventure(user: &User, difficulty: Option<String>, pool: &PgPool) -> String {
    let actual_difficulty = match difficulty {
        Some(d) => d,
        None => {
            if user.level < 5 { "easy" } else if user.level < 10 { "medium" } else { "hard" }.to_string()
        }
    };

    // Roll outcome based on stats
    // Return rewards or penalties
    format!("You went on a {} adventure!", actual_difficulty)
}
