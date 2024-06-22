use crate::{
    message::BotExt,
    utils::{scrub_urls, AsyncError},
};
use once_cell::sync::Lazy;
use regex::Regex;
use teloxide::{types::Message, utils::html::link, Bot};

const HOST_MATCH_GROUP: &str = "host";

pub const DOMAINS: [&str; 1] = ["instagram.com"];
static MATCH_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new("https://(?:www.)?(?P<host>instagram.com)/(p|reel|tv)/[A-Za-z0-9]+.*/").unwrap()
});

pub async fn handler(bot: Bot, message: Message) -> Result<(), AsyncError> {
    if let Some(text) = scrub_urls(&message)
        && let Some(user) = message.from()
        && let Some(caps) = MATCH_REGEX.captures(&text)
    {
        let text = text.replace(&caps[HOST_MATCH_GROUP], "ddinstagram.com");
        let text = format!("{}: {}", link(user.url().as_str(), &user.full_name()), text);
        bot.replace_chat_message(&message, &text).await?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{HOST_MATCH_GROUP, MATCH_REGEX};

    #[test]
    fn verify_regex() {
        let items = vec![
            "https://www.instagram.com/p/CgJESh6hxsS/",
            "https://instagram.com/p/CgJESh6hxsS/",
            "https://www.instagram.com/reel/CgHIG0Ih3XF/",
            "https://www.instagram.com/tv/CgHIG0Ih3XF/",
        ];
        for item in items {
            assert!(MATCH_REGEX.is_match(item), "{item} failed to match");
            assert!(
                MATCH_REGEX.is_match(&format!("Some leading text {item}")),
                "{item} failed to match"
            );
        }
        assert!(!MATCH_REGEX.is_match("https://www.instagram.com/starsmitten_/"));
        let caps = MATCH_REGEX
            .captures("https://www.instagram.com/p/CfZdFVUJyQG/")
            .unwrap();
        assert_eq!(&caps[HOST_MATCH_GROUP], "instagram.com");
    }
}
