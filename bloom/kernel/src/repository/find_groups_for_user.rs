use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::log::error;
use stdx::sqlx;
use stdx::uuid::Uuid;

impl Repository {
    pub async fn find_groups_for_user<'c, C: Queryer<'c>>(
        &self,
        db: C,
        user_id: Uuid,
    ) -> Result<Vec<entities::Group>, Error> {
        const QUERY: &str = "SELECT kernel_groups.* FROM kernel_groups
		INNER JOIN kernel_groups_members ON kernel_groups.id = kernel_groups_members.group_id
		WHERE kernel_groups_members.user_id = $1
		ORDER BY kernel_groups.id DESC";

        match sqlx::query_as::<_, entities::Group>(QUERY)
            .bind(user_id)
            .fetch_all(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_groups_for_user: finding groups: {}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }
}
