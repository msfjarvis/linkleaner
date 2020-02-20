pub fn escape_markdown_str(msg: String) -> String {
    msg.replace("_", r"\_")
}
