use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AddressParams {
    pub city: String,
    pub country: String,
    pub line1: String,
    pub line2: String,
    pub postal_code: String,
    pub state: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BillingPortalSessionParams {
    pub customer: String,
    pub return_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BillingPortalSession {
    pub created: i64,
    pub customer: String,
    pub id: String,
    pub livemode: bool,
    pub return_url: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CheckoutSession {
    pub id: String,
    // TODO
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CheckoutSessionLineItemParams {
    pub price: String,
    pub quantity: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CheckoutSessionParams {
    pub customer: String,
    pub payment_method_types: Vec<String>,
    pub line_items: Vec<CheckoutSessionLineItemParams>,
    pub mode: String,
    pub success_url: String,
    pub cancel_url: String,
    pub subscription_data: Option<CheckoutSessionSubscriptionDataParams>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CheckoutSessionSubscriptionDataParams {
    pub default_tax_rates: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Customer {
    pub id: String,
    pub subscriptions: Option<SubscriptionList>,
    pub invoice_settings: Option<CustomerInvoiceSettings>,
    // TODO
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubscriptionList {
    pub data: Vec<Subscription>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CustomerParams {
    pub email: Option<String>,
    pub address: Option<AddressParams>,
    pub tax_id_data: Option<Vec<CustomerTaxIdDataParams>>,
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip)]
    pub expand: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomerTaxIdDataParams {
    pub r#type: TaxIdType,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomerInvoiceSettings {
    pub default_payment_method: Option<PaymentMethod>,
    // TODO
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethod {
    pub id: String,
    // TODO
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Plan {
    pub id: String,
    pub product: Product,
    // TODO
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Product {
    pub id: String,
    // TODO
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Subscription {
    pub id: String,
    pub plan: Plan,
    // TODO
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TaxIdType {
    EuVat,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Error {
    pub error: ErrorInner,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ErrorInner {
    pub r#type: String,
    pub message: String,
}
