#![forbid(unsafe_code)]
#![deny(
    anonymous_parameters,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    const_err,
    illegal_floating_point_literal_pattern,
    late_bound_lifetime_arguments,
    path_statements,
    patterns_in_fns_without_body,
    clippy::all
)]
#![warn(
    unused_extern_crates,
    missing_copy_implementations,
    missing_debug_implementations,
    single_use_lifetimes,
    unused_qualifications,
    variant_size_differences,
    clippy::pedantic,
    clippy::nursery,
    clippy::decimal_literal_representation,
    clippy::get_unwrap,
    clippy::option_unwrap_used,
    clippy::print_stdout,
    clippy::result_unwrap_used
)]
#![allow(
    clippy::inline_always,
    clippy::cast_possible_wrap,
    clippy::cast_lossless,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::use_self, // Not supported in some situations in older compilers.
)]

// This is required on rustc < 1.42.0.
#[allow(unused_extern_crates)]
extern crate proc_macro;

macro_rules! error {
    ($message:literal) => {
        error!(::proc_macro2::Span::call_site(), $message)
    };

    ($span:expr, $message:literal) => {
        Err(::syn::Error::new($span, $message))
    };

    ($span:expr, $($args:expr),+) => {
        Err(::syn::Error::new($span, format!($($args),+)))
    };
}

mod kw {
    use syn::custom_keyword;
    custom_keyword!(am);
    custom_keyword!(pm);
    custom_keyword!(AM);
    custom_keyword!(PM);
    custom_keyword!(utc);
    custom_keyword!(UTC);
}

mod date;
mod ext;
mod offset;
mod time;
mod time_crate;

use date::Date;
use offset::Offset;
use proc_macro_hack::proc_macro_hack;
use quote::ToTokens;
use syn::parse_macro_input;
use time::Time;

macro_rules! impl_macros {
    ($($name:ident : $type:ty),* $(,)?) => {
        $(
            #[proc_macro_hack]
            pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
                parse_macro_input!(input as $type).to_token_stream().into()
            }
        )*
    };
}

impl_macros! {
    time: Time,
    offset: Offset,
    date: Date,
}
