use super::{CompleteFileUploadInput, Service};
use crate::{consts, entities::File, Error};
use kernel::{
    consts::{BillingPlan, STORAGE_FREE, STORAGE_PRO, STORAGE_STARTER},
    Actor,
};
use stdx::{chrono::Utc, ulid::Ulid};

impl Service {
    pub async fn complete_file_upload(
        &self,
        actor: Actor,
        mut input: CompleteFileUploadInput,
    ) -> Result<File, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let parent = self.repo.find_file_by_id(&self.db, input.parent_id).await?;

        if parent.namespace_id.is_none() {
            return Err(Error::FileNotFound.into());
        }

        let (mut namespace, _) = self
            .kernel_service
            .find_namespace_and_membership(&self.db, actor.id, parent.namespace_id.unwrap())
            .await?;

        // clean and valdiate input
        if parent.trashed_at.is_some() {
            return Err(Error::PermissionDenied.into());
        }

        self.validate_file_name(&input.name)?;

        if input.mime_type.is_empty() {
            input.mime_type = consts::FILE_TYPE_DEFAULT.to_string();
        }

        self.validate_file_type(&input.mime_type)?;

        match self
            .repo
            .find_file_by_parent_and_name(&self.db, parent.id, &input.name)
            .await
        {
            Ok(_) => return Err(Error::FileAlreadyExists.into()),
            Err(Error::FileNotFound) => {}
            Err(err) => return Err(err.into()),
        };

        let mut upload = self.kernel_service.find_upload(&self.db, input.upload_id).await?;

        if upload.namespace_id != parent.namespace_id.unwrap() {
            return Err(Error::PermissionDenied.into());
        }

        if upload.completed {
            return Err(Error::PermissionDenied.into());
        }

        let upload_storage_key = upload.tmp_storage_key();
        let size = self.storage.get_object_size(&upload_storage_key).await?;

        if size != upload.size {
            return Err(Error::PermissionDenied.into());
        }

        let now = Utc::now();
        namespace.updated_at = now;
        namespace.used_storage += size;

        upload.updated_at = now;
        upload.completed = true;

        if !self.kernel_service.self_hosted() {
            if (namespace.plan == BillingPlan::Free && namespace.used_storage > STORAGE_FREE)
                || (namespace.plan == BillingPlan::Starter && namespace.used_storage > STORAGE_STARTER)
                || (namespace.plan == BillingPlan::Pro && namespace.used_storage > STORAGE_PRO)
                // || (namespace.plan == BillingPlan::Ultra && namespace.used_storage > STORAGE_ULTRA)
                || namespace.used_storage < 0
                || size < 0
            {
                return Err(Error::StorageLimitReached.into());
            }
        }

        let file = File {
            id: Ulid::new().into(),
            created_at: now,
            updated_at: now,

            name: input.name,
            size,
            r#type: input.mime_type,
            explicitly_trashed: false,
            trashed_at: None,

            namespace_id: Some(namespace.id),
            parent_id: Some(parent.id),
        };

        self.storage
            .copy_object(&upload_storage_key, &file.storage_key())
            .await?;

        let mut tx = self.db.begin().await?;

        self.repo.create_file(&mut tx, &file).await?;

        self.kernel_service.update_namespace(&mut tx, &namespace).await?;

        self.kernel_service.update_upload(&mut tx, &upload).await?;

        tx.commit().await?;

        Ok(file)
    }
}
