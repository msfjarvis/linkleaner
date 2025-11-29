use crate::{AsyncError, bot_ext::BotExt, router_ext::add_route};
use matchit::Router;
use std::sync::LazyLock;
use teloxide::{Bot, types::Message};

pub const DOMAINS: [&str; 2] = ["instagram.com", "www.instagram.com"];
static URL_MATCHER: LazyLock<Router<()>> = LazyLock::new(|| {
    let mut router = Router::new();
    add_route!(router, "/p/{id}");
    add_route!(router, "/reel/{id}");
    add_route!(router, "/reels/{id}");
    add_route!(router, "/tv/{id}");
    add_route!(router, "/{username}/p/{id}");
    add_route!(router, "/{username}/reel/{id}");
    add_route!(router, "/{username}/reels/{id}");
    add_route!(router, "/share/p/{id}");
    add_route!(router, "/share/reel/{id}");
    add_route!(router, "/share/reels/{id}");
    router
});

pub async fn handler(bot: Bot, message: Message) -> Result<(), AsyncError> {
    bot.perform_replacement(&message, &URL_MATCHER, "fxstagram.com", |_| None)
        .await?;
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
