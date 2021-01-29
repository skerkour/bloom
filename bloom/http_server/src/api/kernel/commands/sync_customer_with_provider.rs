use crate::{api::kernel::model, ServerContext};
use actix_web::web::{self, Json};
use kernel::{http::api, Actor};
use std::sync::Arc;

pub async fn sync_customer_with_provider(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<model::input::SyncCustomerWithProvider>,
    actor: Actor,
) -> Result<api::Response<model::Success>, kernel::Error> {
    let input = input.into_inner();
    ctx.kernel_service
        .sync_customer_with_stripe(Some(actor), Some(input.namespace_id), None)
        .await?;

    Ok(api::Response::ok(true.into()))
}
