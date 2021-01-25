use super::{GetSignedUploadUrlInput, SignedUploadUrl};
use crate::{entities::Upload, Actor, Error, Service};
use stdx::{chrono::Utc, uuid::Uuid};

impl Service {
    /// Create an entity that can be retrieved later with the size, the user and the tmp_key
    pub async fn get_signed_upload_url(
        &self,
        actor: Actor,
        input: GetSignedUploadUrlInput,
    ) -> Result<SignedUploadUrl, Error> {
        let actor = self.current_user(actor)?;

        self.validate_upload_size(input.filesize)?;

        // check namespace membership
        self.check_namespace_membership(&self.db, &actor, input.namespace_id)
            .await?;

        let now = Utc::now();
        let upload = Upload {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            size: input.filesize as i64,
            completed: false,
            namespace_id: input.namespace_id,
        };
        self.repo.create_upload(&self.db, &upload).await?;

        let storage_key = upload.tmp_storage_key();
        let url = self
            .storage
            .get_presigned_upload_url(&storage_key, input.filesize)
            .await;

        Ok(SignedUploadUrl {
            url,
            upload_id: upload.id,
        })
    }
}
