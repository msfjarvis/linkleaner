use crate::{AsyncError, bot_ext::BotExt, router_ext::add_route};
use matchit::Router;
use std::{collections::HashMap, sync::LazyLock};
use teloxide::{Bot, types::Message};

static PROVIDER: LazyLock<String> = LazyLock::new(|| {
    std::env::var("LINKLEANER_INSTAGRAM_PROVIDER").unwrap_or_else(|_| "zzinstagram.com".to_string())
});

static DELETE_PROBABILITIES: LazyLock<HashMap<i64, f64>> = LazyLock::new(|| {
    parse_delete_probabilities(
        &std::env::var("LINKLEANER_INSTAGRAM_DELETE_PROBABILITIES").unwrap_or_default(),
    )
});

fn parse_delete_probabilities(value: &str) -> HashMap<i64, f64> {
    value
        .split(',')
        .filter_map(|entry| {
            let entry = entry.trim();
            if entry.is_empty() {
                return None;
            }
            let parsed = entry.split_once(':').and_then(|(id, probability)| {
                let id = id.trim().parse().ok()?;
                let probability: f64 = probability.trim().parse().ok()?;
                (0.0..=1.0)
                    .contains(&probability)
                    .then_some((id, probability))
            });
            if parsed.is_none() {
                tracing::warn!(%entry, "invalid Instagram deletion probability; ignoring entry");
            }
            parsed
        })
        .collect()
}

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
    let probability = DELETE_PROBABILITIES
        .get(&message.chat.id.0)
        .copied()
        .unwrap_or(0.0);
    let should_delete = probability > 0.0 && rand::random_range(0.0..1.0) < probability;
    bot.perform_replacement(
        &message,
        &URL_MATCHER,
        &PROVIDER,
        None,
        should_delete,
        |_| None,
    )
    .await?;
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn deletion_probabilities_validate_entries_and_default_to_zero() {
        let probabilities = super::parse_delete_probabilities(
            " -100:1, -200:0.25, -300:0, -400:-0.1, -500:1.1, -600:NaN, bad:1, -700:nope, invalid ",
        );
        assert_eq!(probabilities.len(), 3);
        assert_eq!(probabilities.get(&-100), Some(&1.0));
        assert_eq!(probabilities.get(&-200), Some(&0.25));
        assert_eq!(probabilities.get(&-300), Some(&0.0));
        assert_eq!(probabilities.get(&-400).copied().unwrap_or(0.0), 0.0);
        assert!(super::parse_delete_probabilities("").is_empty());
    }

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
