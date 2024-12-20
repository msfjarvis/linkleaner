use crate::{
    message::BotExt,
    utils::{get_preview_url, get_urls_from_message, scrub_urls, AsyncError},
};
use matchit::Router;
use std::sync::LazyLock;
use teloxide::{types::Message, utils::html::link, Bot};
use url::Host;

static URL_MATCHER: LazyLock<Router<()>> = LazyLock::new(|| {
    let mut router = Router::new();
    router
        .insert("/r/{subreddit}/comments/{id}/{slug}/{comment}", ())
        .unwrap();
    router
        .insert("/r/{subreddit}/comments/{id}/{slug}", ())
        .unwrap();
    router.insert("/r/{subreddit}/comments/{id}", ()).unwrap();
    router.insert("/r/{subreddit}/s/{id}", ()).unwrap();
    router.insert("/{id}", ()).unwrap();
    router
});

pub const DOMAINS: [&str; 3] = ["reddit.com", "redd.it", "www.reddit.com"];

pub async fn handler(bot: Bot, message: Message) -> Result<(), AsyncError> {
    let urls = get_urls_from_message(&message);
    if let Some(text) = scrub_urls(&message)
        && let Some(ref user) = message.from
        && !bot.is_self_message(&message)
        && !urls.is_empty()
        && let Some(url) = urls.first()
        && let Some(host) = url.host()
        && let Host::Domain(domain) = host
        && DOMAINS.contains(&domain)
        && let Ok(_) = URL_MATCHER.at(url.path())
    {
        let text = format!("{}: {}", link(user.url().as_str(), &user.full_name()), text);
        bot.send_preview(
            &message,
            &text,
            |msg| get_preview_url(msg, domain, "rxddit.com"),
            |_| None,
        )
        .await?;
    };
    Ok(())
}

#[cfg(test)]
mod test {
    use url::Url;
    const URLS: [&str; 7] = [
        "https://reddit.com/r/shittymoviedetails/comments/160onpq/breaking_actor_from_home_alone_2_arrested_today/jxnkq4g",
        "https://reddit.com/r/shittymoviedetails/comments/160onpq/breaking_actor_from_home_alone_2_arrested_today",
        "https://reddit.com/r/shittymoviedetails/comments/160onpq",
        "https://reddit.com/r/MemePiece/s/15w6vzg82W",
        "https://reddit.com/160onpq",
        "https://redd.it/160onpq",
        "https://www.reddit.com/r/VALORANT/s/MTu577P105",
    ];

    #[test]
    fn test_url_matcher() {
        URLS.iter()
            .flat_map(|url| Url::parse(url))
            .map(|url| url.path().to_string())
            .for_each(|path| {
                assert!(
                    super::URL_MATCHER.at(&path).is_ok(),
                    "Failed to match URL: {path}"
                );
            });
    }
}
