use sentry_core::protocol::{Frame, Stacktrace};

use crate::utils::function_starts_with;

lazy_static::lazy_static! {
    static ref WELL_KNOWN_SYS_MODULES: Vec<&'static str> = vec![
        "std::",
        "core::",
        "alloc::",
        "backtrace::",
        "sentry::",
        "sentry_core::",
        "sentry_types::",
        // these are not modules but things like __rust_maybe_catch_panic
        "__rust_",
        "___rust_",
        // these are well-known library frames
        "anyhow::",
        "log::",
    ];

    static ref WELL_KNOWN_BORDER_FRAMES: Vec<&'static str> = vec![
        "std::panicking::begin_panic",
        "core::panicking::panic",
        // well-known library frames
        "anyhow::",
        "<sentry_log::Logger as log::Log>::log",
    ];

    // TODO: remove all of this together with the deprecated `error_chain` support
    static ref SECONDARY_BORDER_FRAMES: Vec<(&'static str, &'static str)> = vec![
        ("error_chain::make_backtrace", "<T as core::convert::Into<U>>::into")
    ];
}

/// A helper function to trim a stacktrace.
pub fn trim_stacktrace<F>(stacktrace: &mut Stacktrace, f: F)
where
    F: Fn(&Frame, &Stacktrace) -> bool,
{
    let known_cutoff = stacktrace
        .frames
        .iter()
        .rev()
        .position(|frame| match frame.function {
            Some(ref func) => is_well_known(&func) || f(frame, stacktrace),
            None => false,
        });

    if let Some(cutoff) = known_cutoff {
        let secondary = {
            let func = stacktrace.frames[stacktrace.frames.len() - cutoff - 1]
                .function
                .as_ref()
                .unwrap();

            SECONDARY_BORDER_FRAMES
                .iter()
                .filter_map(|&(primary, secondary)| {
                    if function_starts_with(func, primary) {
                        Some(secondary)
                    } else {
                        None
                    }
                })
                .next()
        };
        let trunc = stacktrace.frames.len() - cutoff - 1;
        stacktrace.frames.truncate(trunc);

        if let Some(secondary) = secondary {
            let secondary_cutoff =
                stacktrace
                    .frames
                    .iter()
                    .rev()
                    .position(|frame| match frame.function {
                        Some(ref func) => function_starts_with(&func, secondary),
                        None => false,
                    });

            if let Some(cutoff) = secondary_cutoff {
                let trunc = stacktrace.frames.len() - cutoff - 1;
                stacktrace.frames.truncate(trunc);
            }
        }
    }
}

/// Checks if a function is considered to be not in-app
pub fn is_sys_function(func: &str) -> bool {
    WELL_KNOWN_SYS_MODULES
        .iter()
        .any(|m| function_starts_with(func, m))
}

/// Checks if a function is a well-known system function
fn is_well_known(func: &str) -> bool {
    WELL_KNOWN_BORDER_FRAMES
        .iter()
        .any(|m| function_starts_with(&func, m))
}
