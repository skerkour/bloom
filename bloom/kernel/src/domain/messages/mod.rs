use serde::{Deserialize, Serialize};
use stdx::uuid::Uuid;

/// A Message represents any message that can be sent asynchronously between servers and workers.
/// It's used for jobs and tasks scheduling, async workflows, and analytics ingestion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    // kernel
    KenrnelSendRegisterEmail {
        email: String,
        username: String,
        code: String,
    },
    KernelSendSignInEmail {
        email: String,
        name: String,
        code: String,
    },
    KernelSendEmailChangedEmail {
        email: String,
        name: String,
        new_email: String,
    },
    KernelSendVerifyEmailEmail {
        email: String,
        name: String,
        code: String,
    },
    KernelSendGroupInvitationEmail {
        invitation_id: Uuid,
    },
}
