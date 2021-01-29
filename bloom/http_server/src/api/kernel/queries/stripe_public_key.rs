use crate::{api::kernel::model, ServerContext};
use actix_web::web;
use kernel::{http::api, Actor};
use std::sync::Arc;

pub async fn stripe_public_key(
    ctx: web::Data<Arc<ServerContext>>,
    actor: Actor,
) -> Result<api::Response<model::StripePublicKey>, kernel::Error> {
    let stripe_public_key = ctx.kernel_service.get_stripe_public_key(actor)?;

    Ok(api::Response::ok(model::StripePublicKey {
        stripe_public_key,
    }))
}
