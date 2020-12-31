use crate::{
    ext::LitIntExtension,
    time_crate,
    time_crate::{days_in_year, days_in_year_month, weeks_in_year},
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Ident, LitInt, Result, Token,
};

pub(crate) struct Date {
    year: i32,
    ordinal: u16,
}

impl Parse for Date {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let (year, year_span) = {
            let year_sign = if input.peek(Token![+]) {
                input.parse::<Token![+]>()?;
                1
            } else if input.peek(Token![-]) {
                input.parse::<Token![-]>()?;
                -1
            } else {
                1
            };
            let year = input.parse::<LitInt>()?;
            (year_sign * year.value::<i32>()?, year.span())
        };
        input.parse::<Token![-]>()?;

        // year-week-day
        let (year, ordinal) = if input.peek(Ident) {
            let week = {
                let week = input.parse::<Ident>()?;
                let week_str = week.to_string();
                if week_str.starts_with('W') {
                    LitInt::new(&week_str[1..], week.span())
                } else {
                    return error!(week.span(), "expected week value to start with `W`");
                }
            };
            input.parse::<Token![-]>()?;
            let day = input.parse::<LitInt>()?;

            week.ensure_in_range(0..=weeks_in_year(year) as isize)?;
            day.ensure_in_range(1..=7)?;

            time_crate::Date::from_iso_ywd_unchecked(year, week.value()?, day.value()?).as_yo()
        }
        // year-month-day
        else if input.peek2(Token![-]) {
            let month = input.parse::<LitInt>()?;
            input.parse::<Token![-]>()?;
            let day = input.parse::<LitInt>()?;

            month.ensure_in_range(1..=12)?;
            day.ensure_in_range(1..=days_in_year_month(year, month.value()?) as isize)?;

            time_crate::Date::from_ymd_unchecked(year, month.value()?, day.value()?).as_yo()
        }
        // year-ordinal
        else {
            let ordinal = input.parse::<LitInt>()?;
            ordinal.ensure_in_range(1..=days_in_year(year) as isize)?;
            (year, ordinal.value()?)
        };

        // TODO(upstream) Swap out the following when dtolnay/syn#748 is
        // published on crates.io. Be sure to update Cargo.toml for the minimum
        // version.
        // LitInt::create(year).using_span(year_span).ensure_in_range(-100_000..=100_000)?;
        if year < -100_000 || year > 100_000 {
            return error!(year_span, "value must be in the range -100_000..=100_000");
        }

        Ok(Self { year, ordinal })
    }
}

impl ToTokens for Date {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { year, ordinal } = self;

        tokens.extend(quote! {
            ::time::internals::Date::from_yo_unchecked(#year, #ordinal)
        })
    }
}
