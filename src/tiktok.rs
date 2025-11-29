use crate::{AsyncError, bot_ext::BotExt, router_ext::add_route};
use matchit::Router;
use std::sync::LazyLock;
use teloxide::{Bot, types::Message};

pub const DOMAINS: [&str; 3] = ["tiktok.com", "www.tiktok.com", "vm.tiktok.com"];
static URL_MATCHER: LazyLock<Router<()>> = LazyLock::new(|| {
    let mut router = Router::new();
    add_route!(router, "/{video_id}");
    add_route!(router, "/t/{video_id}");
    add_route!(router, "/embed/{video_id}");
    add_route!(router, "/@{username}/video/{video_id}");
    add_route!(router, "/@{username}/photo/{video_id}");
    add_route!(router, "/@{username}/live");
    router
});

pub async fn handler(bot: Bot, message: Message) -> Result<(), AsyncError> {
    bot.perform_replacement(&message, &URL_MATCHER, "d.tnktok.com", |_| None)
        .await?;
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_url_matcher() {
        crate::url::verify_url_matcher(
            &[
                "https://www.tiktok.com/@houshoumarine_hololivejp/video/7472637264254864657",
                "https://vm.tiktok.com/ZNdJ1eWcb",
            ],
            &super::URL_MATCHER,
        );
    }
}
