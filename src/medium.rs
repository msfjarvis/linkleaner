use crate::{
    message::BotExt,
    utils::{scrub_urls, AsyncError},
};
use once_cell::sync::Lazy;
use regex::Regex;
use teloxide::{prelude::Requester, types::Message, utils::html::link, Bot};

#[allow(dead_code)] // This is used in the tests
const HOST_MATCH_GROUP: &str = "host";
const PATH_MATCH_GROUP: &str = "path";
const USER_MATCH_GROUP: &str = "user";

pub static MATCH_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new("https://(?<user>[a-zA-Z0-9]*)?.?(?<host>medium.com)/(?<path>.*)").unwrap()
});

pub async fn handler(bot: Bot, message: Message) -> Result<(), AsyncError> {
    if let Some(text) = scrub_urls(&message)
        && let Some(user) = message.from()
        && let Some(caps) = MATCH_REGEX.captures(&text)
        && let Some(full_url) = caps.get(0)
    {
        let text = text.replace(full_url.as_str(), &build_url(&caps));
        let text = format!("{}: {}", link(user.url().as_str(), &user.full_name()), text);
        let _del = bot.delete_message(message.chat.id, message.id).await;
        bot.try_reply(&message, text).await?;
    }
    Ok(())
}

fn build_url(caps: &regex::Captures) -> String {
    let mut url = format!(
        "{}/{}/{}",
        "md.vern.cc", &caps[USER_MATCH_GROUP], &caps[PATH_MATCH_GROUP]
    )
    // Easier to replace them here than build logic to avoid it in the first place :D
    .replace("//", "/");
    url.insert_str(0, "https://");
    url
}

#[cfg(test)]
mod test {
    use super::{build_url, HOST_MATCH_GROUP, MATCH_REGEX, PATH_MATCH_GROUP, USER_MATCH_GROUP};

    const URLS: [&str; 3] = [
        "https://medium.com/androiddevelopers/effective-state-management-for-textfield-in-compose-d6e5b070fbe5",
        "https://androiddevelopers.medium.com/effective-state-management-for-textfield-in-compose-d6e5b070fbe5",
        "https://medium.com/@Aaron0928/30-year-old-code-killed-microsoft-rewrites-windows-kernel-with-180-000-lines-of-rust-f891c95959f2",
    ];

    #[test]
    fn verify_builder() {
        let caps = MATCH_REGEX.captures(URLS[0]).unwrap();
        assert_eq!(build_url(&caps), "https://md.vern.cc/androiddevelopers/effective-state-management-for-textfield-in-compose-d6e5b070fbe5");
        let caps = MATCH_REGEX.captures(URLS[1]).unwrap();
        assert_eq!(build_url(&caps), "https://md.vern.cc/androiddevelopers/effective-state-management-for-textfield-in-compose-d6e5b070fbe5");
        let caps = MATCH_REGEX.captures(URLS[2]).unwrap();
        assert_eq!(build_url(&caps), "https://md.vern.cc/@Aaron0928/30-year-old-code-killed-microsoft-rewrites-windows-kernel-with-180-000-lines-of-rust-f891c95959f2");
    }

    #[test]
    fn verify_regex() {
        for item in URLS {
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
        assert_eq!(&caps[USER_MATCH_GROUP], "");
        assert_eq!(
            &caps[PATH_MATCH_GROUP],
            "androiddevelopers/effective-state-management-for-textfield-in-compose-d6e5b070fbe5"
        );
        let caps = MATCH_REGEX
            .captures("https://androiddevelopers.medium.com/effective-state-management-for-textfield-in-compose-d6e5b070fbe5")
            .unwrap();
        assert_eq!(&caps[HOST_MATCH_GROUP], "medium.com");
        assert_eq!(&caps[USER_MATCH_GROUP], "androiddevelopers");
        assert_eq!(
            &caps[PATH_MATCH_GROUP],
            "effective-state-management-for-textfield-in-compose-d6e5b070fbe5"
        );
    }
}
