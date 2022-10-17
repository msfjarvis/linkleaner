use once_cell::sync::Lazy;
use teloxide::types::{Message, MessageEntityKind};
use tracing::trace;

pub(crate) fn get_urls_from_message(msg: &Message) -> Vec<String> {
    if let Some(entities) = msg.entities() && let Some(text) = msg.text() {
        trace!(?entities, "All entities");
        let entities = entities
            .iter()
            .filter(|entity| entity.kind == MessageEntityKind::Url)
            .collect::<Vec<_>>();
        trace!(?entities, "URL entities");
        let mut urls = Vec::with_capacity(entities.len());
        for entity in entities {
            urls.push(text[entity.offset..entity.offset + entity.length].to_string());
        }
        trace!(?urls, "Parsed URLs");
        return urls;
    }
    Vec::with_capacity(0)
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
