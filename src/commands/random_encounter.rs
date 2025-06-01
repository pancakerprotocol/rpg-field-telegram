use teloxide::prelude::*;
use teloxide::types::Message;

pub async fn random_encounter(message: Message, bot: AutoSend<Bot>) {
    // Random encounter logic here
    bot.send_message(message.chat.id, "A wild encounter appears!").await.unwrap();
}
