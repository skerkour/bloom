use stdx::{crypto, otp::totp};

use super::{DeleteMyAccountInput, Service};
use crate::{consts::BillingPlan, errors::kernel::Error, Actor};

impl Service {
    pub async fn delete_my_account(&self, actor: Actor, input: DeleteMyAccountInput) -> Result<(), crate::Error> {
        let actor = self.current_user(actor)?;

        let namespace = self.repo.find_namespace_by_id(&self.db, actor.namespace_id).await?;

        if actor.two_fa_enabled {
            let mut two_fa_code = input.two_fa_totp_code.ok_or(Error::TwoFACodeIsNotValid)?;

            // clean and validate data
            two_fa_code = two_fa_code.trim().to_lowercase().replace("-", "");
            let totp_secret = crypto::aead_decrypt(
                &self.config.master_key,
                &actor
                    .encrypted_totp_secret
                    .expect("kernel/delete_my_account: accessing actor.encrypted_totp_secret"),
                &actor
                    .totp_secret_nonce
                    .expect("kernel/delete_my_account: accessing actor.totp_secret_nonce"),
                &actor.id.as_bytes()[..],
            );
            //     if err != nil {
            //         errMessage := "kernel.DeleteMyAccount: decrypting TOTP secret"
            //         logger.Error(errMessage, log.Err("error", err))
            //         err = errors.Internal(errMessage, err)
            //         return
            //     }
            let totp_secret = String::from_utf8(totp_secret)?;
            if !totp::validate(&two_fa_code, &totp_secret) {
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
            .expect("kernel.create_namespace: unwrapping files_service")
            .clean_namespace(&mut tx, actor.namespace_id)
            .await?;

        self.repo.delete_namespace(&mut tx, actor.namespace_id).await?;

        tx.commit().await?;

        Ok(())
    }
}
