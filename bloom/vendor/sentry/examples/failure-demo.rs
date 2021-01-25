#![allow(deprecated)]

use failure::Fail;
use failure_ as failure;
use sentry_failure::capture_error;

#[derive(Fail, Debug)]
#[fail(display = "An error occurred with error code {}. ({})", code, message)]
struct MyError {
    code: i32,
    message: String,
}

fn execute() -> Result<(), failure::Error> {
    Err(MyError {
        code: 42,
        message: "Something went really wrong".into(),
    }
    .into())
}

fn main() {
    let _sentry = sentry::init(sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    });

    if let Err(err) = execute() {
        println!("error: {}", err);
        capture_error(&err);
    }
}
