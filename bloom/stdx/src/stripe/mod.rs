pub mod model;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("stripe: Unknown: {0}")]
    Unknown(String),
}

#[derive(Debug, Clone)]
pub struct Client {
    key: String,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(stripe_key: String) -> Client {
        let http_client = reqwest::Client::new();

        Client {
            http_client,
            key: stripe_key,
        }
    }

    pub async fn create_customer(&self, params: model::CustomerParams) -> Result<model::Customer, Error> {
        todo!();
    }

    pub async fn update_customer(&self, id: String, params: model::CustomerParams) -> Result<model::Customer, Error> {
        todo!();
    }

    pub async fn get_customer(&self, params: model::CustomerParams) -> Result<model::Customer, Error> {
        todo!();
    }

    pub async fn new_billing_portal_session(
        &self,
        params: model::BillingPortalSessionParams,
    ) -> Result<model::BillingPortalSession, Error> {
        todo!();
    }
}
