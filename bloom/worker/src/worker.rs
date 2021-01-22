use kernel::{
    domain::messages::Message,
    service::{
        SendEmailChangedEmailInput, SendGroupInvitationEmailInput, SendRegisterEmailInput, SendSignInEmailInput,
        SendVerifyEmailEmailInput,
    },
};
use kernel::{drivers::queue::Job, Error};
use std::sync::Arc;

pub struct Worker {
    kernel_service: Arc<kernel::Service>,
    analytics_service: Arc<analytics::Service>,
    inbox_service: Arc<inbox::Service>,
}

impl Worker {
    pub fn new(
        kernel_service: Arc<kernel::Service>,
        analytics_service: Arc<analytics::Service>,
        inbox_service: Arc<inbox::Service>,
    ) -> Self {
        Worker {
            kernel_service,
            analytics_service,
            inbox_service,
        }
    }

    pub async fn handle_job(&self, job: Job) -> Result<(), Error> {
        match job.message {
            Message::KernelSendRegisterEmail {
                email,
                username,
                code,
            } => {
                let input = SendRegisterEmailInput {
                    email,
                    username,
                    code,
                };
                self.kernel_service.send_register_email(input).await
            }
            Message::KernelSendSignInEmail {
                email,
                name,
                code,
            } => {
                let input = SendSignInEmailInput {
                    email,
                    name,
                    code,
                };
                self.kernel_service.send_sign_in_email(input).await
            }
            Message::KernelSendEmailChangedEmail {
                email,
                name,
                new_email,
            } => {
                let input = SendEmailChangedEmailInput {
                    email,
                    name,
                    new_email,
                };
                self.kernel_service.send_email_changed_email(input).await
            }
            Message::KernelSendVerifyEmailEmail {
                email,
                name,
                code,
            } => {
                let input = SendVerifyEmailEmailInput {
                    email,
                    name,
                    code,
                };
                self.kernel_service.send_verify_email_email(input).await
            }
            Message::KernelSendGroupInvitationEmail {
                invitation_id,
            } => {
                let input = SendGroupInvitationEmailInput {
                    invitation_id,
                };
                self.kernel_service.send_group_invitation_email(input).await
            }
            Message::AnalyticsPageEvent(event) => self.analytics_service.process_page_event(event).await,
            Message::AnalyticsTrackEvent(event) => self.analytics_service.process_track_event(event).await,
            Message::InboxSendNewsletterMessage {
                message_id,
                to,
                from,
                subscription_id,
            } => {
                let input = inbox::service::SendNewsletterMessageJobInput {
                    message_id,
                    to,
                    subscription_id,
                    from,
                };
                self.inbox_service.job_send_newsletter_message(input).await
            }
            Message::InboxDispatchSendNewsletterMessage {
                message_id,
            } => {
                self.inbox_service
                    .job_dispatch_send_newsletter_message(message_id)
                    .await
            }
            Message::KernelDeleteOldData => self.kernel_service.delete_old_data().await,
        }
    }
}
