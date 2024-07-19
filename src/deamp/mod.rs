mod model;

use crate::{
    message::BotExt,
    utils::{get_urls_from_message, AsyncError},
};
use model::AMPResponse;
use reqwest::Url;
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

pub fn is_amp(url: &Url) -> bool {
    if let Some(mut segments) = url.path_segments()
        && let Some(host) = url.host_str()
    {
        segments.any(|x| x == "amp")
            || host.ends_with(".cdn.ampproject.org")
            || url.query().map_or(false, |query| query.contains("amp"))
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::is_amp;
    use reqwest::Url;
    const DATA: [(bool, &str); 4] = [
        (true, "https://www.google.com/amp/s/m.gsmarena.com/samsung_galaxy_tab_s9-ampp-12439.php"),
        (true, "https://www.google.com/amp/s/news.abplive.com/news/india/microsoft-server-outage-multiple-airports-hit-by-web-check-in-server-glitch-1703909/amp"),
        (false, "https://github.com"),
        (true, "https://josysnavi.jp/2024/blog-00058?amp"),
    ];

    #[test]
    fn test_is_amp() {
        for (expected, url) in DATA {
            assert_eq!(expected, is_amp(&Url::parse(url).unwrap()), "{url}");
        }
    }
}
