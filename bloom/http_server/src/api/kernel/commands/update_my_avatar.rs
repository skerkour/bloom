use crate::{api::kernel::model, ServerContext};
use actix_multipart::Multipart;
use actix_web::web;
use kernel::{consts, http::api, service, Actor};
use std::{io::Write, sync::Arc};
use stdx::futures::{StreamExt, TryStreamExt};

pub async fn update_my_avatar(
    ctx: web::Data<Arc<ServerContext>>,
    mut payload: Multipart,
    actor: Actor,
) -> Result<api::Response<model::User>, kernel::Error> {
    let mut avatar = Vec::new();
    if let Ok(Some(mut field)) = payload.try_next().await {
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|_| kernel::Error::Internal(String::from("Uploading avatar.")))?;
            avatar.write_all(&data)?;
            if avatar.len() > consts::AVATAR_MAX_SIZE {
                return Err(kernel::Error::InvalidArgument(String::from("Upload is too large.")));
            }
        }

        let service_input = service::UpdateMyAvatarInput {
            avatar,
        };
        let me = ctx.kernel_service.update_my_avatar(actor, service_input).await?;

        Ok(api::Response::ok(model::convert_user(&ctx.kernel_service, me, true)))
    } else {
        Err(kernel::Error::InvalidArgument(String::from("Upload is empty")))
    }
}
