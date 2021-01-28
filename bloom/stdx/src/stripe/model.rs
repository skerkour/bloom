use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct AddressParams {
    pub city: String,
    pub country: String,
    pub line1: String,
    pub line2: String,
    pub postal_code: String,
    pub state: String,
}

#[derive(Serialize, Deserialize)]
pub struct BillingPortalSessionParams {
    pub customer: String,
    pub return_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct BillingPortalSession {
    pub created: i64,
    pub customer: String,
    pub id: String,
    pub livemode: bool,
    pub return_url: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct CheckoutSessionLineItemParams {
    pub price: String,
    pub quantity: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CheckoutSessionParams {
    pub customer: String,
    pub payment_method_type: Vec<String>,
    pub line_items: Vec<CheckoutSessionLineItemParams>,
    pub mode: String,
    pub success_url: String,
    pub cancel_url: String,
    pub subscription_data: Option<CheckoutSessionSubscriptionDataParams>,
}

#[derive(Serialize, Deserialize)]
pub struct CheckoutSessionSubscriptionDataParams {
    pub default_tax_rates: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Customer {
    pub id: String,
    // TODO
}

#[derive(Serialize, Deserialize)]
pub struct CustomerParams {
    pub email: String,
    pub address: AddressParams,
    pub tax_id_data: Vec<CustomerTaxIdDataParams>,
    pub metadata: Option<HashMap<String, String>>,
    pub expands: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct CustomerTaxIdDataParams {
    pub r#type: TaxIdType,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxIdType {
    EuVat,
}
