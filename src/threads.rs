use crate::{AsyncError, bot_ext::BotExt, router_ext::add_route};
use matchit::Router;
use std::sync::LazyLock;
use teloxide::{Bot, types::Message};

pub const DOMAINS: [&str; 2] = ["threads.net", "threads.com"];
static URL_MATCHER: LazyLock<Router<()>> = LazyLock::new(|| {
    let mut router = Router::new();
    add_route!(router, "/@{user}/post/{id}");
    router
});

pub async fn handler(bot: Bot, message: Message) -> Result<(), AsyncError> {
    bot.perform_replacement(&message, &URL_MATCHER, "fixthreads.net", |_| None)
        .await?;
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_url_matcher() {
        crate::url::verify_url_matcher(
            &[
                "https://www.threads.com/@yukimurayy/post/DRjt_AIAV6Q",
                "https://www.threads.net/@yukimurayy/post/DRjt_AIAV6Q",
            ],
            &super::URL_MATCHER,
        );
    }
}
