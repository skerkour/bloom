use crate::{api::kernel::model, ServerContext};
use actix_web::web::{self, Json};
use kernel::{http::api, Actor};
use std::sync::Arc;

pub async fn billing_information(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<model::input::GetBillingInformation>,
    actor: Actor,
) -> Result<api::Response<model::BillingInformation>, kernel::Error> {
    let input = input.into_inner();
    let info = ctx
        .kernel_service
        .get_billing_information(actor, input.namespace_id)
        .await?;

    Ok(api::Response::ok(info.into()))
}
