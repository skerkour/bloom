use std::sync::Arc;

use kernel::{drivers::queue::Job, Error, Service};

pub struct Worker {
    kernel_service: Arc<Service>,
}

impl Worker {
    pub fn new(kernel_service: Arc<Service>) -> Self {
        Worker { kernel_service }
    }

    pub async fn handle_job(&self, job: Job) -> Result<(), Error> {
        match job.message {
            kernel::domain::messages::Message::KenrnelSendRegisterEmail { email, username, code } => {
                self.kernel_service.send_register_email().await
            }
            kernel::domain::messages::Message::KernelSendSignInEmail { email, name, code } => {
                self.kernel_service.send_sign_in_email().await
            }
            kernel::domain::messages::Message::KernelSendEmailChangedEmail { email, name, new_email } => {
                self.kernel_service.send_email_changed_email().await
            }
            kernel::domain::messages::Message::KernelSendVerifyEmailEmail { email, name, code } => {
                self.kernel_service.send_verify_email_email().await
            }
            kernel::domain::messages::Message::KernelSendGroupInvitationEmail { invitation_id } => {
                self.kernel_service.send_group_invitation_email().await
            }
        }
    }
}
