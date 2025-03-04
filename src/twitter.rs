use crate::{bot_ext::BotExt, router_ext::add_route, AsyncError};
use matchit::Router;
use std::sync::LazyLock;
use teloxide::{types::Message, Bot};

pub const DOMAINS: [&str; 4] = ["twitter.com", "mobile.twitter.com", "x.com", "mobile.x.com"];
static URL_MATCHER: LazyLock<Router<()>> = LazyLock::new(|| {
    let mut router = Router::new();
    add_route!(router, "/{user}/status/{tweet_id}");
    add_route!(router, "/{user}/status/{tweet_id}/photo/{num}");
    add_route!(router, "/i/status/{tweet_id}");
    router
});

pub async fn handler(bot: Bot, message: Message) -> Result<(), AsyncError> {
    bot.perform_replacement(&message, &URL_MATCHER, "fixupx.com", |url| {
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
}
