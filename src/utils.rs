use rand::{thread_rng, Rng};
use walkdir::WalkDir;

pub(crate) fn escape_markdown_str(msg: &str) -> String {
    msg.replace("_", r"\_")
}

pub(crate) fn file_name_to_label(msg: &str) -> String {
    escape_markdown_str(msg)
        .replace(r"\_", " ")
        .replace(".jpg", "")
}

pub(crate) fn get_search_results(items: Vec<String>, search_term: &str) -> Vec<String> {
    if search_term.contains('_') {
        items
            .into_iter()
            .filter(|x| x.starts_with(&search_term))
            .collect()
    } else {
        items
            .into_iter()
            .filter(|x| tokenized_search(x, &search_term))
            .collect()
    }
}

pub(crate) fn index_pictures(directory: &str) -> Vec<String> {
    let mut images: Vec<String> = Vec::new();
    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        images.push(String::from(
            entry
                .path()
                .strip_prefix(directory)
                .unwrap()
                .to_str()
                .unwrap(),
        ))
    }
    images
}

pub(crate) fn join_results_to_string(
    search_term: String,
    items: Vec<String>,
    base_url: &str,
) -> String {
    let mut ret = format!(
        "Search results for '{}':\n",
        file_name_to_label(&search_term)
    );
    for item in items.iter() {
        ret.push_str(&format!(
            "[{}]({}/{})\n",
            file_name_to_label(item),
            base_url,
            item
        ));
    }
    ret
}

pub(crate) fn tokenized_search(name: &str, search_term: &str) -> bool {
    let term = search_term.to_lowercase();
    let tokens = file_name_to_label(&name)
        .split(' ')
        .map(|x| x.to_lowercase())
        .filter(|x| x.parse::<f32>().is_err())
        .collect::<Vec<String>>();
    for token in tokens {
        if token == term {
            return true;
        }
    }
    false
}

pub(crate) fn get_random_file(files: Vec<String>) -> String {
    files
        .get(thread_rng().gen_range(0..files.len()))
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{
        escape_markdown_str, file_name_to_label, get_search_results, index_pictures,
        tokenized_search,
    };

    #[test]
    fn markdown_escape_test() {
        assert_eq!(r"John\_Doe\_1.jpg", escape_markdown_str("John_Doe_1.jpg"));
        assert_eq!(
            "[Test link](https://example.com)",
            escape_markdown_str("[Test link](https://example.com)")
        );
    }

    #[test]
    fn file_name_to_label_test() {
        assert_eq!(file_name_to_label("John_Doe_1.jpg"), "John Doe 1");
        assert!(!file_name_to_label("Jane_Doe.jpg").contains('_'))
    }

    #[test]
    fn search_matches_full_terms_test() {
        assert!(tokenized_search("John_Doe_1.jpg", "Doe"));
        assert!(tokenized_search("Jane_Doe.jpg", "Jane"));
    }

    #[test]
    fn search_matches_by_token() {
        let items = index_pictures("testdata");
        assert!(!items.is_empty());
        let results = get_search_results(items, "De");
        assert!(!results.contains(&String::from("Demi_Lovato.jpg")));
        assert!(results.contains(&String::from("Ana_De_Armas.jpg")));
    }
}
