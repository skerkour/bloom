mod format;
mod rates;
mod validate;

pub use format::*;
pub use rates::*;
pub use validate::*;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("vat: Unknown")]
    Unknown,
    #[error("vat: Invalid country: {0}")]
    InvalidCountry(String),
    #[error("vat: Invalid VAT number: {0}")]
    InvalidVatNumber(String),
    #[error("vat: calling VAT API: {0}")]
    VatApi(String),
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::VatApi(err.to_string())
    }
}
