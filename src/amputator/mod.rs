mod model;

use crate::utils::get_urls_from_message;
use model::AMPResponse;
use reqwest::Url;
use std::{error::Error, str::FromStr};
use teloxide::{
    adaptors::AutoSend,
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{Message, ParseMode},
    Bot,
};
use tracing::debug;

const BASE_URL: &str = "https://www.amputatorbot.com/api/v1/convert?gac=true&md=3&q=";

fn deserialize_amp_response(text: &str) -> Result<AMPResponse, serde_json::Error> {
    serde_json::from_str(text)
}

pub async fn handler(
    bot: AutoSend<Bot>,
    message: Message,
) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    if let Some(text) = message.text() && let Some(user) = message.from() {
        let mut text = String::from_str(text)?;
        let urls = get_urls_from_message(&message);
        debug!(?urls);
        for url in &urls {
            let resp = reqwest::get(&format!("{}{}", BASE_URL, url))
                .await?
                .text()
                .await?;
            debug!(?resp, "{url}");
            let resp = deserialize_amp_response(&resp)?;
            if let AMPResponse::Success(ok) = resp {
                text = text.replace(url, &ok[0].canonical.url);
            } else {
                return Ok(());
            }
        }
        let text = format!(
            "<a href=\"{}\">{}</a>: {}",
            user.id.url(),
            user.full_name(),
            text
        );
        let _del = bot.delete_message(message.chat.id, message.id).await;
        if let Some(reply) = message.reply_to_message() {
            bot.send_message(message.chat.id, text)
                .reply_to_message_id(reply.id)
                .parse_mode(ParseMode::Html)
                .await?;
        } else {
            bot.send_message(message.chat.id, text)
                .parse_mode(ParseMode::Html)
                .await?;
        }
    }
    Ok(())
}

pub fn is_amp(msg: &Message) -> bool {
    let urls = get_urls_from_message(msg);
    if urls.is_empty() {
        return false;
    }
    urls.iter().flat_map(|url| Url::parse(url)).any(|url| {
        if let Some(mut segments) = url.path_segments() && let Some(host) = url.host_str() {
            segments.any(|x| x == "amp") || host.ends_with(".cdn.ampproject.org")
        } else {
            false
        }
    })
}
