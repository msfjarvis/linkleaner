use crate::{message::TryReplyMessage, utils::scrub_urls};
use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    error::Error,
    sync::atomic::{AtomicBool, Ordering},
};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{ChatAction, Message},
    utils::html::link,
    Bot,
};

const HOST_MATCH_GROUP: &str = "host";

pub static MATCH_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new("https://(?P<host>(?:.*)?medium.com)/.*").unwrap());

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
                format!("Medium link replacement is {state}"),
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
                format!("Medium link replacement has been {state}"),
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
    if let Some(text) = scrub_urls(&message)
        && let Some(user) = message.from()
        && let Some(caps) = MATCH_REGEX.captures(&text)
    {
        let text = text.replace(&caps[HOST_MATCH_GROUP], "medium.rip");
        let text = format!("{}: {}", link(user.url().as_str(), &user.full_name()), text);
        let _del = bot.delete_message(message.chat.id, message.id).await;
        bot.try_reply(message, text).await?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{HOST_MATCH_GROUP, MATCH_REGEX};

    #[test]
    fn verify_regex() {
        let items = vec![
            "https://medium.com/androiddevelopers/effective-state-management-for-textfield-in-compose-d6e5b070fbe5",
            "https://androiddevelopers.medium.com/effective-state-management-for-textfield-in-compose-d6e5b070fbe5",
            "https://medium.com/@Aaron0928/30-year-old-code-killed-microsoft-rewrites-windows-kernel-with-180-000-lines-of-rust-f891c95959f2",
        ];
        for item in items {
            assert!(MATCH_REGEX.is_match(item), "{item} failed to match");
            assert!(
                MATCH_REGEX.is_match(&format!("Some leading text {item}")),
                "{item} failed to match"
            );
        }
        let caps = MATCH_REGEX
            .captures("https://medium.com/androiddevelopers/effective-state-management-for-textfield-in-compose-d6e5b070fbe5")
            .unwrap();
        assert_eq!(&caps[HOST_MATCH_GROUP], "medium.com");
        let caps = MATCH_REGEX
            .captures("https://androiddevelopers.medium.com/effective-state-management-for-textfield-in-compose-d6e5b070fbe5")
            .unwrap();
        assert_eq!(&caps[HOST_MATCH_GROUP], "androiddevelopers.medium.com");
    }
}
