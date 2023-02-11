use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{Message, ParseMode},
    Bot, RequestError,
};

pub(crate) trait TryReplyMessage {
    async fn try_reply(&self, message: Message, text: String) -> Result<Message, RequestError>;
}

impl TryReplyMessage for Bot {
    async fn try_reply(&self, message: Message, text: String) -> Result<Message, RequestError> {
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
}
