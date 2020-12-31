use proc_macro2::Span;
#[allow(unused_imports)]
use standback::prelude::*;
use std::{
    fmt::{Debug, Display},
    ops::RangeBounds,
    str::FromStr,
};
use syn::{LitInt, Result};

pub(crate) trait LitIntExtension {
    fn create<T: Display>(value: T) -> Self;
    fn with_span(self, span: Span) -> Self;
    fn ensure_in_range(&self, range: impl RangeBounds<isize> + Debug) -> Result<()>;
    fn value<T: FromStr + Display>(&self) -> Result<T>
    where
        T::Err: Display;
}

impl LitIntExtension for LitInt {
    fn create<T: Display>(value: T) -> Self {
        Self::new(&value.to_string(), Span::call_site())
    }

    fn with_span(mut self, span: Span) -> Self {
        self.set_span(span);
        self
    }

    #[allow(unstable_name_collisions)]
    fn ensure_in_range(&self, range: impl RangeBounds<isize> + Debug) -> Result<()> {
        if range.contains(&self.value()?) {
            Ok(())
        } else {
            error!(self.span(), "value must be in range {:?}", range)
        }
    }

    fn value<T: FromStr + Display>(&self) -> Result<T>
    where
        T::Err: Display,
    {
        match self.base10_parse() {
            Ok(value) => Ok(value),
            Err(e) => error!(self.span(), "{}", e),
        }
    }
}
