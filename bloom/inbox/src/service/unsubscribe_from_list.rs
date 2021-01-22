use super::UnsubscribeFromListInput;
use crate::Service;
use kernel::{consts, Actor};
use stdx::{
    rand::{thread_rng, Rng},
    tokio::time::delay_for,
};

impl Service {
    pub async fn unsubscribe_from_list(&self, _: Actor, input: UnsubscribeFromListInput) -> Result<(), kernel::Error> {
        // authentication not required
        // sleep to prevent spam and bruteforce
        let sleep = thread_rng().gen_range(consts::SLEEP_MIN..consts::SLEEP_MAX);
        delay_for(sleep).await;

        self.repo
            .delete_newsletter_list_subscription(&self.db, input.subscription_id)
            .await?;

        Ok(())
    }
}
