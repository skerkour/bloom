use kernel::domain::messages::Message;
use kernel::{drivers::queue::Job, Error};
use std::sync::Arc;

pub struct Worker {
    kernel_service: Arc<kernel::Service>,
    analytics_service: Arc<analytics::Service>,
}

impl Worker {
    pub fn new(kernel_service: Arc<kernel::Service>, analytics_service: Arc<analytics::Service>) -> Self {
        Worker {
            kernel_service,
            analytics_service,
        }
    }

    pub async fn handle_job(&self, job: Job) -> Result<(), Error> {
        match job.message {
            Message::KenrnelSendRegisterEmail { email, username, code } => {
                self.kernel_service.send_register_email().await
            }
            Message::KernelSendSignInEmail { email, name, code } => self.kernel_service.send_sign_in_email().await,
            Message::KernelSendEmailChangedEmail { email, name, new_email } => {
                self.kernel_service.send_email_changed_email().await
            }
            Message::KernelSendVerifyEmailEmail { email, name, code } => {
                self.kernel_service.send_verify_email_email().await
            }
            Message::KernelSendGroupInvitationEmail { invitation_id } => {
                self.kernel_service.send_group_invitation_email().await
            }
            Message::AnalyticsPageEvent(event) => self.analytics_service.process_page_event(event).await,
            Message::AnalyticsTrackEvent(event) => self.analytics_service.process_track_event(event).await,
        }
    }
}
