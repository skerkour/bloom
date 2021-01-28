use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct CustomerParams {
    pub email: String,
    pub address: AddressParams,
    pub tax_id_data: CustomerTaxIdDataParams,
    pub metadata: HashMap<String, String>,
}

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
#[serde(rename_all = "snake_case")]
pub enum TaxIdType {
    EuVat,
}

#[derive(Serialize, Deserialize)]
pub struct CustomerTaxIdDataParams {
    pub r#type: TaxIdType,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Customer {
    pub id: String,
    // TODO
}
