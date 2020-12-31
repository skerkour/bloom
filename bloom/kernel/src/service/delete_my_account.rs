use stdx::{crypto, otp::totp};

use super::{DeleteMyAccountInput, Service};
use crate::{consts::BillingPlan, entities::User, errors::kernel::Error};

impl Service {
    pub async fn delete_my_account(
        &self,
        actor: Option<User>,
        input: DeleteMyAccountInput,
    ) -> Result<(), crate::Error> {
        let actor = self.current_user(actor)?;

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

        if actor.plan != BillingPlan::Free {
            return Err(Error::SubscriptionIsActive.into());
        }

        self.repo.delete_namespace(&self.db, actor.namespace_id).await?;

        Ok(())
    }
}
