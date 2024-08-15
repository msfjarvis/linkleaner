use crate::{
    message::BotExt,
    utils::{scrub_urls, AsyncError},
};
use once_cell::sync::Lazy;
use regex::Regex;
use teloxide::{types::Message, utils::html::link, Bot};

const HOST_MATCH_GROUP: &str = "host";
const ROOT_MATCH_GROUP: &str = "root";

pub const DOMAINS: [&str; 2] = ["twitter.com", "x.com"];
static MATCH_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new("https://(?P<host>(?:mobile.)?(?P<root>(twitter|x)).com)/.*/status/[0-9]+.*")
        .unwrap()
});

pub async fn handler(bot: Bot, message: Message) -> Result<(), AsyncError> {
    if let Some(text) = scrub_urls(&message)
        && let Some(ref user) = message.from
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
        bot.replace_chat_message(&message, &text).await?;
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
