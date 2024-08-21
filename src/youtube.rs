use crate::{
    message::BotExt,
    utils::{scrub_urls, AsyncError},
};
use regex::Regex;
use std::sync::LazyLock;
use teloxide::{types::Message, utils::html::link, Bot};

pub const DOMAINS: [&str; 1] = ["youtube.com"];
static MATCH_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("https://(?:www.)?youtube.com/(?P<shorts>shorts/)[A-Za-z0-9-_]{11}.*").unwrap()
});

pub async fn handler(bot: Bot, message: Message) -> Result<(), AsyncError> {
    if let Some(text) = scrub_urls(&message)
        && let Some(ref user) = message.from
        && let Some(caps) = MATCH_REGEX.captures(&text)
    {
        let text = text.replace(&caps["shorts"], "watch?v=");
        let text = format!("{}: {}", link(user.url().as_str(), &user.full_name()), text);
        bot.replace_chat_message(&message, &text).await?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::MATCH_REGEX;

    #[test]
    fn verify_regex() {
        let items = vec![
            "https://www.youtube.com/shorts/SqjNixegPKk",
            "https://www.youtube.com/shorts/SqjNixegPKk?feature=share",
            "https://youtube.com/shorts/SqjNixegPKk",
            "https://youtube.com/shorts/JY55-UBtlf8?feature=share",
            "https://youtube.com/shorts/afHFjnPy_vk?feature=share",
        ];
        for item in items {
            assert!(MATCH_REGEX.is_match(item), "{item} failed to match");
            assert!(
                MATCH_REGEX.is_match(&format!("Some leading text {item}")),
                "{item} failed to match"
            );
        }
        assert!(!MATCH_REGEX.is_match("https://youtube.com/watch?v=SqjNixegPKk"));
    }
}
