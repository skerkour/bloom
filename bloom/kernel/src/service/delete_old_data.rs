use super::Service;
use stdx::{
    chrono::{Duration, Utc},
    log::error,
};

impl Service {
    pub async fn delete_old_data(&self) -> Result<(), crate::Error> {
        let three_days_ago = Utc::now() - Duration::days(3);

        let old_uploads = self.repo.find_old_uploads(&self.db, three_days_ago).await?;

        for upload in old_uploads {
            let storage_key = upload.tmp_storage_key();

            match self.storage.delete_object(&storage_key).await {
                Ok(_) => {}
                Err(err) => {
                    error!("kernel.delete_old_data: deleting upload from storage: {}", err);
                    continue;
                }
            }

            match self.repo.delete_upload(&self.db, upload.id).await {
                Ok(_) => {}
                Err(err) => error!("kernel.delete_old_data: deleting upload from db: {:?}", err),
            }
        }

        match self.repo.delete_old_pending_users(&self.db, three_days_ago).await {
            Ok(_) => {}
            Err(err) => error!("kernel.delete_old_data: deleting pending users: {:?}", err),
        }

        match self.repo.delete_old_pending_sessions(&self.db, three_days_ago).await {
            Ok(_) => {}
            Err(err) => error!("kernel.delete_old_data: deleting pending sessions: {:?}", err),
        }

        Ok(())
    }
}
