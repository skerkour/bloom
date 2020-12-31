use std::fmt::Debug;

pub trait MarkdownRenderer: Send + Sync + Debug {
    fn render(&self, markdown: &str) -> Result<String, crate::Error>;
}
