use crate::{
    AsyncError,
    bot_ext::BotExt,
    url::{get_preview_url, get_urls_from_message, scrub_urls},
};
use matchit::Router;
use std::sync::LazyLock;
use teloxide::{Bot, types::Message, utils::html::link};
use url::Host;

pub const DOMAINS: [&str; 4] = ["twitter.com", "mobile.twitter.com", "x.com", "mobile.x.com"];
static URL_MATCHER: LazyLock<Router<()>> = LazyLock::new(|| {
    let mut router = Router::new();
    router.insert("/{user}/status/{tweet_id}", ()).unwrap();
    router
});

pub async fn handler(bot: Bot, message: Message) -> Result<(), AsyncError> {
    let urls = get_urls_from_message(&message);
    if !bot.is_self_message(&message)
        && let Some(text) = scrub_urls(&message)
        && let Some(ref user) = message.from
        && let Some(url) = urls.first()
        && let Some(host) = url.host()
        && let Host::Domain(domain) = host
        && let Ok(_) = URL_MATCHER.at(url.path())
    {
        let text = format!("{}: {}", link(user.url().as_str(), &user.full_name()), text);
        bot.send_preview(
            &message,
            &text,
            |msg| get_preview_url(msg, domain, "fixupx.com"),
            |_| {
                let mut button_url = url.clone();
                button_url.set_host(Some("xcancel.com")).unwrap();
                Some(("View on Nitter", button_url))
            },
        )
        .await?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    const URLS: [&str; 4] = [
        "https://mobile.twitter.com/Jack/status/20",
        "https://twitter.com/Jack/status/20",
        "https://mobile.x.com/Jack/status/20",
        "https://x.com/Jack/status/20",
    ];

    #[test]
    fn test_url_matcher() {
        crate::url::verify_url_matcher(&URLS, &super::URL_MATCHER);
    }
}
