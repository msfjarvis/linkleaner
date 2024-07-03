use crate::{
    message::BotExt,
    utils::{scrub_urls, AsyncError},
};
use teloxide::{types::Message, utils::html::link, Bot};

pub const DOMAINS: [&str; 1] = ["reddit.com"];

pub async fn handler(bot: Bot, message: Message) -> Result<(), AsyncError> {
    if let Some(text) = scrub_urls(&message)
        && let Some(user) = message.from()
    {
        let text = text.replace("reddit.com", "rxddit.com");
        let text = format!("{}: {}", link(user.url().as_str(), &user.full_name()), text);
        bot.replace_chat_message(&message, &text).await?;
    }
    Ok(())
}

// Example URLs
// "https://reddit.com/r/shittymoviedetails/comments/160onpq/breaking_actor_from_home_alone_2_arrested_today/jxnkq4g"
// "https://reddit.com/r/shittymoviedetails/comments/160onpq/breaking_actor_from_home_alone_2_arrested_today"
// "https://reddit.com/r/shittymoviedetails/comments/160onpq"
// "https://reddit.com/r/MemePiece/s/15w6vzg82W"
// "https://reddit.com/160onpq"
