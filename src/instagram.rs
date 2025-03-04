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

pub const DOMAINS: [&str; 2] = ["instagram.com", "www.instagram.com"];
static URL_MATCHER: LazyLock<Router<()>> = LazyLock::new(|| {
    let mut router = Router::new();
    add_route!(router, "/p/{id}");
    add_route!(router, "/reel/{id}");
    add_route!(router, "/tv/{id}");
    add_route!(router, "/{username}/reel/{id}");
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
            |msg| get_preview_url(msg, domain, "ddinstagram.com"),
            |_| None,
        )
        .await?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_url_matcher() {
        crate::url::verify_url_matcher(
            &[
                "https://www.instagram.com/p/CgJESh6hxsS",
                "https://instagram.com/p/CgJESh6hxsS",
                "https://www.instagram.com/reel/CgHIG0Ih3XF",
                "https://www.instagram.com/tv/CgHIG0Ih3XF",
                "https://www.instagram.com/zuck/reel/C9AbmIYp3Js",
            ],
            &super::URL_MATCHER,
        );
    }
}
