use teloxide::prelude::*;
use teloxide::types::Message;

pub async fn revive(message: Message, bot: AutoSend<Bot>) {
    // Revive logic here
    bot.send_message(message.chat.id, "You have been revived!").await.unwrap();
}
