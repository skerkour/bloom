mod errors;
mod repository;

pub mod config;
pub mod consts;
pub mod db;
pub mod domain;
pub mod drivers;
pub mod entities;
pub mod http;
pub mod notifications;
pub mod service;
pub use errors::Error;
pub use service::Service;

// #[async_trait::async_trait]
// pub trait Service: Send + Sync + Debug {
//     // admin
//     // async fn admin_find_users(&self) -> Result<Vec<User>, Error>;
//     // async fn admin_find_user(&self, username: String) -> Result<User, Error>;
//     // async fn admin_disable_user(&self, username: String) -> Result<(), Error>;
//     // async fn admin_enable_user(&self, username: String) -> Result<(), Error>;
//     // async fn admin_find_groups(&self) -> Result<Vec<Group>, Error>; // []GroupNamespace
//     // async fn admin_find_group(&self, group_id: uuid::Uuid) -> Result<Group, Error>; // GroupNamespace

//     // // Namespaces
//     // // CurrentUserNamespace(ctx context.Context) (User, Namespace, error)
//     // // FindNamespace(ctx context.Context, fullPath string) (ret GroupUserNamespace, err error)
//     // async fn namespace_exists(&self, db: &DB, namespace: &str) -> Result<bool, Error>;
//     // async fn find_namespace_by_id(&self, db: &DB, namespace_id: uuid::Uuid) -> Result<Namespace, Error>;
//     // async fn update_namespace(&self, db: &DB, namespace: &Namespace) -> Result<(), Error>;
//     // async fn create_namespace(&self, db: &DB, input: CreateNamespaceInput) -> Result<Namespace, Error>;
//     // // CreateOrUpdateInvoice(ctx context.Context, stripeInvoice stripe.Invoice) (err error)

//     // Users
//     async fn register(&self, actor: Option<&User>, input: service::RegisterInput) -> Result<PendingUser, Error>;
//     // async fn complete_registration(&self, input: service::CompleteRegistrationInput) -> Result<SignedIn, Error>;
//     // async fn sign_in(&self, email_or_username: String) -> Result<PendingSession, Error>;
//     // async fn complete_sign_in(&self, input: CompleteSignInInput) -> Result<SignedIn, Error>;
//     // async fn revoke_session(&self, session_id: uuid::Uuid) -> Result<(), Error>;
//     // async fn update_my_profile(&self, input: UpdateMyProfileInput) -> Result<User, Error>;
//     // async fn verify_email(&self, input: VerifyPendingEmailInput) -> Result<(), Error>;
//     // async fn verify_session_token(&self, token: String) -> Result<(User, Session), Error>;
//     // // CurrentUser(ctx context.Context) (User, error)
//     // async fn find_sessions_for_user(&self, user_id: uuid::Uuid) -> Result<Vec<Session>, Error>;
//     // async fn find_my_group_invitations(&self) -> Result<Vec<GroupInvitation>, Error>;
//     // async fn find_users_for_namespace(&self, namespace_id: uuid::Uuid) -> Result<Vec<User>, Error>;
//     // async fn delete_my_account(&self, input: DeleteMyAccountInput) -> Result<(), Error>;
//     // async fn setup_two_fa(&self) -> Result<String, Error>;
//     // async fn enable_two_fa(&self, input: EnableTwoFaInput) -> Result<(), Error>;
//     // async fn disable_two_fa(&self, input: DisableTwoFaInput) -> Result<(), Error>;
//     // async fn complete_two_fa_challenge(&self, input: CompleteTwoFaChallengeInput) -> Result<SignedIn, Error>;

//     // // Groups
//     // async fn create_group(&self, input: CreateGroupInput) -> Result<Group, Error>; // (ret GroupNamespace, err error)
//     // async fn update_group_profile(&self, input: UpdateGroupProfileInput) -> Result<Group, Error>; // (ret GroupNamespace, err error)
//     // async fn delete_group(&self, namespace: String) -> Result<(), Error>;
//     // async fn find_groups_for_user(&self, user_id: uuid::Uuid) -> Result<Vec<Group>, Error>; // []GroupNamespace
//     // async fn find_group_by_namespace(&self, namespace: String) -> Result<Group, Error>; // GroupNamespace
//     // async fn find_group_by_id(&self, group_id: uuid::Uuid) -> Result<Group, Error>; // GroupNamespace
//     // async fn invite_people_in_group(&self, input: InvitePeopleInGroupInput) -> Result<Group, Error>; // GroupNamespace
//     // async fn accept_group_invitation(&self, input: AcceptGroupInvitationInput) -> Result<Group, Error>; // GroupNamespace
//     // async fn decline_group_invitation(&self, input: DeclineGroupInvitationInput) -> Result<(), Error>;
//     // async fn cancel_group_invitation(&self, input: CancelGroupInvitationInput) -> Result<(), Error>;
//     // async fn quit_group(&self, input: QuitGroupInput) -> Result<(), Error>;
//     // async fn remove_member_from_group(&self, input: RemoveMemberFromGroupInput) -> Result<Group, Error>;
//     // async fn find_group_invitations(&self, group_id: uuid::Uuid) -> Result<Vec<GroupInvitation>, Error>;
//     // async fn find_group_members(&self, group_id: uuid::Uuid) -> Result<Vec<GroupMember>, Error>;
//     // async fn find_group_for_invitation(&self, invitation_id: uuid::Uuid) -> Result<Group, Error>; // GroupNamespace
//     // async fn find_inviter_for_invitation(&self, invitation_id: uuid::Uuid) -> Result<User, Error>;
//     // async fn find_invitee_for_invitation(&self, invitation_id: uuid::Uuid) -> Result<User, Error>;

//     // // Billing
//     // // PaymentFailed(ctx context.Context, stripeInvoice stripe.Invoice) (err error)
//     // async fn update_billing_information(&self, input: UpdateBillingInformationInput) -> Result<Customer, Error>; // NamespaceAndCustomer
//     // async fn sync_billing_with_provider(&self, input: SyncBillingWithProviderInput) -> Result<(), Error>;
//     // // HandleStripeEvent(ctx context.Context, event stripe.Event) (err error)
//     // // PaymentActionRequired(ctx context.Context, stripeInvoice stripe.Invoice) (err error)
//     // async fn get_subscribers_count_for_plan(&self, plan: BillingPlan) -> Result<u64, Error>;
//     // async fn find_invoices_for_namespace(&self, namespace_id: uuid::Uuid) -> Result<Vec<Invoice>, Error>;
//     // async fn find_customer_for_group(&self, group_id: uuid::Uuid) -> Result<Customer, Error>; // NamespaceAndCustomer
//     // async fn get_stripe_customer_portal_url(&self, namespace: String) -> Result<String, Error>;
//     // async fn get_stripe_checkout_session(&self, input: GetCheckoutSessionInput) -> Result<String, Error>;
//     // async fn get_stripe_public_key(&self) -> Result<String, Error>;

//     // // Other
//     // async fn serve_avatar(&self, avatar_id: String) -> Result<Vec<u8>, Error>; // (object io.ReadCloser, err error)
//     // fn default_group_avatar_url(&self) -> String;
//     // fn group_avatar_url(&self, avatar: Option<String>) -> String;
//     // fn default_user_avatar_url(&self) -> String;
//     // fn user_avatar_url(&self, avatar: Option<String>) -> String;
//     // async fn get_signed_storage_upload_url(&self, input: GetSignedStorageUploadUrlInput) -> Result<SignedStorageUploadUrl, Error>;
//     // fn is_self_hosted(&self) -> bool;
//     // fn base_url(&self) -> String;
//     // fn cdn_base_url(&self) -> String;
//     // fn avatar_storage_key(&self, avatar: String) -> String;
//     // fn total_storage_for_plan(&self, plan: BillingPlan) -> String;
//     // fn master_key(&self) -> Vec<u8>;
//     // // NotifyEmailAddress() mail.Address
//     // // ValidateEmail(email string, rejectDisposableDomains bool) (err error)
//     // // GuessImageFormat(data []byte) (format string)

//     // // Tasks
//     // async fn dispatch_delete_old_data_job(&self) -> Result<(), Error>;

//     // // Jobs
//     // // SendRegisterEmail(ctx context.Context, message events.UsersSendRegisterEmail) (err error)
//     // // SendSignInEmail(ctx context.Context, message events.UsersSendSignInEmail) (err error)
//     // // SendPaymentFailedEmail(ctx context.Context, message events.KernelSendPaymentFailedEmail) (err error)
//     // // SendPaymentActionRequiredEmail(ctx context.Context, message events.KernelSendPaymentActionRequiredEmail) (err error)
//     // // SendVerifyEmailEmail(ctx context.Context, message events.UsersSendVerifyEmailEmail) (err error)
//     // // SendEmailChangedEmail(ctx context.Context, message events.UsersSendEmailChangedEmail) (err error)
//     // // SendGroupInvitationEmail(ctx context.Context, message events.KernelSendGroupInvitationEmail) (err error)
//     // async fn delete_old_data(&self) -> Result<(), Error>;
// }

// /// Repository is the repository for all the entities of the kernel
// #[async_trait::async_trait]
// pub trait Repository: Debug + Send + Sync {
//     // User
//     // async fn create_user(&self, db: &DB, user: &User) -> Result<(), Error>;
//     // async fn update_user(&self, db: &DB, user: &User) -> Result<(), Error>;
//     // async fn delete_user(&self, db: &DB, user_id: uuid::Uuid) -> Result<(), Error>;
//     async fn find_user_by_id<'c, C: Executor<'c, Database = Postgres>>(&self, db: C, user_id: uuid::Uuid) -> Result<User, Error>;
//     // async fn find_all_users(&self, db: &DB) -> Result<Vec<User>, Error>;
//     // async fn find_user_by_email_or_username(&self, db: &DB, email_or_username: &str) -> Result<User, Error>;
//     // async fn find_user_by_email(&self, db: &DB, email: &str) -> Result<User, Error>;
//     // async fn find_user_by_username(&self, db: &DB, username: &str) -> Result<User, Error>;
//     // async fn get_users_count(&self, db: &DB) -> Result<u64, Error>;
//     // async fn find_users_by_usernames(&self, db: &DB, usernames: &[String]) -> Result<Vec<User>, Error>;
//     // async fn find_user_by_namespace_id(&self, db: &DB, namespace_id: uuid::Uuid) -> Result<User, Error>;

//     // // Session
//     // async fn create_session(&self, db: &DB, session: &Session) -> Result<(), Error>;
//     // async fn delete_session(&self, db: &DB, session_id: uuid::Uuid) -> Result<(), Error>;
//     // async fn find_session_by_id(&self, db: &DB, session_id: uuid::Uuid) -> Result<Session, Error>;
//     // async fn find_sessions_by_user_id(&self, db: &DB, user_id: uuid::Uuid) -> Result<Vec<Session>, Error>;

//     // // PendingUser
//     // async fn find_pending_user_by_id(&self, db: &DB, pending_user_id: uuid::Uuid) -> Result<PendingUser, Error>;
//     // async fn create_pending_user(&self, db: &DB, pending_user: &PendingUser) -> Result<(), Error>;
//     // async fn delete_pending_user(&self, db: &DB, pending_user_id: uuid::Uuid) -> Result<(), Error>;
//     // async fn update_pending_user(&self, db: &DB, pending_user: &PendingUser) -> Result<(), Error>;
//     // async fn delete_old_pending_users(&self, db: &DB, older_than_date: chrono::DateTime<chrono::Utc>) -> Result<(), Error>;

//     // // PendingSession
//     // async fn find_pending_session_by_id(&self, db: &DB, pending_session_id: uuid::Uuid) -> Result<PendingSession, Error>;
//     // async fn create_pending_session(&self, db: &DB, pending_session: &PendingSession) -> Result<(), Error>;
//     // async fn delete_pending_session(&self, db: &DB, pending_session_id: uuid::Uuid) -> Result<(), Error>;
//     // async fn update_pending_session(&self, db: &DB, pending_session: &PendingSession) -> Result<(), Error>;
//     // async fn delete_old_pending_sessions(&self, db: &DB, older_than_date: chrono::DateTime<chrono::Utc>) -> Result<(), Error>;

//     // // PendingEmail
//     // async fn find_pending_email_by_id(&self, db: &DB, pending_email_id: uuid::Uuid) -> Result<PendingEmail, Error>;
//     // async fn create_pending_email(&self, db: &DB, pending_email: &PendingEmail) -> Result<(), Error>;
//     // async fn delete_pending_email(&self, db: &DB, pending_session_id: uuid::Uuid) -> Result<(), Error>;
//     // async fn delete_old_pending_emails(&self, db: &DB, older_than_date: chrono::DateTime<chrono::Utc>) -> Result<(), Error>;

//     // // Group
//     // async fn create_group(&self, db: &DB, group: &Group) -> Result<(), Error>;
//     // async fn update_group(&self, db: &DB, group: &Group) -> Result<(), Error>;
//     // async fn find_group_by_namespace_id(&self, db: &DB, namespace_id: uuid::Uuid) -> Result<Group, Error>;
//     // async fn find_groups_for_user(&self, db: &DB, user_id: uuid::Uuid) -> Result<Vec<Group>, Error>;
//     // async fn delete_group(&self, db: &DB, group_id: uuid::Uuid) -> Result<(), Error>;
//     // async fn find_group_by_id(&self, db: &DB, group_id: uuid::Uuid) -> Result<Group, Error>;
//     // async fn get_group_admins_count(&self, db: &DB, group_id: uuid::Uuid) -> Result<u64, Error>;
//     // async fn find_all_groups(&self, db: &DB) -> Result<Vec<Group>, Error>;

//     // // Group membership
//     // async fn find_group_membership(&self, db: &DB, user_id: uuid::Uuid, group_id: uuid::Uuid) -> Result<GroupMembership, Error>;
//     // async fn create_membership(&self, db: &DB, group_membership: &GroupMembership) -> Result<(), Error>;
//     // async fn delete_all_group_memberships(&self, db: &DB, group_id: uuid::Uuid) -> Result<(), Error>;
//     // async fn delete_membership(&self, db: &DB, group_membership: &GroupMembership) -> Result<(), Error>;
//     // async fn find_group_members(&self, db: &DB, group_id: uuid::Uuid) -> Result<Vec<GroupMember>, Error>;
//     // async fn find_group_invitees(&self, db: &DB, group_id: uuid::Uuid) -> Result<Vec<User>, Error>;
//     // async fn create_group_invitations(&self, db: &DB, invitation: &GroupInvitation) -> Result<(), Error>;
//     // async fn delete_group_invitation(&self, db: &DB, invitation_id: uuid::Uuid) -> Result<(), Error>;
//     // async fn find_group_invitations_by_invitee_id(&self, db: &DB, invitee_id: uuid::Uuid) -> Result<Vec<GroupInvitation>, Error>;
//     // async fn find_group_invitation_by_id(&self, db: &DB, invitation_id: uuid::Uuid) -> Result<GroupInvitation, Error>;
//     // async fn find_group_membership_by_username(&self, db: &DB, group_id: uuid::Uuid, username: &str) -> Result<GroupMembership, Error>;
//     // async fn find_group_invitations_by_group_id(&self, db: &DB, group_id: uuid::Uuid) -> Result<Vec<GroupInvitation>, Error>;

//     // // Namespaces
//     // async fn create_namespace(&self, db: &DB, namespace: &Namespace) -> Result<(), Error>;
//     // async fn update_namespace(&self, db: &DB, namespace: &Namespace) -> Result<(), Error>;
//     // async fn delete_namespace(&self, db: &DB, path: &str) -> Result<(), Error>;
//     // async fn find_namespace_by_path(&self, db: &DB, path: &str) -> Result<Namespace, Error>;
//     // async fn get_subscribers_count_for_plan(&self, db: &DB, plan: BillingPlan) -> Result<u64, Error>;
//     // async fn find_namespace_by_id(&self, db: &DB, namespace_id: uuid::Uuid) -> Result<Namespace, Error>;
//     // async fn find_groups_namespaces_for_user(&self, db: &DB, user_id: uuid::Uuid) -> Result<Vec<Namespace>, Error>;
//     // async fn find_all_groups_namespaces(&self, db: &DB) -> Result<Vec<Namespace>, Error>;
//     // async fn find_namespace_by_customer_id(&self, db: &DB, customer_id: uuid::Uuid) -> Result<Namespace, Error>;

//     // // Customer
//     // async fn find_customer_by_namespace_id(&self, db: &DB, namespace_id: uuid::Uuid) -> Result<Customer, Error>;
//     // async fn create_customer(&self, db: &DB, customer: &Customer) -> Result<(), Error>;
//     // async fn update_customer(&self, db: &DB, customer: &Customer) -> Result<(), Error>;
//     // async fn find_customer_by_stripe_customer_id(&self, db: &DB, stripe_customer_id: String) -> Result<Customer, Error>;

//     // // Invoice
//     // async fn create_invoice(&self, db: &DB, invoice: &Invoice) -> Result<(), Error>;
//     // async fn update_invoice(&self, db: &DB, invoice: &Invoice) -> Result<(), Error>;
//     // async fn find_invoices_for_namespace(&self, db: &DB, naemspace_id: uuid::Uuid) -> Result<Vec<Invoice>, Error>;
//     // async fn find_invoice_by_stripe_invoice_id(&self, db: &DB, stripe_invoice_id: &str) -> Result<Invoice, Error>;
// }
