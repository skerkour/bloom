//! Macros for use with Tokio
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

/// Marks async function to be executed by actix system.
///
/// ## Usage
///
/// ```rust
/// #[actix_rt::main]
/// async fn main() {
///     println!("Hello world");
/// }
/// ```
#[allow(clippy::needless_doctest_main)]
#[proc_macro_attribute]
#[cfg(not(test))] // Work around for rust-lang/rust#62127
pub fn main(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(item as syn::ItemFn);
    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &mut input.sig;
    let body = &input.block;
    let name = &sig.ident;

    if sig.asyncness.is_none() {
        return syn::Error::new_spanned(sig.fn_token, "only async fn is supported")
            .to_compile_error()
            .into();
    }

    sig.asyncness = None;

    if cfg!(feature = "actix-reexport") {
        (quote! {
            #(#attrs)*
            #vis #sig {
                actix::System::new(stringify!(#name))
                    .block_on(async move { #body })
            }
        })
        .into()
    } else {
        (quote! {
            #(#attrs)*
            #vis #sig {
                actix_rt::System::new(stringify!(#name))
                    .block_on(async move { #body })
            }
        })
        .into()
    }
}

/// Marks async test function to be executed by actix runtime.
///
/// ## Usage
///
/// ```no_run
/// #[actix_rt::test]
/// async fn my_test() {
///     assert!(true);
/// }
/// ```
#[proc_macro_attribute]
pub fn test(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(item as syn::ItemFn);
    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &mut input.sig;
    let body = &input.block;
    let mut has_test_attr = false;

    for attr in attrs {
        if attr.path.is_ident("test") {
            has_test_attr = true;
        }
    }

    if sig.asyncness.is_none() {
        return syn::Error::new_spanned(
            input.sig.fn_token,
            format!("only async fn is supported, {}", input.sig.ident),
        )
        .to_compile_error()
        .into();
    }

    sig.asyncness = None;

    let result = if has_test_attr {
        quote! {
            #(#attrs)*
            #vis #sig {
                actix_rt::System::new("test")
                    .block_on(async { #body })
            }
        }
    } else {
        quote! {
            #[test]
            #(#attrs)*
            #vis #sig {
                actix_rt::System::new("test")
                    .block_on(async { #body })
            }
        }
    };

    result.into()
}
