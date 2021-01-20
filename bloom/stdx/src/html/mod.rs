pub fn escape(input: &str) -> String {
    askama_escape::escape(input, askama_escape::Html).to_string()
}

pub fn sanitize_xss(input: &str) -> String {
    ammonia::clean(input)
}
