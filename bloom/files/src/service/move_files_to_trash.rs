use std::collections::{HashMap, HashSet};

use crate::{entities::File, Error};

use super::{MoveFilesToTrashInput, Service};
use kernel::Actor;
use stdx::{chrono::Utc, uuid::Uuid};

impl Service {
    pub async fn move_files_to_trash(&self, actor: Actor, input: MoveFilesToTrashInput) -> Result<(), kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        if input.files.is_empty() {
            return Ok(());
        }

        let files = self.repo.find_files_by_ids(&self.db, &input.files).await?;

        if files.len() != input.files.len() {
            return Err(Error::FileNotFound.into());
        }

        let namespace_id = files[0].namespace_id;
        if namespace_id.is_none() {
            return Err(Error::FileNotFound.into());
        }
        let namespace_id = namespace_id.unwrap();

        for file in &files {
            if file.namespace_id.is_none() || file.namespace_id.unwrap() != namespace_id {
                return Err(Error::PermissionDenied.into());
            }

            if file.is_root() {
                return Err(Error::PermissionDenied.into());
            }

            if file.trashed_at.is_some() {
                return Err(Error::PermissionDenied.into());
            }
        }

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, namespace_id)
            .await?;

        let mut all_children = HashMap::<Uuid, Vec<File>>::with_capacity(files.len());
        let mut unique_children = HashSet::<Uuid>::new();
        let mut children_count: usize = 0;

        for file in &files {
            let children = self.repo.find_children_recursively(&self.db, file.id).await?;
            for child in &children {
                unique_children.insert(child.id);
                children_count += 1;
            }
            all_children.insert(file.id, children);
        }

        if unique_children.len() != children_count {
            return Err(Error::PermissionDenied.into());
        }

        for file in &files {
            if unique_children.contains(&file.id) {
                return Err(Error::PermissionDenied.into());
            }
        }

        let now = Utc::now();
        let mut tx = self.db.begin().await?;

        for mut file in files {
            file.updated_at = now;
            file.trashed_at = Some(now);
            file.explicitly_trashed = true;
            self.repo.update_file(&mut tx, &file).await?;
        }

        for (_, children) in all_children {
            for mut child in children {
                child.updated_at = now;
                child.trashed_at = Some(now);
                child.explicitly_trashed = false;
                self.repo.update_file(&mut tx, &child).await?;
            }
        }

        tx.commit().await?;

        Ok(())
    }
}
