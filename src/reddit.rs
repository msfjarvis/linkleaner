use crate::{
    bot_ext::BotExt,
    router_ext::add_route,
    url::{get_preview_url, get_urls_from_message, scrub_urls},
    AsyncError,
};
use matchit::Router;
use std::sync::LazyLock;
use teloxide::{types::Message, utils::html::link, Bot};
use url::Host;

static URL_MATCHER: LazyLock<Router<()>> = LazyLock::new(|| {
    let mut router = Router::new();
    add_route!(router, "/r/{subreddit}/comments/{id}/{slug}/{comment}");
    add_route!(router, "/r/{subreddit}/comments/{id}/{slug}");
    add_route!(router, "/r/{subreddit}/comments/{id}");
    add_route!(router, "/r/{subreddit}/s/{id}");
    add_route!(router, "/{id}");
    router
});

pub const DOMAINS: [&str; 3] = ["reddit.com", "redd.it", "www.reddit.com"];

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
            |msg| get_preview_url(msg, domain, "rxddit.com"),
            |_| None,
        )
        .await?;
    };
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_url_matcher() {
        crate::url::verify_url_matcher(
            &[
                "https://reddit.com/r/shittymoviedetails/comments/160onpq/breaking_actor_from_home_alone_2_arrested_today/jxnkq4g",
                "https://reddit.com/r/shittymoviedetails/comments/160onpq/breaking_actor_from_home_alone_2_arrested_today",
                "https://reddit.com/r/shittymoviedetails/comments/160onpq",
                "https://reddit.com/r/MemePiece/s/15w6vzg82W",
                "https://reddit.com/160onpq",
                "https://redd.it/160onpq",
                "https://www.reddit.com/r/VALORANT/s/MTu577P105",
            ],
            &super::URL_MATCHER,
        );
    }
}
