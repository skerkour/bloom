use crate::{
    ext::LitIntExtension,
    kw::{utc, UTC},
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    LitInt, Result, Token,
};

#[repr(i8)]
enum Direction {
    East = 1,
    West = -1,
}

pub(crate) struct Offset {
    pub(crate) offset: i32,
}

impl Parse for Offset {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        if input.peek(utc) {
            input.parse::<utc>()?;
            return Ok(Self { offset: 0 });
        }
        if input.peek(UTC) {
            input.parse::<UTC>()?;
            return Ok(Self { offset: 0 });
        }

        let direction = if input.peek(Token![+]) {
            input.parse::<Token![+]>()?;
            Direction::East
        } else if input.peek(Token![-]) {
            input.parse::<Token![-]>()?;
            Direction::West
        } else {
            return error!("offset must have an explicit sign");
        };

        let hour = input.parse::<LitInt>()?;

        // Minutes are optional, defaulting to zero.
        let minute = if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            input.parse::<LitInt>()?
        } else {
            LitInt::create(0)
        };

        // Seconds are optional, defaulting to zero.
        let second = if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            input.parse::<LitInt>()?
        } else {
            LitInt::create(0)
        };

        // Ensure none of the components are out of range.
        hour.ensure_in_range(0..24)?;
        minute.ensure_in_range(0..60)?;
        second.ensure_in_range(0..60)?;

        let offset = direction as i32
            * (hour.value::<i32>()? * 3_600
                + minute.value::<i32>()? * 60
                + second.value::<i32>()?);

        Ok(Self { offset })
    }
}

impl ToTokens for Offset {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { offset } = self;
        tokens.extend(quote! { ::time::UtcOffset::seconds(#offset) });
    }
}
