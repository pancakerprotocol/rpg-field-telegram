pub async fn buy_item(user_id: i64, item_name: &str, pool: &PgPool) -> String {
    // Fetch item price
    // Check user balance
    // Insert into user_items
    // Update gold balance
    format!("You bought a {}!", item_name)
}
