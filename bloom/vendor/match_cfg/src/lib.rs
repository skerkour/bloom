//! A convenience macro to ergonomically define an item depending on a large
//! number of `#[cfg]` parameters. Structured like match statement, the first
//! matching branch is the item that gets emitted.

#![cfg_attr(not(feature = "use_core"), feature(no_core))]
#![doc(html_root_url = "https://docs.rs/cfg-if")]
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(not(feature = "use_core"), no_core)]
#![cfg_attr(feature = "use_core", no_std)]

/// The macro provided by this crate, `match_cfg`, is similar to the `if/elif` C
/// preprocessor directives and allows defining a cascade of `#[cfg]` cases,
/// emitting the implementation which matches first.
///
/// This conveniently allows providing a long list `#[cfg]`'d blocks of code
/// without having to rewrite each `cfg()` clause multiple times.
///
/// # Example
///
/// ```
/// #[macro_use(match_cfg)]
/// extern crate match_cfg;
///
/// match_cfg! {
///     #[cfg(unix)] => {
///         fn foo() { /* unix specific functionality */ }
///     }
///     #[cfg(target_pointer_width = "32")] => {
///         fn foo() { /* non-unix, 32-bit functionality */ }
///     }
///     _ => {
///         fn foo() { /* fallback implementation */ }
///     }
/// }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! match_cfg {
    (#[cfg($cfg:meta)] => { $($i:item)* }) => {
        $(
            #[cfg($cfg)] $i
        )*
    };
    (#[cfg($cfg:meta)] @ #[cfg($cfg_not:meta)] => { $($i:item)* }) => {
        $(
            #[cfg(not($cfg_not))] #[cfg($cfg)] $i
        )*
    };
    (_ => { $($i:item)* }) => { $( $i )* };
    (_ @ #[cfg($cfg_not:meta)] => { $($i:item)* }) => {
        $(
            #[cfg(not($cfg_not))] $i
        )*
    };
    (
        #[cfg($cfg0:meta)] => { $($i:item)* }
        $(#[cfg($cfgs:meta)] => { $($is:item)* })*
    ) => {
        match_cfg! {
            #[cfg($cfg0)] => { $($i)* }
        }
        $(
            match_cfg! {
                #[cfg($cfgs)] @ #[cfg($cfg0)] => { $($is)* }
            }
        )*
    };
    (
        $(#[cfg($cfgs:meta)] => { $($is:item)* })*
        _ => { $($ni:item)* }
    ) => {
        match_cfg! {
            $( #[cfg($cfgs)] => { $($is)* } )*
        }
        match_cfg! {
            _ @ #[cfg(any($($cfgs),*))] => { $($ni)* }
        }
    };
}

#[cfg(test)]
mod tests {
    match_cfg! {
        #[cfg(target_pointer_width = "64")] => { fn f0_() -> bool { true }}
    }
    match_cfg! {
        #[cfg(unix)] => { fn f1_() -> bool { true }}
        #[cfg(any(target_os = "macos", target_os = "linux"))] => { fn f1_() -> bool { false }}
    }

    match_cfg! {
        #[cfg(target_pointer_width = "64")] => { fn f2_() -> bool { true }}
        #[cfg(target_pointer_width = "32")] => { fn f2_() -> bool { false }}
    }

    match_cfg! {
        #[cfg(target_pointer_width = "8")] => { fn f3_() -> i32 { 0 }}
        #[cfg(target_pointer_width = "16")] => { fn f3_() -> i32 { 1 }}
        _ => { fn f3_() -> i32 { 2 }}
    }

    #[test]
    fn tests() {
        #[cfg(target_pointer_width = "64")]
        {
            assert!(f0_());
        }
        #[cfg(unix)]
        {
            assert!(f1_());
        }
        assert!(f2_());
        assert_eq!(f3_(), 2);
    }

    match_cfg! {
        #[cfg(test)] => {
            use core::option::Option as Option2;
            fn works1() -> Option2<u32> { Some(1) }
        }
        _ => {
            fn works1() -> Option<u32> { None }
        }
    }

    match_cfg! {
        #[cfg(foo)] => {
            fn works2() -> bool { false }
        }
        #[cfg(test)] => {
            fn works2() -> bool { true }
        }
        _ => {
            fn works2() -> bool { false }
        }
    }

    match_cfg! {
        #[cfg(foo)] => {
            fn works3() -> bool { false }
        }
        _ => {
            fn works3() -> bool { true }
        }
    }

    match_cfg! {
        #[cfg(test)] => {
            use core::option::Option as Option3;
            fn works4() -> Option3<u32> { Some(1) }
        }
    }

    match_cfg! {
        #[cfg(foo)] => {
            fn works5() -> bool { false }
        }
        #[cfg(test)] => {
            fn works5() -> bool { true }
        }
    }

    #[test]
    fn it_works() {
        assert!(works1().is_some());
        assert!(works2());
        assert!(works3());
        assert!(works4().is_some());
        assert!(works5());
    }
}
