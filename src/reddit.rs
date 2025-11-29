use crate::{AsyncError, bot_ext::BotExt, router_ext::add_route};
use matchit::Router;
use std::sync::LazyLock;
use teloxide::{Bot, types::Message};

static URL_MATCHER: LazyLock<Router<()>> = LazyLock::new(|| {
    let mut router = Router::new();
    add_route!(router, "/r/{username}/comments/{id}/{slug}/{comment}");
    add_route!(router, "/r/{username}/comments/{id}/{slug}");
    add_route!(router, "/r/{username}/comments/{id}");
    add_route!(router, "/r/{username}/s/{id}/{slug}/{comment}");
    add_route!(router, "/r/{username}/s/{id}/{slug}");
    add_route!(router, "/r/{username}/s/{id}");

    add_route!(router, "/u/{username}/comments/{id}/{slug}/{comment}");
    add_route!(router, "/u/{username}/comments/{id}/{slug}");
    add_route!(router, "/u/{username}/comments/{id}");
    add_route!(router, "/u/{username}/s/{id}/{slug}/{comment}");
    add_route!(router, "/u/{username}/s/{id}/{slug}");
    add_route!(router, "/u/{username}/s/{id}");

    add_route!(router, "/user/{username}/comments/{id}/{slug}/{comment}");
    add_route!(router, "/user/{username}/comments/{id}/{slug}");
    add_route!(router, "/user/{username}/comments/{id}");
    add_route!(router, "/user/{username}/s/{id}/{slug}/{comment}");
    add_route!(router, "/user/{username}/s/{id}/{slug}");
    add_route!(router, "/user/{username}/s/{id}");

    add_route!(router, "/{id}");
    router
});

pub const DOMAINS: [&str; 3] = ["reddit.com", "redd.it", "www.reddit.com"];

pub async fn handler(bot: Bot, message: Message) -> Result<(), AsyncError> {
    bot.perform_replacement(&message, &URL_MATCHER, "vxreddit.com", |_| None)
        .await?;
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
