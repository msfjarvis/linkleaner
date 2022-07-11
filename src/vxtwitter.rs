use std::error::Error;

use once_cell::sync::Lazy;
use regex::Regex;
use teloxide::{
    adaptors::AutoSend,
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{Message, ParseMode},
    Bot,
};

static TWITTER_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new("^https://(mobile.)?twitter.com/.*/status/[0-9]+.*").unwrap());

static TWITTER_REGEX_LINK: Lazy<Regex> =
    Lazy::new(|| Regex::new("^https://(mobile.)?twitter.com").unwrap());

pub async fn handler(
    bot: AutoSend<Bot>,
    message: Message,
) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    if let Some(text) = message.text() && let Some(user) = message.from() && TWITTER_REGEX.is_match(text) {
        let text =  TWITTER_REGEX_LINK.replace(text, "https://vxtwitter.com");
        let text = format!(
            "<a href=\"{}\">{}</a>: {}",
            user.id.url(),
            user.full_name(),
            text
        );
        bot.delete_message(message.chat.id, message.id).await?;
        if let Some(reply) = message.reply_to_message() {
            bot.send_message(message.chat.id, text)
                .reply_to_message_id(reply.id)
                .parse_mode(ParseMode::Html)
                .await?;
        } else {
            bot.send_message(message.chat.id, text)
                .parse_mode(ParseMode::Html)
                .await?;
        }
    }
    Ok(())
}
