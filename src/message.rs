use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{ChatAction, Message, ParseMode, ReplyParameters},
    Bot, RequestError,
};

pub(crate) trait BotExt {
    async fn reply(&self, message: &Message, text: &str) -> Result<Message, RequestError>;
    async fn try_reply(&self, message: &Message, text: &str) -> Result<Message, RequestError>;
    async fn try_reply_silent(
        &self,
        message: &Message,
        text: &str,
    ) -> Result<Message, RequestError>;
    async fn replace_chat_message(
        &self,
        message: &Message,
        text: &str,
    ) -> Result<Message, RequestError>;
}

impl BotExt for Bot {
    async fn reply(&self, message: &Message, text: &str) -> Result<Message, RequestError> {
        self.send_message(message.chat.id, text)
            .reply_parameters(ReplyParameters::new(message.id))
            .parse_mode(ParseMode::Html)
            .await
    }

    async fn try_reply(&self, message: &Message, text: &str) -> Result<Message, RequestError> {
        self.send_chat_action(message.chat.id, ChatAction::Typing)
            .await?;
        self.try_reply_silent(message, text).await
    }

    async fn try_reply_silent(
        &self,
        message: &Message,
        text: &str,
    ) -> Result<Message, RequestError> {
        if let Some(reply) = message.reply_to_message() {
            self.send_message(message.chat.id, text)
                .reply_parameters(ReplyParameters::new(reply.id))
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
        self.try_reply_silent(message, text).await
    }
}
