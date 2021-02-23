use super::Service;
use crate::{consts, domain::inbox::UpdateChatboxAvatarInput, entities::Group, errors::kernel::Error, service, Actor};
use stdx::{
    image::{self, imageops::FilterType, ImageOutputFormat},
    log::error,
    sync::threadpool::spawn_blocking,
    uuid::Uuid,
};

impl Service {
    pub async fn update_group_avatar(
        &self,
        actor: Actor,
        input: service::UpdateGroupAvatarInput,
    ) -> Result<Group, crate::Error> {
        let actor = self.current_user(actor)?;

        // check group membership
        let (mut group, _) = self
            .find_group_and_membership(&self.db, actor.id, input.group_id)
            .await?;

        // validate input
        if input.avatar.len() > consts::AVATAR_MAX_SIZE {
            return Err(Error::UploadIsTooLarge.into());
        }

        let avatar = match spawn_blocking(move || -> Result<Vec<u8>, crate::Error> {
            let avatar = image::load_from_memory(&input.avatar)?;
            let avatar = avatar.resize(
                consts::AVATAR_SIZE as u32,
                consts::AVATAR_SIZE as u32,
                FilterType::Lanczos3,
            );

            let mut avatar_buffer: Vec<u8> = Vec::new();
            avatar.write_to(&mut avatar_buffer, ImageOutputFormat::Jpeg(consts::AVATAR_JPEG_QUALITY))?;

            Ok(avatar_buffer)
        })
        .await?
        {
            Ok(avatar) => avatar,
            Err(err) => {
                error!("kernel.update_group_avatar: processing image: {}", err);
                return Err(err.into());
            }
        };

        // upload new avatar
        let avatar_id = Uuid::new_v4().to_hyphenated().to_string();
        let avatar_storage_key = self.get_avatar_storage_key(&avatar_id);
        self.storage
            .put_object(&avatar_storage_key, avatar, consts::AVATAR_CONTENT_TYPE)
            .await?;

        // delete old avatar
        if let Some(old_avatar_id) = group.avatar_id {
            let old_avatar_storage_key = self.get_avatar_storage_key(&old_avatar_id);
            match self.storage.delete_object(&old_avatar_storage_key).await {
                Ok(_) => {}
                Err(err) => {
                    error!("kernel.update_group_avatar: deleting old avatar: {}", err);
                }
            }
        }

        // update group and chatbox preferences
        group.avatar_id = Some(avatar_id);

        let inbox_service_input = UpdateChatboxAvatarInput {
            namespace_id: group.namespace_id,
            avatar_id: group.avatar_id.clone(),
        };

        let mut tx = self.db.begin().await?;

        self.repo.update_group(&mut tx, &group).await?;

        self.inbox_service
            .as_ref()
            .expect("kernel.update_group_avatar: unwrapping inbox_service")
            .update_chatbox_avatar_unauthenticated(&mut tx, inbox_service_input)
            .await?;

        tx.commit().await?;

        Ok(group)
    }
}
