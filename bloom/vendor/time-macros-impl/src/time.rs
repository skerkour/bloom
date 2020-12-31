use crate::ext::LitIntExtension;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    LitFloat, LitInt, Result, Token,
};

#[derive(PartialEq)]
enum AmPm {
    Am,
    Pm,
}

impl Parse for AmPm {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        use crate::kw::{am, pm, AM, PM};
        if input.peek(am) {
            input.parse::<am>()?;
            Ok(AmPm::Am)
        } else if input.peek(AM) {
            input.parse::<AM>()?;
            Ok(AmPm::Am)
        } else if input.peek(pm) {
            input.parse::<pm>()?;
            Ok(AmPm::Pm)
        } else if input.peek(PM) {
            input.parse::<PM>()?;
            Ok(AmPm::Pm)
        } else {
            error!("expected am or pm")
        }
    }
}

pub(crate) struct Time {
    pub(crate) hour: LitInt,
    pub(crate) minute: LitInt,
    pub(crate) second: LitInt,
    pub(crate) nanosecond: LitInt,
}

impl Parse for Time {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut hour = input.parse::<LitInt>()?;
        input.parse::<Token![:]>()?;
        let minute = input.parse::<LitInt>()?;

        // Seconds and nanoseconds are optional, defaulting to zero.
        let (second, nanosecond) = if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;

            if input.peek(LitFloat) {
                let float = input.parse::<LitFloat>()?;

                // Temporary value to satisfy the compiler.
                let float_str = float.to_string();
                let parts: Vec<_> = float_str.splitn(2, '.').collect();

                let seconds = LitInt::new(parts[0], float.span());
                let nanoseconds = {
                    // Prepend a zero to avoid having an empty string.
                    // Strip the suffix to avoid syn thinking it's a float.
                    let raw_padded = format!("0{}", parts[1].trim_end_matches(float.suffix()));

                    // Take an extra digit due to the padding.
                    let digits: String = raw_padded
                        .chars()
                        .filter(char::is_ascii_digit)
                        .take(10)
                        .collect();

                    // Scale the value based on how many digits were provided.
                    #[allow(clippy::cast_possible_truncation)]
                    let value = LitInt::new(&digits, float.span()).base10_parse::<usize>()?
                        * 10_usize.pow(10 - digits.len() as u32);

                    LitInt::create(value).with_span(float.span())
                };

                (seconds, nanoseconds)
            } else {
                (input.parse::<LitInt>()?, LitInt::create(0))
            }
        } else {
            (LitInt::create(0), LitInt::create(0))
        };

        let am_pm = input.parse::<AmPm>().ok();

        // Ensure none of the components are out of range.
        match am_pm {
            Some(am_pm) => {
                hour.ensure_in_range(1..=12)?;
                // Adjust the hour if necessary.
                hour = match (hour.value()?, am_pm) {
                    (12, AmPm::Am) => LitInt::create(0).with_span(hour.span()),
                    (value, AmPm::Pm) if value != 12 => {
                        LitInt::create(value + 12).with_span(hour.span())
                    }
                    _ => hour,
                }
            }
            None => hour.ensure_in_range(0..24)?,
        }
        minute.ensure_in_range(0..60)?;
        second.ensure_in_range(0..60)?;
        // This likely isn't necessary, but it can't hurt.
        nanosecond.ensure_in_range(0..1_000_000_000)?;

        Ok(Self {
            hour,
            minute,
            second,
            nanosecond,
        })
    }
}

impl ToTokens for Time {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            hour,
            minute,
            second,
            nanosecond,
        } = self;

        tokens.extend(quote! {
            ::time::internals::Time::from_hms_nanos_unchecked(#hour, #minute, #second, #nanosecond)
        });
    }
}
