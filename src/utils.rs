use reqwest::Url;
use std::{error::Error, sync::LazyLock};
use teloxide::types::{Message, MessageEntityKind};
use tracing::{error, trace};

pub(crate) type AsyncError = Box<dyn Error + Send + Sync + 'static>;

pub(crate) fn get_urls_from_message(msg: &Message) -> Vec<Url> {
    if let Some(entities) = msg.entities()
        && !entities.is_empty()
        && let Some(text) = msg.text()
    {
        if entities[0].kind == MessageEntityKind::BotCommand {
            return Vec::new();
        };
        let url_entities: Vec<_> = entities
            .iter()
            .filter(|entity| entity.kind == MessageEntityKind::Url)
            .collect();
        if url_entities.is_empty() {
            return Vec::new();
        }
        let utf16 = text.encode_utf16().collect::<Vec<u16>>();
        let mut urls = Vec::with_capacity(url_entities.len());
        for entity in &url_entities {
            if let Ok(url) = Url::parse(&String::from_utf16_lossy(
                &utf16[entity.offset..entity.offset + entity.length],
            )) {
                urls.push(url);
            }
        }
        let url_str = urls.iter().map(reqwest::Url::as_str).collect::<Vec<&str>>();
        trace!(message_id = %msg.id.0, urls = ?url_str, "get_urls_from_message");
        return urls;
    }
    Vec::new()
}

pub(crate) fn has_matching_urls(msg: &Message, domains: &[&str]) -> bool {
    get_urls_from_message(msg)
        .iter()
        .any(|url| check_matches_domain(url, domains))
}

pub(crate) fn get_preview_url(msg: &Message, from: &str, to: &str) -> Option<String> {
    get_urls_from_message(msg)
        .first()
        .map(|url| String::from(url.clone()).replace(from, to))
}

fn check_matches_domain(url: &Url, domains: &[&str]) -> bool {
    if let Some(host) = url.host_str() {
        let host = host.trim_start_matches("www.");
        return domains.iter().any(|domain| host == *domain);
    }
    false
}

pub(crate) fn scrub_urls(msg: &Message) -> Option<String> {
    if let Some(text) = msg.text() {
        let urls = get_urls_from_message(msg);
        let mut final_text = text.to_owned();
        for url in urls {
            if let Some(query_str) = url.query() {
                let scrubbed_url = url.as_str().replace(&format!("?{query_str}"), "");
                final_text = final_text.replace(url.as_str(), &scrubbed_url);
            }
        }
        trace!(?text, ?final_text, "scrub_urls");
        Some(final_text)
    } else {
        error!(message_id = %msg.id.0, "scrub_urls failed to find text");
        None
    }
}

const TRUE_VALUES: [&str; 4] = ["true", "on", "yes", "enable"];
const FALSE_VALUES: [&str; 4] = ["false", "off", "no", "disable"];
static EXPECTED_VALUES: LazyLock<String> = LazyLock::new(|| {
    [TRUE_VALUES, FALSE_VALUES]
        .concat()
        .iter()
        .map(|item| format!("'{item}'"))
        .collect::<Vec<_>>()
        .join(", ")
});

pub(crate) fn parse_bool(input: &str) -> Result<bool, String> {
    let input = input.split(' ').collect::<Vec<_>>();
    if input.len() > 1 {
        return Err(format!(
            "Unexpected number of arguments. Expected one of: {}.",
            *EXPECTED_VALUES
        ));
    }

    match input[0].to_lowercase().as_str() {
        arg if TRUE_VALUES.contains(&arg) => Ok(true),
        arg if FALSE_VALUES.contains(&arg) => Ok(false),
        arg => {
            let message = format!(
                "Unexpected argument '{arg}'. Expected one of: {}.",
                *EXPECTED_VALUES
            );
            Err(message)
        }
    }
}

pub(crate) fn extract_dice_count(input: &str, default: u8) -> Result<u8, String> {
    if input.is_empty() {
        return Ok(default);
    }

    let input = input.split(' ').collect::<Vec<_>>();
    if input.len() > 1 {
        return Err(String::from(
            "Unexpected number of arguments. Expected a numeric value from 1-255.",
        ));
    }

    if let Ok(value) = input[0].parse::<u8>() {
        Ok(value)
    } else {
        let message = format!(
            "Unexpected argument '{}'. Expected a number from 1-255.",
            input[0]
        );
        Err(message)
    }
}

#[cfg(test)]
mod check_matches_domain_tests {
    use super::check_matches_domain;

    #[test]
    fn ignores_www() {
        let url = "https://www.example.com";
        let domains = ["example.com"];
        let url = reqwest::Url::parse(url).unwrap();
        assert!(check_matches_domain(&url, &domains));
    }

    #[test]
    fn ignores_substring_match() {
        let url = "https://www.ddinstagram.com";
        let domains = ["instagram.com"];
        let url = reqwest::Url::parse(url).unwrap();
        assert!(!check_matches_domain(&url, &domains));
    }
}
