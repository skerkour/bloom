//! Parsing and formatting various types.

/// Pad a given value if requested.
macro_rules! pad {
    ($f:ident, $padding:ident, $width:literal, $value:expr) => {
        match $padding {
            Padding::None => write!($f, "{}", $value),
            Padding::Space => write!($f, concat!("{:", stringify!($width), "}"), $value),
            Padding::Zero => write!($f, concat!("{:0", stringify!($width), "}"), $value),
        }
    };
}

pub(crate) mod date;
pub(crate) mod deferred_format;
#[allow(clippy::module_inception)]
pub(crate) mod format;
pub(crate) mod offset;
pub(crate) mod parse;
pub(crate) mod parse_items;
pub(crate) mod time;
pub(crate) mod well_known;

use crate::{error, Date, Time, UtcOffset};
use core::fmt::Formatter;
pub(crate) use deferred_format::DeferredFormat;
#[allow(unreachable_pub)] // rust-lang/rust#64762
pub use format::Format;
pub(crate) use parse::{parse, ParseResult, ParsedItems};
pub(crate) use parse_items::{parse_fmt_string, try_parse_fmt_string};

/// The type of padding to use when formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Padding {
    /// No padding. Minimizes width.
    None,
    /// Pad to the requisite width using spaces.
    Space,
    /// Pad to the requisite width using zeros.
    Zero,
}

/// Specifiers are similar to C's `strftime`, with some omissions and changes.
///
/// See the table in `lib.rs` for a description of each specifier (and
/// equivalences for combination specifiers).
#[allow(
    non_snake_case,
    non_camel_case_types,
    clippy::missing_docs_in_private_items
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Specifier {
    a,
    A,
    b,
    B,
    c,
    C { padding: Padding },
    d { padding: Padding },
    D,
    F,
    g { padding: Padding },
    G { padding: Padding },
    H { padding: Padding },
    I { padding: Padding },
    j { padding: Padding },
    m { padding: Padding },
    M { padding: Padding },
    N,
    p,
    P,
    r,
    R,
    S { padding: Padding },
    T,
    u,
    U { padding: Padding },
    V { padding: Padding },
    w,
    W { padding: Padding },
    y { padding: Padding },
    Y { padding: Padding },
    z,
}

/// Given all the information necessary, write the provided specifier to the
/// formatter.
fn format_specifier(
    f: &mut Formatter<'_>,
    date: Option<Date>,
    time: Option<Time>,
    offset: Option<UtcOffset>,
    specifier: Specifier,
) -> Result<(), error::Format> {
    /// Push the provided specifier to the list of items.
    macro_rules! specifier {
        ($type:ident :: $specifier_fn:ident ( $specifier:ident $(, $param:expr)? )) => {
            $type::$specifier_fn(
                f,
                match $type {
                    Some(v) => v,
                    None => return Err(error::Format::InsufficientTypeInformation),
                },
                $($param)?
            )?
        };
    }

    macro_rules! literal {
        ($string:literal) => {
            f.write_str($string)?
        };
    }

    use Specifier::*;
    match specifier {
        a => specifier!(date::fmt_a(a)),
        A => specifier!(date::fmt_A(A)),
        b => specifier!(date::fmt_b(b)),
        B => specifier!(date::fmt_B(B)),
        c => {
            specifier!(date::fmt_a(a));
            literal!(" ");
            specifier!(date::fmt_b(b));
            literal!(" ");
            specifier!(date::fmt_d(d, Padding::None));
            literal!(" ");
            specifier!(time::fmt_H(H, Padding::None));
            literal!(":");
            specifier!(time::fmt_M(M, Padding::Zero));
            literal!(":");
            specifier!(time::fmt_S(S, Padding::Zero));
            literal!(" ");
            specifier!(date::fmt_Y(Y, Padding::None));
        }
        C { padding } => specifier!(date::fmt_C(C, padding)),
        d { padding } => specifier!(date::fmt_d(d, padding)),
        D => {
            specifier!(date::fmt_m(m, Padding::None));
            literal!("/");
            specifier!(date::fmt_d(d, Padding::Zero));
            literal!("/");
            specifier!(date::fmt_y(y, Padding::Zero));
        }
        F => {
            specifier!(date::fmt_Y(Y, Padding::None));
            literal!("-");
            specifier!(date::fmt_m(m, Padding::Zero));
            literal!("-");
            specifier!(date::fmt_d(d, Padding::Zero));
        }
        g { padding } => specifier!(date::fmt_g(g, padding)),
        G { padding } => specifier!(date::fmt_G(G, padding)),
        H { padding } => specifier!(time::fmt_H(H, padding)),
        I { padding } => specifier!(time::fmt_I(I, padding)),
        j { padding } => specifier!(date::fmt_j(j, padding)),
        m { padding } => specifier!(date::fmt_m(m, padding)),
        M { padding } => specifier!(time::fmt_M(M, padding)),
        N => specifier!(time::fmt_N(N)),
        p => specifier!(time::fmt_p(p)),
        P => specifier!(time::fmt_P(P)),
        r => {
            specifier!(time::fmt_I(I, Padding::None));
            literal!(":");
            specifier!(time::fmt_M(M, Padding::Zero));
            literal!(":");
            specifier!(time::fmt_S(S, Padding::Zero));
            literal!(" ");
            specifier!(time::fmt_p(p));
        }
        R => {
            specifier!(time::fmt_H(H, Padding::None));
            literal!(":");
            specifier!(time::fmt_M(M, Padding::Zero));
        }
        S { padding } => specifier!(time::fmt_S(S, padding)),
        T => {
            specifier!(time::fmt_H(H, Padding::None));
            literal!(":");
            specifier!(time::fmt_M(M, Padding::Zero));
            literal!(":");
            specifier!(time::fmt_S(S, Padding::Zero));
        }
        u => specifier!(date::fmt_u(u)),
        U { padding } => specifier!(date::fmt_U(U, padding)),
        V { padding } => specifier!(date::fmt_V(V, padding)),
        w => specifier!(date::fmt_w(w)),
        W { padding } => specifier!(date::fmt_W(W, padding)),
        y { padding } => specifier!(date::fmt_y(y, padding)),
        Y { padding } => specifier!(date::fmt_Y(Y, padding)),
        z => specifier!(offset::fmt_z(z)),
    }

    Ok(())
}

/// An enum that can store both literals and specifiers.
#[allow(variant_size_differences, single_use_lifetimes)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FormatItem<'a> {
    /// A value that should be printed as-is.
    Literal(&'a str),
    /// A value that needs to be interpreted when formatting.
    Specifier(Specifier),
}
