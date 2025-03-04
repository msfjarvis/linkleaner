use teloxide::types::{Message, MessageEntityKind};
use tracing::{error, trace};
use url::Url;

pub(crate) fn get_urls_from_message(msg: &Message) -> Vec<Url> {
    if let Some(entities) = msg.entities()
        && !entities.is_empty()
        && let Some(text) = msg.text()
    {
        if entities[0].kind == MessageEntityKind::BotCommand {
            return Vec::new();
        }
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
        let url_str = urls.iter().map(Url::as_str).collect::<Vec<&str>>();
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
        return domains.contains(&host);
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

#[cfg(test)]
pub fn verify_url_matcher(urls: &[&str], router: &matchit::Router<()>) {
    use url::Url;
    // Build up a list of the URLs and their trailing slash versions to ensure the macro
    // we use to add routes to the router works as expected.
    let mut all_urls = vec![];
    all_urls.extend(urls.iter().map(|url| (*url).to_string()));
    all_urls.extend(urls.iter().map(|url| format!("{url}/")));
    all_urls
        .iter()
        .flat_map(|url| Url::parse(url))
        .map(|url| url.path().to_string())
        .for_each(|path| {
            assert!(router.at(&path).is_ok(), "Failed to match URL: {path}");
        });
}

#[cfg(test)]
mod test {
    use super::check_matches_domain;
    use url::Url;

    #[test]
    fn ignores_www() {
        let url = "https://www.example.com";
        let domains = ["example.com"];
        let url = Url::parse(url).unwrap();
        assert!(check_matches_domain(&url, &domains));
    }

    #[test]
    fn ignores_substring_match() {
        let url = "https://www.ddinstagram.com";
        let domains = ["instagram.com"];
        let url = Url::parse(url).unwrap();
        assert!(!check_matches_domain(&url, &domains));
    }
}
