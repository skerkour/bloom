use crate::{
    api::{inbox::model::input, kernel::model::Success},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn delete_contact(
    _ctx: web::Data<Arc<ServerContext>>,
    _input: Json<input::DeleteContact>,
    _actor: Actor,
) -> Result<api::Response<Success>, kernel::Error> {
    todo!();
}
