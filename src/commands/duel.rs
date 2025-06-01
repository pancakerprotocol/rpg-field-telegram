use teloxide::prelude::*;
use teloxide::types::Message;

pub async fn duel(message: Message, bot: AutoSend<Bot>) {
    // Duel logic here
    bot.send_message(message.chat.id, "Challenge issued!").await.unwrap();
}
