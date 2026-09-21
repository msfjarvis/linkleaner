use crate::{AsyncError, bot_ext::BotExt, router_ext::add_route};
use matchit::Router;
use rand::seq::IndexedRandom;
use std::sync::LazyLock;
use teloxide::{Bot, types::Message};

static PROVIDERS: LazyLock<Vec<String>> = LazyLock::new(|| {
    provider_pool(
        std::env::var("LINKLEANER_TWITTER_PROVIDERS").ok(),
        std::env::var("LINKLEANER_TWITTER_PROVIDER").ok(),
    )
});

fn provider_pool(providers: Option<String>, provider: Option<String>) -> Vec<String> {
    providers
        .as_deref()
        .map(|providers| {
            providers
                .split(',')
                .map(str::trim)
                .filter(|provider| !provider.is_empty())
                .map(str::to_owned)
                .collect()
        })
        .filter(|providers: &Vec<String>| !providers.is_empty())
        .unwrap_or_else(|| {
            vec![
                provider
                    .filter(|provider| !provider.trim().is_empty())
                    .unwrap_or_else(|| "fixupx.com".to_string())
                    .trim()
                    .to_owned(),
            ]
        })
}

pub const DOMAINS: [&str; 4] = ["twitter.com", "mobile.twitter.com", "x.com", "mobile.x.com"];
static URL_MATCHER: LazyLock<Router<()>> = LazyLock::new(|| {
    let mut router = Router::new();
    add_route!(router, "/{user}/status/{tweet_id}");
    add_route!(router, "/{user}/status/{tweet_id}/photo/{num}");
    add_route!(router, "/i/status/{tweet_id}");
    router
});

pub async fn handler(bot: Bot, message: Message) -> Result<(), AsyncError> {
    let provider = PROVIDERS
        .choose(&mut rand::rng())
        .expect("provider pool must contain a fallback provider");
    bot.perform_replacement(&message, &URL_MATCHER, provider, Some("/en"), |_| None)
        .await?;
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn provider_pool_uses_configured_domains() {
        assert_eq!(
            super::provider_pool(
                Some(" fixupx.com, fxtwitter.com, , vxtwitter.com ".to_string()),
                None,
            ),
            ["fixupx.com", "fxtwitter.com", "vxtwitter.com"],
        );
    }

    #[test]
    fn provider_pool_prefers_plural_configuration_over_legacy_provider() {
        assert_eq!(
            super::provider_pool(
                Some("vxtwitter.com".to_string()),
                Some("fxtwitter.com".to_string()),
            ),
            ["vxtwitter.com"],
        );
    }

    #[test]
    fn provider_pool_falls_back_to_legacy_provider_or_default() {
        assert_eq!(
            super::provider_pool(None, Some(" fxtwitter.com ".to_string())),
            ["fxtwitter.com"],
        );
        assert_eq!(
            super::provider_pool(Some(" , ".to_string()), Some(" ".to_string())),
            ["fixupx.com"],
        );
    }

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
            let provider = "fixupx.com";
            let result = get_preview_url_with_suffix(&url, domain, provider, Some("/en"));
            assert!(
                result.ends_with("/en"),
                "Expected preview URL to end with /en, got: {result}"
            );
            assert!(
                result.contains(provider),
                "Expected preview URL to use {provider}, got: {result}"
            );
        }
    }
}
