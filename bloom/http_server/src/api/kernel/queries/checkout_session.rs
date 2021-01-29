use crate::{api::kernel::model, ServerContext};
use actix_web::web::{self, Json};
use kernel::{http::api, service::GetStripeCheckoutSessionInput, Actor};
use std::sync::Arc;

pub async fn checkout_session(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<model::input::GetCheckoutSession>,
    actor: Actor,
) -> Result<api::Response<model::CheckoutSession>, kernel::Error> {
    let input = input.into_inner();
    let service_input = GetStripeCheckoutSessionInput {
        namespace_id: input.namespace_id,
        plan: input.plan,
    };
    let session_id = ctx
        .kernel_service
        .get_stripe_checkout_session(actor, service_input)
        .await?;

    Ok(api::Response::ok(model::CheckoutSession {
        session_id,
    }))
}
