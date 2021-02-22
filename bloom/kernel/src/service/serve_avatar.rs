use super::Service;

impl Service {
    pub async fn serve_avatar(&self, avatar_id: String) -> Result<Vec<u8>, crate::Error> {
        let avatar_storage_key = self.get_avatar_storage_key(&avatar_id);
        let avatar = self.storage.get_object(&avatar_storage_key).await?;
        Ok(avatar)
    }
}
