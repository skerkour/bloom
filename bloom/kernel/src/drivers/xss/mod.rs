use std::fmt::Debug;

pub mod stdx;

pub trait XssSanitizer: Send + Sync + Debug {
    fn sanitize(&self, input: &str) -> Result<String, crate::Error>;
    fn escape(&self, input: &str) -> String;
}
