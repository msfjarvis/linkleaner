use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{ChatAction, Message, ParseMode},
    Bot, RequestError,
};

pub(crate) trait BotExt {
    async fn try_reply(&self, message: &Message, text: &str) -> Result<Message, RequestError>;
    async fn replace_chat_message(
        &self,
        message: &Message,
        text: &str,
    ) -> Result<Message, RequestError>;
}

impl BotExt for Bot {
    async fn try_reply(&self, message: &Message, text: &str) -> Result<Message, RequestError> {
        self.send_chat_action(message.chat.id, ChatAction::Typing)
            .await?;
        if let Some(reply) = message.reply_to_message() {
            self.send_message(message.chat.id, text)
                .reply_to_message_id(reply.id)
                .parse_mode(ParseMode::Html)
                .await
        } else {
            self.send_message(message.chat.id, text)
                .parse_mode(ParseMode::Html)
                .await
        }
    }

    async fn replace_chat_message(
        &self,
        message: &Message,
        text: &str,
    ) -> Result<Message, RequestError> {
        let _del = self.delete_message(message.chat.id, message.id).await;
        self.try_reply(message, text).await
    }
}
