use std::fmt::Debug;

pub mod stdx;

pub trait XssSanitizer: Send + Sync + Debug {
    fn sanitize(&self, input: &str) -> String;
    fn escape(&self, input: &str) -> String;
}
