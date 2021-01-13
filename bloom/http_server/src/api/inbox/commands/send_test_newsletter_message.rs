use crate::{
    api::inbox::model::{self, input},
    ServerContext,
};
use actix_web::web;
use kernel::{http::api, Actor};
use std::sync::Arc;
use web::Json;

pub async fn send_test_newsletter_message(
    _ctx: web::Data<Arc<ServerContext>>,
    _input: Json<input::SendNewsletterMessage>,
    _actor: Actor,
) -> Result<api::Response<model::NewsletterMessage>, kernel::Error> {
    todo!();
}
