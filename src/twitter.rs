use crate::{message::BotExt, utils::scrub_urls};
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
const ROOT_MATCH_GROUP: &str = "root";

pub static MATCH_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new("https://(?P<host>(?:mobile.)?(?P<root>(twitter|x)).com)/.*/status/[0-9]+.*")
        .unwrap()
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
                format!("Twitter link replacement is {state}"),
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
                format!("Twitter link replacement has been {state}"),
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
        let text = match &caps[ROOT_MATCH_GROUP] {
            "twitter" => text.replace(&caps[HOST_MATCH_GROUP], "vxtwitter.com"),
            "x" => text.replace(&caps[HOST_MATCH_GROUP], "fixupx.com"),
            _ => {
                tracing::trace!("No URL match found in {text}");
                return Ok(());
            }
        };
        let text = format!("{}: {}", link(user.url().as_str(), &user.full_name()), text);
        let _del = bot.delete_message(message.chat.id, message.id).await;
        bot.try_reply(message, text).await?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{HOST_MATCH_GROUP, MATCH_REGEX, ROOT_MATCH_GROUP};

    #[test]
    fn verify_regex() {
        let hosts = [
            ("mobile.twitter.com", "twitter"),
            ("twitter.com", "twitter"),
            ("mobile.x.com", "x"),
            ("x.com", "x"),
        ];
        for (host, root) in hosts {
            let url = format!("https://{host}/Jack/status/20");
            assert!(MATCH_REGEX.is_match(&url), "{url} failed to match");
            assert!(
                MATCH_REGEX.is_match(&format!("Some leading text {url}")),
                "{url} failed to match"
            );
            assert!(!MATCH_REGEX.is_match(&format!("https://{host}/Jack/")));
            let caps = MATCH_REGEX.captures(&url).unwrap();
            assert_eq!(&caps[HOST_MATCH_GROUP], host);
            assert_eq!(&caps[ROOT_MATCH_GROUP], root);
        }
    }
}
