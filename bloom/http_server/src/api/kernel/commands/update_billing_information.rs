use crate::{api::kernel::model, ServerContext};
use actix_web::web::{self, Json};
use kernel::{http::api, service::UpdateBillingInformationInput, Actor};
use std::sync::Arc;

pub async fn update_billing_information(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<model::input::UpdateBillingInformation>,
    actor: Actor,
) -> Result<api::Response<model::BillingInformation>, kernel::Error> {
    let input = input.into_inner();
    let service_input = UpdateBillingInformationInput {
        namespace_id: input.namespace_id,
        name: input.name,
        email: input.email,
        country_code: input.country_code,
        city: input.city,
        postal_code: input.postal_code,
        address_line1: input.address_line1,
        address_line2: input.address_line2,
        state: input.state,
        tax_id: input.tax_id,
    };
    let info = ctx
        .kernel_service
        .update_billing_information(actor, service_input)
        .await?;

    Ok(api::Response::ok(info.into()))
}
