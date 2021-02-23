use crate::{api::kernel::model, ServerContext};
use actix_multipart::Multipart;
use actix_web::web;
use kernel::{consts, http::api, service, Actor};
use std::{io::Write, sync::Arc};
use stdx::{
    futures::{StreamExt, TryStreamExt},
    uuid::Uuid,
};

pub async fn update_group_avatar(
    ctx: web::Data<Arc<ServerContext>>,
    mut payload: Multipart,
    actor: Actor,
) -> Result<api::Response<model::Group>, kernel::Error> {
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

        // big hack, we send the group_id as file.name...
        let content_disposition = field
            .content_disposition()
            .ok_or(kernel::Error::Internal(String::from("Uploading avatar.")))?;
        let group_id_str = content_disposition
            .get_filename()
            .ok_or(kernel::Error::Internal(String::from("Uploading avatar.")))?;
        let group_id = Uuid::parse_str(group_id_str)?;

        let service_input = service::UpdateGroupAvatarInput {
            avatar,
            group_id,
        };
        let group = ctx.kernel_service.update_group_avatar(actor, service_input).await?;

        Ok(api::Response::ok(model::convert_group(
            &ctx.kernel_service,
            group,
            true,
        )))
    } else {
        Err(kernel::Error::InvalidArgument(String::from("Upload is empty")))
    }
}
