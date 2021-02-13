use std::collections::HashMap;

use super::{Service, UpdateBillingInformationInput};
use crate::{
    consts::{self, BillingPlan},
    entities,
    errors::kernel::Error,
    Actor,
};
use consts::TaxIdType;
use entities::Customer;
use stdx::{chrono::Utc, stripe, ulid::Ulid, vat};

impl Service {
    pub async fn update_billing_information(
        &self,
        actor: Actor,
        input: UpdateBillingInformationInput,
    ) -> Result<entities::BillingInformation, crate::Error> {
        let actor = self.current_user(actor)?;

        self.check_namespace_membership(&self.db, &actor, input.namespace_id)
            .await?;

        if self.self_hosted() {
            return Err(Error::BillingCantBeAccessedWhenSelfHosting.into());
        }

        let namespace = self.repo.find_namespace_by_id(&self.db, input.namespace_id).await?;

        let countries = self.config.countries.clone();
        let mut tax_id: Option<String> = None;
        let mut stripe_tax: Option<String> = None;
        // unwrap is safe as if we are here we are not self-hosting
        let stripe_data = &self.config.stripe.as_ref().unwrap().data;
        let mut tax_id_type: Option<TaxIdType> = None;
        let mut stripe_customer_tax_data: Vec<stripe::model::CustomerTaxIdDataParams> = Vec::new();

        let email = input.email.trim().to_lowercase();
        self.validate_customer_email(&email)?;

        let name = input.name.trim().to_string();
        self.validate_customer_name(&name)?;

        let city = input.city.trim().to_string();
        self.validate_customer_city(&name)?;

        let postal_code = input.postal_code.trim().to_string();
        self.validate_customer_postal_code(&name)?;

        let state = input.state.trim().to_string();
        self.validate_customer_state(&name)?;

        let address_line1 = input.address_line1.trim().to_string();
        let address_line2 = input.address_line2.trim().to_string();
        self.validate_customer_address(&address_line1, &address_line2)?;

        let country_code = input.country_code.trim().to_uppercase();
        let country = countries.get(&country_code).ok_or(Error::CountryNotValid)?.to_string();

        if let Some(mut tax_id_input) = input.tax_id {
            tax_id_input = tax_id_input.trim().to_string();
            stripe_tax = Some(
                stripe_data
                    .taxes
                    .get(&country_code)
                    .ok_or(Error::TaxIdIsAcceptedOnlyForEu)?
                    .to_string(),
            );

            let vat_company = vat::validate_vat_number(&tax_id_input).await?;

            if vat_company.country_code != country_code {
                return Err(Error::VatDoesNotMatchCountry.into());
            }

            tax_id = Some(tax_id_input);
            tax_id_type = Some(TaxIdType::EuVat);
        }

        let now = Utc::now();
        let res = self.repo.find_customer_by_namespace_id(&self.db, namespace.id).await;
        let customer = match res {
            Ok(mut customer) => {
                customer.updated_at = now;
                customer.name = name;
                customer.email = email;
                customer.country = country;
                customer.country_code = country_code;
                customer.city = city;
                customer.postal_code = postal_code;
                customer.address_line1 = address_line1;
                customer.address_line2 = address_line2;
                customer.state = state;
                customer.tax_id = tax_id.clone();
                customer.stripe_tax_id = stripe_tax;
                customer.tax_id_type = tax_id_type;

                if let Some(tax_id) = tax_id {
                    stripe_customer_tax_data.push(stripe::model::CustomerTaxIdDataParams {
                        r#type: stripe::model::TaxIdType::EuVat,
                        value: tax_id,
                    })
                }

                // update stripe customer
                let customer_params = stripe::model::CustomerParams {
                    email: Some(customer.email.clone()),
                    address: Some(stripe::model::AddressParams {
                        city: customer.city.clone(),
                        country: customer.country.clone(),
                        line1: customer.address_line1.clone(),
                        line2: customer.address_line2.clone(),
                        postal_code: customer.postal_code.clone(),
                        state: customer.state.clone(),
                    }),
                    tax_id_data: Some(stripe_customer_tax_data),
                    ..Default::default()
                };
                // unwrap is safe as if we are here we are not self-hosting
                self.stripe_client
                    .as_ref()
                    .unwrap()
                    .update_customer(customer.stripe_customer_id.clone(), customer_params)
                    .await?;

                self.repo.update_customer(&self.db, &customer).await?;

                Ok(customer)
            }
            Err(Error::CustomerNotFound) => {
                let mut customer = Customer {
                    id: Ulid::new().into(),
                    created_at: now,
                    updated_at: now,
                    subscription_updated_at: now,
                    plan: BillingPlan::Free,
                    name,
                    email,
                    country,
                    country_code,
                    city,
                    postal_code,
                    address_line1,
                    address_line2,
                    state,
                    tax_id_type,
                    tax_id: tax_id.clone(),
                    stripe_customer_id: String::new(),
                    stripe_subscription_id: None,
                    stripe_product_id: None,
                    stripe_price_id: None,
                    stripe_tax_id: stripe_tax,
                    stripe_default_payment_method_id: None,
                    namespace_id: Some(namespace.id),
                };

                if let Some(tax_id) = tax_id {
                    stripe_customer_tax_data.push(stripe::model::CustomerTaxIdDataParams {
                        r#type: stripe::model::TaxIdType::EuVat,
                        value: tax_id,
                    })
                }

                // create stripe customer
                let mut customer_metadata = HashMap::new();
                customer_metadata.insert(String::from("namespace.id"), namespace.id.to_hyphenated().to_string());
                customer_metadata.insert(String::from("customer.id"), customer.id.to_hyphenated().to_string());

                let customer_params = stripe::model::CustomerParams {
                    email: Some(customer.email.clone()),
                    address: Some(stripe::model::AddressParams {
                        city: customer.city.clone(),
                        country: customer.country.clone(),
                        line1: customer.address_line1.clone(),
                        line2: customer.address_line2.clone(),
                        postal_code: customer.postal_code.clone(),
                        state: customer.state.clone(),
                    }),
                    tax_id_data: Some(stripe_customer_tax_data),
                    metadata: Some(customer_metadata),
                    ..Default::default()
                };
                // unwrap is safe as if we are here we are not self-hosting
                let stripe_customer = self
                    .stripe_client
                    .as_ref()
                    .unwrap()
                    .create_customer(customer_params)
                    .await?;

                customer.stripe_customer_id = stripe_customer.id;
                self.repo.create_customer(&self.db, &customer).await?;

                Ok(customer)
            }
            Err(err) => Err(err),
        }?;

        Ok(entities::BillingInformation {
            total_storage: self.get_storage_for_plan(namespace.plan),
            namespace,
            customer: Some(customer),
        })
    }
}
