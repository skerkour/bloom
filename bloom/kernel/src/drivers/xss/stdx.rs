use crate::Error;

#[derive(Debug, Clone)]
pub struct StdxXssSanitizer {}

impl StdxXssSanitizer {
    pub fn new() -> StdxXssSanitizer {
        StdxXssSanitizer {}
    }
}

impl super::XssSanitizer for StdxXssSanitizer {
    fn sanitize(&self, input: &str) -> Result<String, Error> {
        todo!();
    }
    fn escape(&self, input: &str) -> Result<String, Error> {
        todo!();
    }
}
