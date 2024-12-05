use crate::{
    message::BotExt,
    utils::{get_preview_url, scrub_urls, AsyncError},
};
use regex::Regex;
use reqwest::Url;
use std::sync::LazyLock;
use teloxide::{types::Message, utils::html::link, Bot};
use tracing::trace;

const HOST_MATCH_GROUP: &str = "host";
const ROOT_MATCH_GROUP: &str = "root";

pub const DOMAINS: [&str; 4] = ["twitter.com", "mobile.twitter.com", "x.com", "mobile.x.com"];
static MATCH_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("https://(?P<host>(?:mobile.)?(?P<root>(twitter|x)).com)/.*/status/[0-9]+.*")
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
            |msg| match &caps[ROOT_MATCH_GROUP] {
                "twitter" => get_preview_url(msg, &caps[HOST_MATCH_GROUP], "vxtwitter.com"),
                "x" => get_preview_url(msg, &caps[HOST_MATCH_GROUP], "fixupx.com"),
                _ => {
                    trace!("No URL match found in {text}");
                    None
                }
            },
            |msg| {
                if let Some(url) = get_preview_url(msg, &caps[HOST_MATCH_GROUP], "xcancel.com")
                    && let Ok(url) = Url::parse(url.as_str())
                {
                    Some(("View on Nitter", url))
                } else {
                    None
                }
            },
        )
        .await?;
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
