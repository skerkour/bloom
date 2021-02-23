use crate::api::scalars::{Id, Time};
use consts::GroupRole;
use kernel::consts::{self, BillingPlan};
use serde::{Deserialize, Serialize};

pub mod input;

pub fn convert_user(kernel: &kernel::Service, user: kernel::entities::User, private_details: bool) -> User {
    let mut ret = User {
        avatar_url: kernel.get_avatar_url(user.avatar_id.as_ref()),
        id: None,
        created_at: None,
        name: user.name,
        username: user.username,
        namespace_id: None,
        two_fa_enabled: None,
        is_admin: None,
        email: None,
        description: user.description,
    };

    if private_details {
        ret.id = Some(user.id);
        ret.created_at = Some(user.created_at);
        ret.namespace_id = Some(user.namespace_id);
        ret.two_fa_enabled = Some(user.two_fa_enabled);
        ret.is_admin = Some(user.is_admin);
        ret.email = Some(user.email);
    }

    ret
}

pub fn convert_group(kernel: &kernel::Service, group: kernel::entities::Group, private_details: bool) -> Group {
    let mut ret = Group {
        avatar_url: kernel.get_avatar_url(group.avatar_id.as_ref()),
        id: None,
        created_at: None,
        namespace_id: None,
        name: group.name,
        path: group.path,
        description: group.description,
    };

    if private_details {
        ret.id = Some(group.id);
        ret.created_at = Some(group.created_at);
        ret.namespace_id = Some(group.namespace_id);
    }

    ret
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<Id>,
    pub created_at: Option<Time>,
    pub avatar_url: String,
    pub name: String,
    pub username: String,
    pub namespace_id: Option<Id>,
    pub two_fa_enabled: Option<bool>,
    pub is_admin: Option<bool>,
    pub email: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingInformation {
    pub namespace_id: Id,
    pub path: String,
    pub used_storage: i64,
    pub total_storage: i64,
    pub customer: Option<Customer>,
}

impl From<kernel::entities::BillingInformation> for BillingInformation {
    fn from(info: kernel::entities::BillingInformation) -> Self {
        let customer = match info.customer {
            Some(customer) => Some(customer.into()),
            None => None,
        };
        BillingInformation {
            namespace_id: info.namespace.id,
            path: info.namespace.path,
            used_storage: info.namespace.used_storage,
            total_storage: info.total_storage,
            customer,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub plan: BillingPlan,
    pub name: String,
    pub email: String,
    pub country: String,
    pub country_code: String,
    pub city: String,
    pub postal_code: String,
    pub address_line1: String,
    pub address_line2: String,
    pub state: String,
    pub tax_id: Option<String>,
}

impl From<kernel::entities::Customer> for Customer {
    fn from(customer: kernel::entities::Customer) -> Self {
        Customer {
            plan: customer.plan,
            name: customer.name,
            email: customer.email,
            country: customer.country,
            country_code: customer.country_code,
            city: customer.city,
            postal_code: customer.postal_code,
            address_line1: customer.address_line1,
            address_line2: customer.address_line2,
            state: customer.state,
            tax_id: customer.tax_id,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegistrationStarted {
    pub pending_user_id: Id,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Me {
    pub session: Session,
    pub user: User,
    pub groups: Vec<Group>,
}

pub fn convert_me(kernel: &kernel::Service, me: kernel::service::Me) -> Me {
    Me {
        session: me.session.into(),
        user: convert_user(kernel, me.user, true),
        groups: me.groups.into_iter().map(|g| convert_group(kernel, g, true)).collect(),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: Id,
    pub created_at: Time,
}

impl From<kernel::entities::Session> for Session {
    fn from(session: kernel::entities::Session) -> Self {
        Session {
            id: session.id,
            created_at: session.created_at,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignedIn {
    pub me: Option<Me>,
    pub two_fa_method: Option<TwoFaMethod>,
    pub token: Option<String>,
}

pub fn convert_registered(kernel: &kernel::Service, input: kernel::service::Registered) -> SignedIn {
    SignedIn {
        me: Some(Me {
            user: convert_user(kernel, input.user, true),
            session: input.session.into(),
            groups: Vec::new(),
        }),
        token: Some(input.token),
        two_fa_method: None,
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TwoFaMethod {
    Totp,
}

impl From<kernel::consts::TwoFaMethod> for TwoFaMethod {
    fn from(item: kernel::consts::TwoFaMethod) -> Self {
        match item {
            kernel::consts::TwoFaMethod::Totp => TwoFaMethod::Totp,
        }
    }
}

pub fn convert_signed_in(kernel: &kernel::Service, item: kernel::service::SignedIn) -> SignedIn {
    match item {
        kernel::service::SignedIn::Success {
            me,
            token,
        } => SignedIn {
            me: Some(convert_me(kernel, me)),
            two_fa_method: None,
            token: Some(token),
        },
        kernel::service::SignedIn::TwoFa(method) => SignedIn {
            me: None,
            token: None,
            two_fa_method: Some(method.into()),
        },
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignInStarted {
    pub pending_session_id: Id,
}

impl From<kernel::entities::PendingSession> for SignInStarted {
    fn from(item: kernel::entities::PendingSession) -> Self {
        SignInStarted {
            pending_session_id: item.id,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Success {
    pub success: bool,
}

impl From<bool> for Success {
    fn from(item: bool) -> Self {
        Success {
            success: item,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Empty {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SetupTwoFa {
    pub base64_qr_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: Option<Id>,
    pub created_at: Option<Time>,
    pub avatar_url: String,
    pub namespace_id: Option<Id>,
    pub name: String,
    pub path: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignedUploadUrl {
    pub url: String,
    pub upload_id: Id,
}

impl From<kernel::service::SignedUploadUrl> for SignedUploadUrl {
    fn from(item: kernel::service::SignedUploadUrl) -> Self {
        SignedUploadUrl {
            url: item.url,
            upload_id: item.upload_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct QrCode {
    pub base64_jpeg_qr_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct GroupInvitation {
    pub id: Id,
    pub created_at: Time,
    pub group: Group,
    pub inviter: User,
    pub invitee: User,
}

pub fn convert_group_invitation_with_details(
    kernel: &kernel::Service,
    item: kernel::service::GroupInvitationWithDetails,
) -> GroupInvitation {
    GroupInvitation {
        id: item.invitation.id,
        created_at: item.invitation.created_at,
        group: convert_group(kernel, item.group, false),
        inviter: convert_user(kernel, item.inviter, false),
        invitee: convert_user(kernel, item.invitee, false),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarkdownHtml {
    pub html: String,
}

#[derive(Serialize, Deserialize)]
pub struct GroupMember {
    pub user_id: Id,
    pub username: String,
    pub avatar_url: String,
    pub name: String,
    pub role: GroupRole,
    pub joined_at: Time,
}

pub fn convert_group_member(kernel: &kernel::Service, item: kernel::entities::GroupMember) -> GroupMember {
    GroupMember {
        avatar_url: kernel.get_avatar_url(item.avatar_id.as_ref()),
        user_id: item.user_id,
        username: item.username,
        name: item.name,
        role: item.role,
        joined_at: item.joined_at,
    }
}

#[derive(Serialize, Deserialize)]
pub struct GroupWithMembersAndInvitations {
    pub group: Group,
    pub members: Vec<GroupMember>,
    pub invitations: Vec<GroupInvitation>,
}

pub fn convert_group_with_members_and_invitations(
    kernel: &kernel::Service,
    item: kernel::service::GroupWithMembersAndInvitations,
) -> GroupWithMembersAndInvitations {
    GroupWithMembersAndInvitations {
        group: convert_group(kernel, item.group, true),
        invitations: item
            .invitations
            .into_iter()
            .map(|i| convert_group_invitation_with_details(kernel, i))
            .collect(),
        members: item
            .members
            .into_iter()
            .map(|m| convert_group_member(kernel, m))
            .collect(),
    }
}

#[derive(Serialize, Deserialize)]
pub struct StripePublicKey {
    pub stripe_public_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct CheckoutSession {
    pub checkout_session_id: String,
    pub stripe_public_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct CustomerPortal {
    pub customer_portal_url: String,
}
