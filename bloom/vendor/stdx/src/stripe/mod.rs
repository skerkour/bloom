use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

pub mod model;

// TODO: use https://docs.rs/async-stripe ?

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("stripe: Unknown: {0}")]
    Unknown(String),
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Unknown(err.to_string())
    }
}

impl std::convert::From<serde_qs::Error> for Error {
    fn from(err: serde_qs::Error) -> Self {
        Error::Unknown(err.to_string())
    }
}

/// An enum representing the versions of the Stripe API.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ApiVersion {
    #[serde(rename = "2019-09-09")]
    V2019_09_09,
    #[serde(rename = "2020-08-27")]
    V2020_08_27,
}

impl ApiVersion {
    pub fn as_str(self) -> &'static str {
        match self {
            ApiVersion::V2019_09_09 => "2019-09-09",
            ApiVersion::V2020_08_27 => "2020-08-27",
        }
    }
}

#[derive(Serialize)]
pub struct Expand<'a> {
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],
}

impl Expand<'_> {
    pub(crate) fn is_empty(expand: &[&str]) -> bool {
        expand.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    secret_key: String,
    http_client: reqwest::Client,
    api_version: ApiVersion,
    api_base_url: String,
}

impl Client {
    pub fn new(secret_key: String) -> Client {
        let http_client = reqwest::Client::new();

        Client {
            http_client,
            secret_key,
            api_version: ApiVersion::V2020_08_27,
            api_base_url: String::from("https://api.stripe.com/v1"),
        }
    }

    pub async fn create_customer(&self, params: model::CustomerParams) -> Result<model::Customer, Error> {
        let url = self.url("/customers");
        let mut headers = self.headers();
        headers.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
        );

        let res = self
            .http_client
            .post(&url)
            .headers(headers)
            .body(serde_qs::to_string(&params)?)
            .send()
            .await?;
        let customer: model::Customer = res.json().await?;
        Ok(customer)
    }

    pub async fn update_customer(
        &self,
        customer_id: String,
        params: model::CustomerParams,
    ) -> Result<model::Customer, Error> {
        let mut headers = self.headers();
        headers.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
        );
        let expands = [
            "subscriptions",
            "sources",
            "default_source",
            "subscriptions.data.plan",
            "subscriptions.data.plan.product",
            "invoice_settings.default_payment_method",
        ];
        let expand = Expand {
            expand: &expands,
        };
        let url = self.url_with_params(&format!("/customers/{}", &customer_id), &expand)?;

        let res = self
            .http_client
            .post(&url)
            .headers(headers)
            .body(serde_qs::to_string(&params)?)
            .send()
            .await?;

        let res_status = res.status();
        if !res_status.is_success() {
            let err: model::Error = res.json().await?;
            return Err(Error::Unknown(err.error.message));
        }

        let customer: model::Customer = res.json().await?;
        Ok(customer)
    }

    pub async fn get_customer(
        &self,
        customer_id: &str,
        params: model::CustomerParams,
    ) -> Result<model::Customer, Error> {
        if params.expand.is_none() {
            return Err(Error::Unknown("stripe.get_customer: expand is missing".to_string()));
        }

        let expand = params.expand.unwrap().clone();
        let expand: Vec<&str> = expand.iter().map(|s| s.as_ref()).collect();
        let expand = Expand {
            expand: &expand,
        };
        let url = self.url_with_params(&format!("/customers/{}", &customer_id), &expand)?;

        let res = self.http_client.get(&url).headers(self.headers()).send().await?;

        let res_status = res.status();
        if !res_status.is_success() {
            let err: model::Error = res.json().await?;
            return Err(Error::Unknown(err.error.message));
        }

        let customer: model::Customer = res.json().await?;
        Ok(customer)
    }

    pub async fn new_billing_portal_session(
        &self,
        params: model::BillingPortalSessionParams,
    ) -> Result<model::BillingPortalSession, Error> {
        let url = self.url("/billing_portal/sessions");
        let mut headers = self.headers();
        headers.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
        );

        let res = self
            .http_client
            .post(&url)
            .headers(headers)
            .body(serde_qs::to_string(&params)?)
            .send()
            .await?;

        let res_status = res.status();
        if !res_status.is_success() {
            let err: model::Error = res.json().await?;
            return Err(Error::Unknown(err.error.message));
        }

        let session: model::BillingPortalSession = res.json().await?;
        Ok(session)
    }

    pub async fn new_checkout_session(
        &self,
        params: model::CheckoutSessionParams,
    ) -> Result<model::CheckoutSession, Error> {
        let url = self.url("/checkout/sessions");
        let mut headers = self.headers();
        headers.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
        );

        let res = self
            .http_client
            .post(&url)
            .headers(headers)
            .body(serde_qs::to_string(&params)?)
            .send()
            .await?;

        let res_status = res.status();
        if !res_status.is_success() {
            let err: model::Error = res.json().await?;
            return Err(Error::Unknown(err.error.message));
        }

        let session: model::CheckoutSession = res.json().await?;
        Ok(session)
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(&format!("Bearer {}", self.secret_key)).unwrap(),
        );
        headers.insert(
            HeaderName::from_static("stripe-version"),
            HeaderValue::from_str(self.api_version.as_str()).unwrap(),
        );
        headers
    }

    fn url(&self, path: &str) -> String {
        format!("{}/{}", self.api_base_url, path.trim_start_matches('/'))
    }

    fn url_with_params<P: serde::Serialize>(&self, path: &str, params: P) -> Result<String, Error> {
        let params = serde_qs::to_string(&params)?;
        Ok(format!("{}?{}", self.url(path), params))
    }
}
