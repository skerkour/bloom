use crate::{
    api::analytics::model::{input, Analytics},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn analytics(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::Analytics>,
    actor: Actor,
) -> Result<api::Response<Analytics>, kernel::Error> {
    let input = input.into_inner();
    let analytics = ctx.analytics_service.find_analytics(actor, input.namespace_id).await?;

    Ok(api::Response::ok(analytics.into()))
}
