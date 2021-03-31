use proc_macro::{Delimiter, Ident, Literal, Span, TokenStream, TokenTree};

use crate::{iter::TokenIter, to_tokens::ToTokens, utils::tt_span, Result};

pub(crate) struct Func {
    attrs: Vec<Attribute>,
    // [const] [async] [unsafe] [extern [<abi>]] fn
    sig: Vec<TokenTree>,
    body: TokenStream,
    pub(crate) print_const: bool,
}

pub(crate) fn parse_input(input: TokenStream) -> Result<Func> {
    let mut input = TokenIter::new(input);

    let attrs = parse_attrs(&mut input)?;
    let sig = parse_signature(&mut input);
    let body: TokenStream = input.collect();

    if body.is_empty()
        || !sig
            .iter()
            .any(|tt| if let TokenTree::Ident(i) = tt { i.to_string() == "fn" } else { false })
    {
        return Err(error!(
            Span::call_site(),
            "#[const_fn] attribute may only be used on functions"
        ));
    }

    Ok(Func { attrs, sig, body, print_const: true })
}

impl ToTokens for Func {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.attrs.iter().for_each(|attr| attr.to_tokens(tokens));
        if self.print_const {
            self.sig.iter().for_each(|attr| attr.to_tokens(tokens));
        } else {
            self.sig
                .iter()
                .filter(
                    |tt| if let TokenTree::Ident(i) = tt { i.to_string() != "const" } else { true },
                )
                .for_each(|tt| tt.to_tokens(tokens));
        }
        self.body.to_tokens(tokens);
    }
}

fn parse_signature(input: &mut TokenIter) -> Vec<TokenTree> {
    let mut sig = Vec::new();
    let mut has_const = false;
    loop {
        match input.peek() {
            None => break,
            Some(TokenTree::Ident(i)) if !has_const => {
                match &*i.to_string() {
                    "fn" => {
                        sig.push(TokenTree::Ident(Ident::new("const", i.span())));
                        sig.push(input.next().unwrap());
                        break;
                    }
                    "const" => {
                        has_const = true;
                    }
                    "async" | "unsafe" | "extern" => {
                        has_const = true;
                        sig.push(TokenTree::Ident(Ident::new("const", i.span())));
                    }
                    _ => {}
                }
                sig.push(input.next().unwrap());
            }
            Some(TokenTree::Ident(i)) if i.to_string() == "fn" => {
                sig.push(input.next().unwrap());
                break;
            }
            Some(_) => sig.push(input.next().unwrap()),
        }
    }
    sig
}

fn parse_attrs(input: &mut TokenIter) -> Result<Vec<Attribute>> {
    let mut attrs = Vec::new();
    loop {
        let pound_token = match input.peek() {
            Some(TokenTree::Punct(p)) if p.as_char() == '#' => input.next().unwrap(),
            _ => break,
        };
        let group = match input.peek() {
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Bracket => {
                input.next().unwrap()
            }
            tt => return Err(error!(tt_span(tt), "expected `[`")),
        };
        attrs.push(Attribute { pound_token, group });
    }
    Ok(attrs)
}

pub(crate) struct Attribute {
    // `#`
    pub(crate) pound_token: TokenTree,
    // `[...]`
    pub(crate) group: TokenTree,
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pound_token.to_tokens(tokens);
        self.group.to_tokens(tokens);
    }
}

pub(crate) struct LitStr {
    pub(crate) token: Literal,
    value: String,
}

impl LitStr {
    pub(crate) fn new(token: Literal) -> Result<Self> {
        let value = token.to_string();
        // unlike `syn::LitStr`, only accepts `"..."`
        if value.starts_with('"') && value.ends_with('"') {
            Ok(Self { token, value })
        } else {
            Err(error!(token.span(), "expected string literal"))
        }
    }

    pub(crate) fn value(&self) -> &str {
        &self.value[1..self.value.len() - 1]
    }

    pub(crate) fn span(&self) -> Span {
        self.token.span()
    }
}

impl ToTokens for LitStr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.token.to_tokens(tokens);
    }
}
