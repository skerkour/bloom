use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationEmailParams {
    pub code: String,
}

pub const REGISTRATION_EMAIL_TEMPLATE_ID: &str = "REGISTRATION_EMAIL_TEMPLATE";
pub const REGISTRATION_EMAIL_TEMPLATE: &str = "
 <h1> {{ code }} </h1>
";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingInEmailParams {
    pub code: String,
}

pub const SIGN_IN_EMAIL_TEMPLATE_ID: &str = "SIGN_IN_EMAIL_TEMPLATE";
pub const SIGN_IN_EMAIL_TEMPLATE: &str = "
 <h1> {{ code }} </h1>
";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentFailedEmailParams {
    pub name: String,
    pub billing_portal_url: String,
    pub amount: f64,
}

pub const PAYMENT_FAILED_EMAIL_TEMPLATE_ID: &str = "PAYMENT_FAILED_EMAIL_TEMPLATE";
pub const PAYMENT_FAILED_EMAIL_TEMPLATE: &str = "
Hi {{ name }}, <br />
 Unfortunately, your most recent invoice payment for {{ amount }} was declined.
 This could be due to a change in your card number, your card expiring,
 cancellation of your credit card, or the card issuer not recognizing the
 payment and therefore taking action to prevent it. <br />

 Please go to you Billing Portal: <a href=\"{{ billing_portal_url }}\">{{ billing_portal_url }}</a> to update your payment information
 as soon as possible.

 Best regards, <br />
 The Bloom team
";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentActionRequiredEmailParams {
    pub name: String,
    pub billing_portal_url: String,
}

pub const PAYMENT_ACTION_REQUIRED_EMAIL_TEMPLATE_ID: &str = "PAYMENT_ACTION_REQUIRED_EMAIL_TEMPLATE";
pub const PAYMENT_ACTION_REQUIRED_EMAIL_TEMPLATE: &str = "
Hi {{ name }}, <br />
 there was a problem processing your last payment <br />

Please go to you Billing Portal: <a href=\"{{ billing_portal_url }}\">{{ billing_portal_url }}</a> to fix the issue
 as soon as possible.


Best regards, <br />
 The Bloom team
";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyEmailEmailParams {
    pub code: String,
}

pub const VERIFY_EMAIL_EMAIL_TEMPLATE_ID: &str = "VERIFY_EMAIL_EMAIL_TEMPLATE";
pub const VERIFY_EMAIL_EMAIL_TEMPLATE: &str = "
<h1> {{ code }} </h1>
";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailChangedEmailData {
    pub new_email: String,
    pub contact_url: String,
}

pub const EMAIL_CHANGED_EMAIL_TEMPLATE_ID: &str = "EMAIL_CHANGED_EMAIL_TEMPLATE_ID";
pub const EMAIL_CHANGED_EMAIL_TEMPLATE: &str = "
Hi, <br />
You just updated your email address for: <br />

<h4>{{ new_email }}</h4> <br />

If you do not recognize this email address please <a href=\"{{ contact_url }}\">contact Bloom Support</a> as soon as possible. <br />
Otherwise, no action is required. <br />
<br />

Best regards, <br />
The Bloom team
";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInvitationEmailData {
    pub inviter_name: String,
    pub inviter_url: String,
    pub invitee_name: String,
    pub group_name: String,
    pub invitations_url: String,
}

pub const GROUP_INVITATION_EMAIL_TEMPLATE_ID: &str = "GROUP_INVITATION_EMAIL_TEMPLATE";
pub const GROUP_INVITATION_EMAIL_TEMPLATE: &str = "
Hi {{ inviter_name }}, <br />
<a href=\"{{ inviter_url }}\">{{ inviter_name }}</a> just invited you to join the group: <b>{{ group_name }}</b> <br />

Click on the following link to accept or decline the invitation: <br />

<a href=\"{{ invitations_url }}\">{{ invitations_url }}</a>

<br /> <br />

Best regards, <br />
The Bloom team
";
