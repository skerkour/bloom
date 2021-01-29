use crate::{api::kernel::model, ServerContext};
use actix_web::web::{self, Json};
use kernel::{http::api, Actor};
use std::sync::Arc;

pub async fn customer_portal(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<model::input::GetCustomerPortal>,
    actor: Actor,
) -> Result<api::Response<model::CustomerPortal>, kernel::Error> {
    let input = input.into_inner();
    let customer_portal_url = ctx
        .kernel_service
        .get_stripe_customer_portal_url(actor, input.namespace_id)
        .await?;

    Ok(api::Response::ok(model::CustomerPortal {
        customer_portal_url,
    }))
}
