use std::fmt::Debug;

pub trait XssSanitizer: Send + Sync + Debug {
    fn sanitize(&self, input: &str) -> Result<String, crate::Error>;
    fn escape(&self, input: &str) -> Result<String, crate::Error>;
}
