use crate::{
    message::BotExt,
    utils::{get_preview_url, scrub_urls, AsyncError},
};
use regex::Regex;
use std::sync::LazyLock;
use teloxide::{types::Message, utils::html::link, Bot};

const HOST_MATCH_GROUP: &str = "host";
pub const DOMAINS: [&str; 1] = ["instagram.com"];
static MATCH_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("https://(?:www.)?(?P<host>instagram.com)/(.*/)?(p|reel|tv)/[A-Za-z0-9]+.*/")
        .unwrap()
});

pub async fn handler(bot: Bot, message: Message) -> Result<(), AsyncError> {
    if let Some(text) = scrub_urls(&message)
        && let Some(ref user) = message.from
        && let Some(caps) = MATCH_REGEX.captures(&text)
        && !bot.is_self_message(&message)
    {
        let text = format!("{}: {}", link(user.url().as_str(), &user.full_name()), text);
        bot.send_preview(
            &message,
            &text,
            |msg| get_preview_url(msg, &caps[HOST_MATCH_GROUP], "ddinstagram.com"),
            |_| None,
        )
        .await?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{HOST_MATCH_GROUP, MATCH_REGEX};

    // https://regex101.com/r/pOizaT/1
    #[test]
    fn verify_regex() {
        let items = vec![
            "https://www.instagram.com/p/CgJESh6hxsS/",
            "https://instagram.com/p/CgJESh6hxsS/",
            "https://www.instagram.com/reel/CgHIG0Ih3XF/",
            "https://www.instagram.com/tv/CgHIG0Ih3XF/",
            "https://www.instagram.com/zuck/reel/C9AbmIYp3Js/",
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
