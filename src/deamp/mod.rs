mod model;

use crate::{
    message::BotExt,
    utils::{get_urls_from_message, AsyncError},
};
use model::AMPResponse;
use std::str::FromStr;
use teloxide::{types::Message, utils::html::link, Bot};
use tracing::debug;

const BASE_URL: &str = "https://www.amputatorbot.com/api/v1/convert?gac=true&md=3&q=";

fn deserialize_amp_response(text: &str) -> Result<AMPResponse, serde_json::Error> {
    serde_json::from_str(text)
}

pub async fn handler(bot: Bot, message: Message) -> Result<(), AsyncError> {
    if let Some(text) = message.text()
        && let Some(user) = message.from()
    {
        let mut text = String::from_str(text)?;
        let urls = get_urls_from_message(&message);
        debug!(?urls);
        for url in &urls {
            let resp = reqwest::get(&format!("{BASE_URL}{url}"))
                .await?
                .text()
                .await?;
            debug!(?resp, "{url}");
            let resp = deserialize_amp_response(&resp)?;
            if let AMPResponse::Success(ok) = resp {
                text = text.replace(url.as_str(), &ok[0].canonical.url);
            } else {
                return Ok(());
            }
        }
        let text = format!("{}: {}", link(user.url().as_str(), &user.full_name()), text);
        bot.replace_chat_message(&message, &text).await?;
    }
    Ok(())
}

// The dptree handler requires this to be a move
#[allow(clippy::needless_pass_by_value)]
pub fn is_amp(msg: Message) -> bool {
    let urls = get_urls_from_message(&msg);
    if urls.is_empty() {
        return false;
    }
    urls.iter().any(|url| {
        if let Some(mut segments) = url.path_segments()
            && let Some(host) = url.host_str()
        {
            segments.any(|x| x == "amp") || host.ends_with(".cdn.ampproject.org")
        } else {
            false
        }
    })
}
