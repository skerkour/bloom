#[macro_use]
extern crate nom;

use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use syn::{
    parse::{Parse, ParseBuffer},
    spanned::Spanned,
    Token,
};

mod generator;
mod parser;

/// Generate static tables and call macros
#[proc_macro]
pub fn derive(input: TokenStream) -> TokenStream {
    let Args {
        avx,
        pairs,
        print,
        simd,
    } = match syn::parse::<Builder>(input).and_then(Builder::build) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };
    let code = generator::generate(&parser::parse(&pairs), simd, avx);

    if print {
        eprintln!("{}", code);
    }

    code.parse().unwrap()
}

/// Proc macro arguments data
struct Args {
    pairs: String,
    avx: bool,
    print: bool,
    simd: bool,
}

/// Key-value argument
struct MetaOpt<Lit: Parse> {
    pub path: syn::Path,
    pub eq_token: Token![=],
    pub lit: Lit,
}

impl<Lit: Parse> Parse for MetaOpt<Lit> {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> syn::Result<Self> {
        Ok(Self {
            path: input.parse()?,
            eq_token: input.parse()?,
            lit: input.parse()?,
        })
    }
}

/// Proc macro arguments parser
struct Builder {
    pub pairs: syn::LitStr,
    pub comma: Option<Token![,]>,
    pub opts: Punctuated<MetaOpt<syn::LitBool>, Token![,]>,
}

impl Parse for Builder {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> syn::Result<Self> {
        Ok(Self {
            pairs: input.parse()?,
            comma: input.parse()?,
            opts: Punctuated::parse_terminated(input)?,
        })
    }
}

impl Builder {
    /// Consume and return arguments data
    fn build(self) -> syn::Result<Args> {
        let Builder { pairs, opts, .. } = self;
        let mut avx = true;
        let mut print = false;
        let mut simd = true;

        for MetaOpt { path, lit, .. } in opts {
            if path.is_ident("avx") {
                avx = lit.value
            } else if path.is_ident("print") {
                print = lit.value;
            } else if path.is_ident("simd") {
                simd = lit.value;
            } else {
                return Err(syn::Error::new(
                    path.span(),
                    format!("invalid attribute '{:?}'", path.get_ident()),
                ));
            }
        }

        Ok(Args {
            pairs: pairs.value(),
            avx,
            print,
            simd,
        })
    }
}
