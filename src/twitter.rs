use crate::{AsyncError, bot_ext::BotExt, router_ext::add_route};
use matchit::Router;
use std::sync::LazyLock;
use teloxide::{Bot, types::Message};

static PROVIDER: LazyLock<String> = LazyLock::new(|| {
    std::env::var("zeppelinker_TWITTER_PROVIDER").unwrap_or_else(|_| "fixupx.com".to_string())
});

pub const DOMAINS: [&str; 4] = ["twitter.com", "mobile.twitter.com", "x.com", "mobile.x.com"];
static URL_MATCHER: LazyLock<Router<()>> = LazyLock::new(|| {
    let mut router = Router::new();
    add_route!(router, "/{user}/status/{tweet_id}");
    add_route!(router, "/{user}/status/{tweet_id}/photo/{num}");
    add_route!(router, "/i/status/{tweet_id}");
    router
});

pub async fn handler(bot: Bot, message: Message) -> Result<(), AsyncError> {
    bot.perform_replacement(&message, &URL_MATCHER, &PROVIDER, Some("/en"), |url| {
        let mut button_url = url.clone();
        button_url.set_host(Some("xcancel.com")).unwrap();
        Some(("View on Nitter", button_url))
    })
    .await?;
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_url_matcher() {
        crate::url::verify_url_matcher(
            &[
                "https://mobile.twitter.com/Jack/status/20",
                "https://twitter.com/Jack/status/20",
                "https://mobile.x.com/Jack/status/20",
                "https://x.com/Jack/status/20",
                "https://x.com/realonx1/status/1879076905925980535/photo/1",
                "https://x.com/i/status/1878964730242687241",
            ],
            &super::URL_MATCHER,
        );
    }

    #[test]
    fn test_preview_url_has_en_suffix() {
        use crate::url::get_preview_url_with_suffix;
        use url::Url;

        for (input, domain) in &[
            ("https://twitter.com/Jack/status/20", "twitter.com"),
            ("https://x.com/Jack/status/20", "x.com"),
            (
                "https://mobile.twitter.com/Jack/status/20",
                "mobile.twitter.com",
            ),
            ("https://mobile.x.com/Jack/status/20", "mobile.x.com"),
        ] {
            let url = Url::parse(input).unwrap();
            let result = get_preview_url_with_suffix(&url, domain, &super::PROVIDER, Some("/en"));
            assert!(
                result.ends_with("/en"),
                "Expected preview URL to end with /en, got: {result}"
            );
            assert!(
                result.contains(&*super::PROVIDER),
                "Expected preview URL to use {}, got: {result}",
                *super::PROVIDER
            );
        }
    }
}
