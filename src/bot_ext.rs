use crate::{
    get_urls_from_message,
    url::{get_preview_url, scrub_urls},
    AsyncError,
};
use matchit::Router;
use std::sync::LazyLock;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{
        ChatAction, InlineKeyboardButton, InlineKeyboardMarkup, LinkPreviewOptions, Message,
        ParseMode, ReplyParameters, UserId,
    },
    utils::html::link,
    Bot, RequestError,
};
use url::{Host, Url};

static BOT_ID: LazyLock<UserId> = LazyLock::new(|| {
    let value = std::env::var("BOT_ID").expect("BOT_ID must be defined");
    let id = value
        .parse::<u64>()
        .expect("BOT_ID must be a valid integer");
    UserId(id)
});

pub(crate) trait BotExt {
    async fn reply(&self, message: &Message, text: &str) -> Result<Message, RequestError>;
    async fn try_reply(&self, message: &Message, text: &str) -> Result<Message, RequestError>;
    async fn try_reply_silent(
        &self,
        message: &Message,
        text: &str,
    ) -> Result<Message, RequestError>;
    async fn replace_chat_message(
        &self,
        message: &Message,
        text: &str,
    ) -> Result<Message, RequestError>;
    async fn perform_replacement(
        &self,
        message: &Message,
        url_matcher: &Router<()>,
        preview_domain: &str,
        get_button_data: impl Fn(&Url) -> Option<(&str, Url)>,
    ) -> Result<(), AsyncError>;
    fn is_self_message(&self, message: &Message) -> bool;
}

impl BotExt for Bot {
    async fn reply(&self, message: &Message, text: &str) -> Result<Message, RequestError> {
        self.send_message(message.chat.id, text)
            .reply_parameters(ReplyParameters::new(message.id))
            .parse_mode(ParseMode::Html)
            .await
    }

    async fn try_reply(&self, message: &Message, text: &str) -> Result<Message, RequestError> {
        self.send_chat_action(message.chat.id, ChatAction::Typing)
            .await?;
        self.try_reply_silent(message, text).await
    }

    async fn try_reply_silent(
        &self,
        message: &Message,
        text: &str,
    ) -> Result<Message, RequestError> {
        if let Some(reply) = message.reply_to_message() {
            self.send_message(message.chat.id, text)
                .reply_parameters(ReplyParameters::new(reply.id))
                .parse_mode(ParseMode::Html)
                .await
        } else {
            self.send_message(message.chat.id, text)
                .parse_mode(ParseMode::Html)
                .await
        }
    }

    async fn replace_chat_message(
        &self,
        message: &Message,
        text: &str,
    ) -> Result<Message, RequestError> {
        let _del = self.delete_message(message.chat.id, message.id).await;
        self.try_reply_silent(message, text).await
    }

    fn is_self_message(&self, message: &Message) -> bool {
        if let Some(forwarder) = message.forward_from_user() {
            forwarder.id == *BOT_ID
        } else {
            message
                .from
                .as_ref()
                .is_some_and(|from| from.id.0 == BOT_ID.0)
        }
    }

    async fn perform_replacement(
        &self,
        message: &Message,
        url_matcher: &Router<()>,
        preview_domain: &str,
        get_button_data: impl Fn(&Url) -> Option<(&str, Url)>,
    ) -> Result<(), AsyncError> {
        let urls = get_urls_from_message(message);
        if !self.is_self_message(message)
            && let Some(text) = scrub_urls(message)
            && let Some(ref user) = message.from
            && let Some(url) = urls.first()
            && let Some(host) = url.host()
            && let Host::Domain(domain) = host
            && let Ok(_) = url_matcher.at(url.path())
        {
            let text = format!("{}: {}", link(user.url().as_str(), &user.full_name()), text);
            let reply_button = match get_button_data(url) {
                Some((label, url)) => Some(InlineKeyboardMarkup::new(vec![vec![
                    InlineKeyboardButton::url(label, url),
                ]])),
                _ => None,
            };
            let preview_options = LinkPreviewOptions {
                is_disabled: false,
                url: get_preview_url(message, domain, preview_domain),
                prefer_small_media: false,
                prefer_large_media: true,
                show_above_text: false,
            };
            let _del = self.delete_message(message.chat.id, message.id).await;
            if let Some(reply) = message.reply_to_message() {
                let send_message = self
                    .send_message(message.chat.id, text)
                    .reply_parameters(ReplyParameters::new(reply.id))
                    .link_preview_options(preview_options)
                    .parse_mode(ParseMode::Html);
                if let Some(reply_button) = reply_button {
                    send_message.reply_markup(reply_button)
                } else {
                    send_message
                }
            } else {
                let send_message = self
                    .send_message(message.chat.id, text)
                    .parse_mode(ParseMode::Html)
                    .link_preview_options(preview_options);
                if let Some(reply_button) = reply_button {
                    send_message.reply_markup(reply_button)
                } else {
                    send_message
                }
            }
            .await?;
        }
        Ok(())
    }
}
