use crate::{
    message::BotExt,
    utils::{get_preview_url, AsyncError},
};
use teloxide::{types::Message, utils::html::link, Bot};

pub const DOMAINS: [&str; 1] = ["reddit.com"];

pub async fn handler(bot: Bot, message: Message) -> Result<(), AsyncError> {
    // We ideally run text through utils::scrub_urls to drop query parameters
    // but this breaks Reddit media URLs.
    if let Some(text) = message.text()
        && let Some(ref user) = message.from
    {
        let text = format!("{}: {}", link(user.url().as_str(), &user.full_name()), text);
        bot.send_preview(&message, &text, |msg| {
            get_preview_url(msg, "reddit.com", "rxddit.com")
        })
        .await?;
    }
    Ok(())
}

// Example URLs
// "https://reddit.com/r/shittymoviedetails/comments/160onpq/breaking_actor_from_home_alone_2_arrested_today/jxnkq4g"
// "https://reddit.com/r/shittymoviedetails/comments/160onpq/breaking_actor_from_home_alone_2_arrested_today"
// "https://reddit.com/r/shittymoviedetails/comments/160onpq"
// "https://reddit.com/r/MemePiece/s/15w6vzg82W"
// "https://reddit.com/160onpq"
