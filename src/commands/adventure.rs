use teloxide::prelude::*;
use teloxide::types::Message;

pub async fn adventure(message: Message, bot: AutoSend<Bot>) {
    // Adventure logic here
    bot.send_message(message.chat.id, "Embarking on an adventure...").await.unwrap();
}
