pub(crate) fn file_name_to_label(msg: String) -> String {
    escape_markdown_str(msg).replace(r"\_", " ").replace(".jpg", "")
}

pub(crate) fn escape_markdown_str(msg: String) -> String {
    msg.replace("_", r"\_")
}

pub(crate) fn join_results_to_string(items: Vec<String>, base_url: &str) -> String {
    let mut ret = String::new();
    for item in items.iter() {
        ret.push_str(&format!("[{}]({}/{})\n", item, base_url, item));
    }
    ret
}
