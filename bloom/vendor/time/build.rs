use std::env;
use version_check as rustc;

const MSRV: u16 = 32;
const NO_STD_MSRV: u16 = 36;

macro_rules! cfg_emit {
    ($s:ident) => {
        println!(concat!("cargo:rustc-cfg=", stringify!($s)));
    };
}

macro_rules! warning {
    ($($s:tt)*) => {
        println!("cargo:warning={}", format_args!($($s)*));
    };
}

fn main() {
    println!("cargo:rerun-if-env-changed=COMPILING_UNDER_CARGO_WEB");

    // Are we compiling with `cargo web`?
    if env::var("COMPILING_UNDER_CARGO_WEB") == Ok("1".into()) {
        cfg_emit!(__time_02_cargo_web);
    }

    // Treat nightly & dev compilers as the equivalent of the then-beta.
    // As features are never stabilized in patch versions, we can safely ignore it.
    let rustc_info = (rustc::Version::read(), rustc::Channel::read());
    let (effective_compiler_version, channel) = match rustc_info {
        (Some(version), Some(channel)) if channel.is_dev() => (version.to_mmp().1 - 2, channel),
        (Some(version), Some(channel)) if channel.is_nightly() => (version.to_mmp().1 - 1, channel),
        (Some(version), Some(channel)) => (version.to_mmp().1, channel),
        (None, _) | (_, None) => {
            warning!(
                "Unable to determine rustc version. Assuming rustc 1.{}.0.",
                MSRV
            );
            (
                MSRV,
                rustc::Channel::parse("1.0.0").expect("This is a generic stable version."),
            )
        }
    };

    // Warn if the version is below MSRV.
    if effective_compiler_version < MSRV {
        warning!(
            "The time crate has a minimum supported rust version of {}.",
            MSRV
        );
    }

    // Warn if the version is below `#![no_std]` MSRV.
    if effective_compiler_version < NO_STD_MSRV {
        #[cfg(not(feature = "std"))]
        warning!(
            "Using the time crate without the standard library enabled requires a global \
             allocator. This was stabilized in Rust {}. You can either upgrade or enable the \
             standard library.",
            NO_STD_MSRV
        );
    }

    // Warn if the `__doc` feature is used on stable or beta.
    if !channel.supports_features() {
        #[cfg(__time_02_docs)]
        warning!(
            "`--cfg __time_02_docs` requires a nightly compiler, and is intended for internal \
             usage only."
        );
    }

    // ==== features that affect runtime directly ====

    // `#[non_exhaustive]` was stabilized in 1.40.0.
    if effective_compiler_version >= 40 {
        cfg_emit!(__time_02_supports_non_exhaustive);
    }

    // `Instant::checked_add` and `Instant::checked_sub` were added in 1.34.0.
    // `NonZeroI*` was stabilized in 1.34.0.
    if effective_compiler_version >= 34 {
        cfg_emit!(__time_02_instant_checked_ops);
        cfg_emit!(__time_02_nonzero_signed);
    }

    // `use <trait> as _;` was stabilized in 1.33.0.
    if effective_compiler_version >= 33 {
        cfg_emit!(__time_02_use_trait_as_underscore);
    }
}
