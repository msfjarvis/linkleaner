use once_cell::sync::Lazy;
use reqwest::Url;
use teloxide::types::{Message, MessageEntityKind};
use tracing::trace;

pub(crate) fn get_urls_from_message(msg: &Message) -> Vec<String> {
    if let Some(entities) = msg.entities() && let Some(text) = msg.text() {
        let url_entities = entities
            .iter()
            .filter(|entity| entity.kind == MessageEntityKind::Url)
            .collect::<Vec<_>>();
        let utf16 = text.encode_utf16().collect::<Vec<u16>>();
        let mut urls = Vec::with_capacity(url_entities.len());
        for entity in &url_entities {
            urls.push(String::from_utf16_lossy(&utf16[entity.offset..entity.offset + entity.length]));
        }
        trace!(?entities, ?url_entities, ?urls, "get_urls_from_message");
        return urls;
    }
    Vec::with_capacity(0)
}

pub(crate) fn scrub_urls(msg: &Message) -> Option<String> {
    if let Some(text) = msg.text() {
        let urls = get_urls_from_message(msg);
        let mut final_text = text.to_owned();
        for item in urls {
            if let Ok(url) = Url::parse(&item) && let Some(query_str) = url.query() {
                let scrubbed_url = item.replace(&format!("?{query_str}"), "");
                final_text = final_text.replace(&item, &scrubbed_url);
            }
        }
        trace!(?text, ?final_text, "scrub_urls");
        Some(final_text)
    } else {
        trace!(message_id = %msg.id.0, "scrub_urls failed to find text");
        None
    }
}

pub(crate) fn parse_bool(input: &str) -> Result<Option<bool>, String> {
    const TRUE_VALUES: [&str; 4] = ["true", "on", "yes", "enable"];
    const FALSE_VALUES: [&str; 4] = ["false", "off", "no", "disable"];
    static EXPECTED_VALUES: Lazy<String> = Lazy::new(|| {
        [TRUE_VALUES, FALSE_VALUES]
            .concat()
            .iter()
            .map(|item| format!("'{item}'"))
            .collect::<Vec<_>>()
            .join(", ")
    });

    let input = input.split(' ').collect::<Vec<_>>();
    if input.len() > 1 {
        return Err(format!(
            "Unexpected number of arguments. Expected one of: {}.",
            *EXPECTED_VALUES
        ));
    }

    match input[0].to_lowercase().as_str() {
        arg if TRUE_VALUES.contains(&arg) => Ok(Some(true)),
        arg if FALSE_VALUES.contains(&arg) => Ok(Some(false)),
        "" => Ok(None),
        arg => {
            let message = format!(
                "Unexpected argument '{arg}'. Expected one of: {}.",
                *EXPECTED_VALUES
            );
            Err(message)
        }
    }
}
