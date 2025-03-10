use crate::{AsyncError, bot_ext::BotExt, router_ext::add_route, url::get_urls_from_message};
use matchit::Router;
use std::sync::LazyLock;
use teloxide::{Bot, types::Message, utils::html::link};
use url::Host;

pub const DOMAINS: [&str; 2] = ["youtube.com", "www.youtube.com"];
static URL_MATCHER: LazyLock<Router<()>> = LazyLock::new(|| {
    let mut router = Router::new();
    add_route!(router, "/shorts/{id}");
    router
});

pub async fn handler(bot: Bot, message: Message) -> Result<(), AsyncError> {
    let urls = get_urls_from_message(&message);
    if !bot.is_self_message(&message)
        // This would usually use `scrub_urls` but we reconstruct it manually later
        // and we need the original text to replace the URL.
        && let Some(text) = message.text()
        && let Some(ref user) = message.from
        && let Some(url) = urls.first()
        && let Some(host) = url.host()
        && let Host::Domain(domain) = host
        && let Ok(result) = URL_MATCHER.at(url.path())
        && let Some(id) = result.params.get("id")
    {
        let text = text.replace(url.as_str(), &(format!("https://{domain}/watch?v={id}")));
        let text = format!("{}: {}", link(user.url().as_str(), &user.full_name()), text);
        bot.replace_chat_message(&message, &text).await?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_url_matcher() {
        crate::url::verify_url_matcher(
            &[
                "https://www.youtube.com/shorts/SqjNixegPKk",
                "https://www.youtube.com/shorts/SqjNixegPKk?feature=share",
                "https://youtube.com/shorts/SqjNixegPKk",
                "https://youtube.com/shorts/JY55-UBtlf8?feature=share",
                "https://youtube.com/shorts/afHFjnPy_vk?feature=share",
            ],
            &super::URL_MATCHER,
        );
    }
}
