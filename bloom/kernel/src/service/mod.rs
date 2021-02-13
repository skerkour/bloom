use crate::{
    config::Config,
    consts::NamespaceType,
    consts::{self, BillingPlan, TwoFaMethod},
    db::DB,
    domain::{files, inbox},
    drivers,
    entities::{Group, GroupInvitation, GroupMember, Session, User},
    notifications::PAYMENT_ACTION_REQUIRED_EMAIL_TEMPLATE_ID,
    notifications::PAYMENT_FAILED_EMAIL_TEMPLATE,
    notifications::PAYMENT_FAILED_EMAIL_TEMPLATE_ID,
    notifications::REGISTRATION_EMAIL_TEMPLATE,
    notifications::REGISTRATION_EMAIL_TEMPLATE_ID,
    notifications::SIGN_IN_EMAIL_TEMPLATE,
    notifications::{
        EMAIL_CHANGED_EMAIL_TEMPLATE, EMAIL_CHANGED_EMAIL_TEMPLATE_ID, GROUP_INVITATION_EMAIL_TEMPLATE,
        GROUP_INVITATION_EMAIL_TEMPLATE_ID, PAYMENT_ACTION_REQUIRED_EMAIL_TEMPLATE, SIGN_IN_EMAIL_TEMPLATE_ID,
        VERIFY_EMAIL_EMAIL_TEMPLATE, VERIFY_EMAIL_EMAIL_TEMPLATE_ID,
    },
    repository::Repository,
};
use std::{collections::HashSet, fmt::Debug, sync::Arc};
use stdx::{stripe, uuid::Uuid};

mod accept_group_invitation;
mod admin_block_user;
mod admin_find_group;
mod admin_find_groups;
mod admin_find_user;
mod admin_find_users;
mod admin_unblock_user;
mod cancel_group_invitation;
mod check_namespace_exists;
mod check_namespace_membership;
mod complete_registration;
mod complete_sign_in;
mod complete_two_fa_challenge;
mod complete_two_fa_setup;
mod config;
mod create_group;
mod create_namespace;
mod decline_group_invitation;
mod decode_and_validate_session_token;
mod delete_group;
mod delete_my_account;
mod delete_old_data;
mod disable_two_fa;
mod dispatch_delete_old_data;
mod find_group;
mod find_group_and_membership;
mod find_group_members_and_invitations;
mod find_my_group_invitations;
mod find_my_sessions;
mod find_namespace;
mod find_namespace_and_membership;
mod find_upload;
mod get_billing_information;
mod get_signed_upload_url;
mod get_stripe_checkout_session;
mod get_stripe_customer_portal_url;
mod get_stripe_public_key;
mod invite_people_in_group;
mod markdown;
mod me;
mod new_session;
mod qr_code;
mod quit_group;
mod register;
mod remove_member_from_group;
mod revoke_session;
mod send_email_changed_email;
mod send_group_invitation_email;
mod send_register_email;
mod send_sign_in_email;
mod send_verify_email_email;
mod setup_two_fa;
mod sign_in;
mod sync_customer_with_stripe;
mod update_billing_information;
mod update_group_profile;
mod update_my_profile;
mod update_namespace;
mod update_upload;
mod utils;
mod validators;
mod verify_email;

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: DB,
    config: Arc<Config>,
    queue: Arc<dyn drivers::Queue>,
    mailer: Arc<dyn drivers::Mailer>,
    storage: Arc<dyn drivers::Storage>,
    templates: tera::Tera,
    invalid_namespaces: HashSet<String>,
    valid_namespace_alphabet: HashSet<char>,
    files_service: Option<Arc<dyn files::Service>>,
    inbox_service: Option<Arc<dyn inbox::Service>>,
    xss: Arc<dyn drivers::XssSanitizer>,
    stripe_client: Option<stripe::Client>,
}

impl Service {
    pub fn new(
        config: Config,
        db: DB,
        queue: Arc<dyn drivers::Queue>,
        mailer: Arc<dyn drivers::Mailer>,
        storage: Arc<dyn drivers::Storage>,
        xss: Arc<dyn drivers::XssSanitizer>,
    ) -> Service {
        let mut templates = tera::Tera::default();
        // don't escape input as it's provided by us
        templates.autoescape_on(Vec::new());
        templates
            .add_raw_template(REGISTRATION_EMAIL_TEMPLATE_ID, REGISTRATION_EMAIL_TEMPLATE)
            .expect("kernel: parsing REGISTRATION_EMAIL_TEMPLATE");
        templates
            .add_raw_template(SIGN_IN_EMAIL_TEMPLATE_ID, SIGN_IN_EMAIL_TEMPLATE)
            .expect("kernel: parsing SIGN_IN_EMAIL_TEMPLATE");
        templates
            .add_raw_template(PAYMENT_FAILED_EMAIL_TEMPLATE_ID, PAYMENT_FAILED_EMAIL_TEMPLATE)
            .expect("kernel: parsing PAYMENT_FAILED_EMAIL_TEMPLATE");
        templates
            .add_raw_template(
                PAYMENT_ACTION_REQUIRED_EMAIL_TEMPLATE_ID,
                PAYMENT_ACTION_REQUIRED_EMAIL_TEMPLATE,
            )
            .expect("kernel: parsing PAYMENT_ACTION_REQUIRED_EMAIL_TEMPLATE");
        templates
            .add_raw_template(VERIFY_EMAIL_EMAIL_TEMPLATE_ID, VERIFY_EMAIL_EMAIL_TEMPLATE)
            .expect("kernel: parsing VERIFY_EMAIL_EMAIL_TEMPLATE");
        templates
            .add_raw_template(EMAIL_CHANGED_EMAIL_TEMPLATE_ID, EMAIL_CHANGED_EMAIL_TEMPLATE)
            .expect("kernel: parsing EMAIL_CHANGED_EMAIL_TEMPLATE");
        templates
            .add_raw_template(GROUP_INVITATION_EMAIL_TEMPLATE_ID, GROUP_INVITATION_EMAIL_TEMPLATE)
            .expect("kernel: parsing GROUP_INVITATION_EMAIL_TEMPLATE");

        let repo = Repository::new();

        let invalid_namespaces = consts::INVALID_NAMESPACES
            .iter()
            .map(|namespace| namespace.to_string())
            .collect();

        let valid_namespace_alphabet = consts::NAMESPACE_ALPHABET.chars().collect();

        let stripe_client = if config.self_hosted {
            None
        } else {
            Some(stripe::Client::new(config.stripe.as_ref().unwrap().secret_key.clone()))
        };

        let config = Arc::new(config);

        Service {
            db,
            repo,
            config,
            queue,
            mailer,
            storage,
            templates,
            invalid_namespaces,
            valid_namespace_alphabet,
            files_service: None,
            inbox_service: None,
            xss,
            stripe_client,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SignedIn {
    Success { me: Me, token: String },
    TwoFa(TwoFaMethod),
}

#[derive(Debug, Clone)]
pub struct Me {
    pub session: Session,
    pub user: User,
    pub groups: Vec<Group>,
}

#[derive(Debug, Clone)]
pub struct NewSession {
    pub session: Session,
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct Registered {
    pub session: Session,
    pub user: User,
    pub token: String,
}

/// RegisterInput are the data required to start to register to bloom
#[derive(Debug, Clone)]
pub struct RegisterInput {
    pub email: String,
    pub username: String,
}

/// CompleteRegistrationInput are the data required to complete a bloom registration
#[derive(Debug, Clone)]
pub struct CompleteRegistrationInput {
    pub pending_user_id: Uuid,
    pub code: String,
}

/// CompleteSignInInput are the data required to complete a sign in
#[derive(Debug, Clone)]
pub struct CompleteSignInInput {
    pub pending_session_id: Uuid,
    pub code: String,
}

/// SignInInput are the data required to start a sign in
#[derive(Debug, Clone)]
pub struct SignInInput {
    pub email_or_username: String,
}

#[derive(Debug, Clone)]
pub struct CreateGroupInput {
    pub name: String,
    pub path: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct DeleteGroupInput {
    pub group_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct CreateNamespaceInput {
    pub path: String,
    pub namespace_type: NamespaceType,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct UpdatePaymentMethodInput {
    pub stripe_id: String,
    pub namespace_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct ChangeSubscriptionInput {
    pub namespace_id: Uuid,
    pub plan: BillingPlan,
}

#[derive(Debug, Clone)]
pub struct GetStripeCheckoutSessionInput {
    pub namespace_id: Uuid,
    pub plan: BillingPlan,
}

#[derive(Debug, Clone)]
pub struct UpdateBillingInformationInput {
    pub namespace_id: Uuid,
    pub name: String,
    pub email: String,
    pub country_code: String,
    pub city: String,
    pub postal_code: String,
    pub address_line1: String,
    pub address_line2: String,
    pub state: String,
    pub tax_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SyncBillingWithProviderInput {
    pub namespace: String,
}

#[derive(Debug, Clone)]
pub struct UpdateMyProfileInput {
    pub name: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub description: Option<String>,
    // pub avatar: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct VerifyPendingEmailInput {
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct UpdateGroupProfileInput {
    pub group_id: Uuid,
    pub name: Option<String>,
    pub path: Option<String>,
    pub description: Option<String>,
    // pub avatar: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct InvitePeopleInGroupInput {
    pub group_id: Uuid,
    pub usernames: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AcceptGroupInvitationInput {
    pub invitation_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct CancelGroupInvitationInput {
    pub invitation_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct DeclineGroupInvitationInput {
    pub invitation_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct RemoveMemberFromGroupInput {
    pub group_id: Uuid,
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct QuitGroupInput {
    pub group_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct GetSignedUploadUrlInput {
    pub namespace_id: Uuid,
    pub filesize: u64,
}

#[derive(Debug, Clone)]
pub struct CompleteTwoFaSetup {
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct DisableTwoFaInput {
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct CompleteTwoFaChallengeInput {
    pub pending_session_id: Uuid,
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct DeleteMyAccountInput {
    pub two_fa_code: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RevokeSessionInput {
    pub session_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct VerifyEmailInput {
    pub pending_email_id: Uuid,
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct DecodedSessionToken {
    pub session_id: Uuid,
    pub secret: Vec<u8>,
}

// type GroupMember struct {
// 	User
// 	Role GroupRole `db:"role"`
// }

#[derive(Debug, Clone)]
pub struct SignedUploadUrl {
    pub url: String,
    pub upload_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct SendGroupInvitationEmailInput {
    pub invitation_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct SendRegisterEmailInput {
    pub email: String,
    pub username: String,
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct SendSignInEmailInput {
    pub email: String,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct SendEmailChangedEmailInput {
    pub email: String,
    pub name: String,
    pub new_email: String,
}

#[derive(Debug, Clone)]
pub struct SendVerifyEmailEmailInput {
    pub email: String,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct GroupInvitationWithDetails {
    pub invitation: GroupInvitation,
    pub group: Group,
    pub inviter: User,
    pub invitee: User,
}

#[derive(Debug, Clone)]
pub struct GroupWithMembersAndInvitations {
    pub group: Group,
    pub invitations: Vec<GroupInvitationWithDetails>,
    pub members: Vec<GroupMember>,
}

// type NamespaceAndCustomer struct {
// 	Customer
// 	Namespace
// }

#[cfg(test)]
pub mod test {
    use crate::{
        config::Config,
        drivers::{
            mailer::test::MailerMock, queue::test::QueueMock, storage::test::StorageMock, xss::stdx::StdxXssSanitizer,
        },
        Service,
    };
    use std::sync::Arc;

    pub async fn new_service_mock(config: Config) -> Arc<Service> {
        let db = crate::db::connect(&config.database).await.expect("connecting to db");
        let queue = Arc::new(QueueMock::new());
        let mailer = Arc::new(MailerMock::new());
        let storage = Arc::new(StorageMock::new());
        let stdx_xss_sanitizer = Arc::new(StdxXssSanitizer::new());

        Arc::new(Service::new(config, db, queue, mailer, storage, stdx_xss_sanitizer))
    }
}
