use crate::{
    AsyncError,
    bot_ext::BotExt,
    url::{get_preview_url, get_urls_from_message, scrub_urls},
};
use matchit::Router;
use std::sync::LazyLock;
use teloxide::{Bot, types::Message, utils::html::link};
use url::Host;

pub const DOMAINS: [&str; 2] = ["instagram.com", "www.instagram.com"];
static URL_MATCHER: LazyLock<Router<()>> = LazyLock::new(|| {
    let mut router = Router::new();
    router.insert("/p/{id}/", ()).unwrap();
    router.insert("/reel/{id}/", ()).unwrap();
    router.insert("/tv/{id}/", ()).unwrap();
    router.insert("/{username}/reel/{id}/", ()).unwrap();
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
    };
    Ok(())
}

#[cfg(test)]
mod test {
    const URLS: [&str; 5] = [
        "https://www.instagram.com/p/CgJESh6hxsS/",
        "https://instagram.com/p/CgJESh6hxsS/",
        "https://www.instagram.com/reel/CgHIG0Ih3XF/",
        "https://www.instagram.com/tv/CgHIG0Ih3XF/",
        "https://www.instagram.com/zuck/reel/C9AbmIYp3Js/",
    ];

    #[test]
    fn test_url_matcher() {
        crate::url::verify_url_matcher(&URLS, &super::URL_MATCHER);
    }
}
