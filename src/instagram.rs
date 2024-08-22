use crate::utils::{get_urls_from_message, AsyncError};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{LinkPreviewOptions, Message, ParseMode, ReplyParameters},
    utils::html::link,
    Bot,
};
use tracing::trace;
pub const DOMAINS: [&str; 1] = ["instagram.com"];

pub async fn handler(bot: Bot, message: Message) -> Result<(), AsyncError> {
    if let Some(text) = &message.text()
        && let Some(ref user) = message.from
    {
        let text = format!("{}: {}", link(user.url().as_str(), &user.full_name()), text);
        let url: Option<String> = get_urls_from_message(&message)
            .first()
            .map(|url| String::from(url.clone()).replace("instagram.com", "ddinstagram.com"));
        trace!(?url);
        let preview_options = LinkPreviewOptions {
            is_disabled: false,
            url,
            prefer_small_media: false,
            prefer_large_media: true,
            show_above_text: true,
        };
        let _del = bot.delete_message(message.chat.id, message.id).await;
        if let Some(reply) = message.reply_to_message() {
            bot.send_message(message.chat.id, text)
                .parse_mode(ParseMode::Html)
                .reply_parameters(ReplyParameters::new(reply.id))
                .link_preview_options(preview_options)
                .await?;
            Ok(())
        } else {
            bot.send_message(message.chat.id, text)
                .parse_mode(ParseMode::Html)
                .link_preview_options(preview_options)
                .await?;
            Ok(())
        }
    } else {
        Ok(())
    }
}
