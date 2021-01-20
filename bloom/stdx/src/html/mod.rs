pub fn escape(input: &str) -> String {
    askama_escape::escape(input, askama_escape::Html).to_string()
}
