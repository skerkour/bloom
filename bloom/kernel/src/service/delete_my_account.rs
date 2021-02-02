use stdx::{crypto, log::error, otp::totp, sync::threadpool::spawn_blocking};

use super::{DeleteMyAccountInput, Service};
use crate::{consts::BillingPlan, errors::kernel::Error, Actor};

impl Service {
    pub async fn delete_my_account(&self, actor: Actor, input: DeleteMyAccountInput) -> Result<(), crate::Error> {
        let actor = self.current_user(actor)?;

        let namespace = self.repo.find_namespace_by_id(&self.db, actor.namespace_id).await?;

        if actor.two_fa_enabled {
            let mut two_fa_code = input.two_fa_code.ok_or(Error::TwoFACodeIsNotValid)?;

            // clean and validate data
            two_fa_code = two_fa_code.trim().to_lowercase().replace("-", "");
            let master_key = self.config.master_key.clone();
            let encrypted_totp_secret = actor
                .encrypted_totp_secret
                .clone()
                .expect("kernel.delete_my_account: accessing actor.encrypted_totp_secret");
            let totp_secret_nonce = actor
                .totp_secret_nonce
                .clone()
                .expect("kernel.delete_my_account: accessing actor.totp_secret_nonce");
            let ad: Vec<u8> = actor.id.as_bytes()[..].into();
            let totp_secret = match spawn_blocking(move || {
                crypto::aead_decrypt(&master_key, &encrypted_totp_secret, &totp_secret_nonce, &ad)
            })
            .await?
            {
                Ok(res) => res,
                Err(err) => {
                    error!("kernel.delete_my_account: decrypting totp secret: {}", err);
                    return Err(err.into());
                }
            };

            let totp_secret = String::from_utf8(totp_secret)?;
            if !(totp::validate(two_fa_code, totp_secret).await?) {
                return Err(Error::TwoFACodeIsNotValid.into());
            }
        }

        if actor.is_admin {
            return Err(Error::AdminUserCantBeDeleted.into());
        }

        let groups = self.repo.find_groups_for_user(&self.db, actor.id).await?;

        if !groups.is_empty() {
            return Err(Error::LeaveAllGroupsBeforeDeletingAccount.into());
        }

        if namespace.plan != BillingPlan::Free {
            return Err(Error::SubscriptionIsActive.into());
        }

        let mut tx = self.db.begin().await?;

        self.files_service
            .as_ref()
            .expect("kernel.delete_my_account: unwrapping files_service")
            .clean_namespace(&mut tx, actor.namespace_id)
            .await?;

        self.inbox_service
            .as_ref()
            .expect("kernel.delete_my_account: unwrapping inbox_service")
            .clean_user(&mut tx, actor.id)
            .await?;

        self.repo
            .detach_uploads_for_namespace(&mut tx, actor.namespace_id)
            .await?;

        self.repo.delete_namespace(&mut tx, actor.namespace_id).await?;

        tx.commit().await?;

        Ok(())
    }
}
