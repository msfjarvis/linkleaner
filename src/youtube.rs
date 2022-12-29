use crate::utils::scrub_urls;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    error::Error,
    sync::atomic::{AtomicBool, Ordering},
};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{ChatAction, Message, ParseMode},
    utils::html::link,
    Bot,
};

pub static MATCH_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new("^https://(?:www.)?youtube.com/(?P<shorts>shorts/)[A-Za-z0-9]{11}.*").unwrap()
});

pub static FILTER_ENABLED: AtomicBool = AtomicBool::new(true);

pub async fn set_filter_state(
    bot: Bot,
    message: Message,
    filter_state: Option<bool>,
) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    match filter_state {
        None => {
            let state = if FILTER_ENABLED.load(Ordering::Relaxed) {
                "enabled"
            } else {
                "disabled"
            };
            bot.send_chat_action(message.chat.id, ChatAction::Typing)
                .await?;
            bot.send_message(
                message.chat.id,
                format!("YouTube link replacement is {state}"),
            )
            .reply_to_message_id(message.id)
            .await?;
        }
        Some(state) => {
            FILTER_ENABLED.store(state, Ordering::Relaxed);
            let state = if state { "enabled" } else { "disabled" };
            bot.send_chat_action(message.chat.id, ChatAction::Typing)
                .await?;
            bot.send_message(
                message.chat.id,
                format!("YouTube link replacement has been {state}"),
            )
            .reply_to_message_id(message.id)
            .await?;
        }
    };
    Ok(())
}

pub async fn handler(
    bot: Bot,
    message: Message,
) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    if let Some(text) = scrub_urls(&message) && let Some(user) = message.from() &&
        let Some(caps) = MATCH_REGEX.captures(&text) {
        let text = text.replace(&caps["shorts"], "watch?v=");
        let text = format!(
            "{}: {}",
            link(user.url().as_str(), &user.full_name()),
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
    use super::MATCH_REGEX;

    #[test]
    fn verify_regex() {
        assert!(MATCH_REGEX.is_match("https://www.youtube.com/shorts/SqjNixegPKk"));
        assert!(MATCH_REGEX.is_match("https://www.youtube.com/shorts/SqjNixegPKk?feature=share"));
        assert!(MATCH_REGEX.is_match("https://youtube.com/shorts/SqjNixegPKk"));
        assert!(!MATCH_REGEX.is_match("https://youtube.com/watch?v=SqjNixegPKk"));
    }
}
