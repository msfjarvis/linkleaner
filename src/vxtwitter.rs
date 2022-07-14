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

static TWITTER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new("^https://(?P<host>(?:mobile.)?twitter.com)/.*/status/[0-9]+.*").unwrap()
});

pub async fn handler(
    bot: AutoSend<Bot>,
    message: Message,
) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    if let Some(text) = message.text() && let Some(user) = message.from() &&
        TWITTER_REGEX.is_match(text) && let Some(caps) = TWITTER_REGEX.captures(text) {
        let text = text.replace(&caps[HOST_MATCH_GROUP], "vxtwitter.com");
        let text = format!(
            "<a href=\"{}\">{}</a>: {}",
            user.id.url(),
            user.full_name(),
            text
        );
        let _ = bot.delete_message(message.chat.id, message.id).await;
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
    use super::{HOST_MATCH_GROUP, TWITTER_REGEX};

    #[test]
    fn verify_regex() {
        assert!(TWITTER_REGEX.is_match("https://twitter.com/Jack/status/20"));
        assert!(TWITTER_REGEX.is_match("https://mobile.twitter.com/Jack/status/20"));
        assert!(!TWITTER_REGEX.is_match("https://twitter.com/Jack/"));
        let caps = TWITTER_REGEX
            .captures("https://twitter.com/Jack/status/20")
            .unwrap();
        assert_eq!(&caps[HOST_MATCH_GROUP], "twitter.com");
        let caps = TWITTER_REGEX
            .captures("https://mobile.twitter.com/Jack/status/20")
            .unwrap();
        assert_eq!(&caps[HOST_MATCH_GROUP], "mobile.twitter.com");
    }
}
