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

const HOST_MATCH_GROUP: &str = "host";

pub static MATCH_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new("^https://(?:www.)?(?P<host>instagram.com)/(p|reel)/[A-Za-z0-9]+.*/").unwrap()
});

pub async fn handler(
    bot: AutoSend<Bot>,
    message: Message,
) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    if let Some(text) = message.text() && let Some(user) = message.from() &&
        let Some(caps) = MATCH_REGEX.captures(text) {
        let text = text.replace(&caps[HOST_MATCH_GROUP], "ddinstagram.com");
        let text = format!(
            "<a href=\"{}\">{}</a>: {}",
            user.id.url(),
            user.full_name(),
            text
        );
        let _del = bot.delete_message(message.chat.id, message.id).await;
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

#[cfg(test)]
mod test {
    use super::{HOST_MATCH_GROUP, MATCH_REGEX};

    #[test]
    fn verify_regex() {
        assert!(MATCH_REGEX.is_match("https://www.instagram.com/p/CgJESh6hxsS/"));
        assert!(MATCH_REGEX.is_match("https://instagram.com/p/CgJESh6hxsS/"));
        assert!(MATCH_REGEX.is_match("https://www.instagram.com/reel/CgHIG0Ih3XF/"));
        assert!(!MATCH_REGEX.is_match("https://www.instagram.com/starsmitten_/"));
        let caps = MATCH_REGEX
            .captures("https://www.instagram.com/p/CfZdFVUJyQG/")
            .unwrap();
        assert_eq!(&caps[HOST_MATCH_GROUP], "instagram.com");
    }
}
