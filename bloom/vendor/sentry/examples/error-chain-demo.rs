#![allow(deprecated)]

#[macro_use]
extern crate error_chain_;

use sentry_error_chain::{capture_error_chain, ErrorChainIntegration};

error_chain! {
    errors {
        MyCoolError(t: &'static str) {
            description("my cool error happened")
            display("my cool error happened: {}", t)
        }
    }
}

fn execute() -> Result<()> {
    Err(ErrorKind::MyCoolError("Something went really wrong").into())
}

fn main() {
    let _sentry = sentry::init(
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        }
        .add_integration(ErrorChainIntegration),
    );

    if let Err(err) = execute() {
        println!("error: {}", err);
        capture_error_chain(&err);
    }
}
