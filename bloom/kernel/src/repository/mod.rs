mod create_customer;
mod create_group;
mod create_group_invitation;
mod create_group_membership;
mod create_namespace;
mod create_pending_email;
mod create_pending_session;
mod create_pending_user;
mod create_session;
mod create_upload;
mod create_user;
mod delete_group_invitation;
mod delete_group_membership;
mod delete_namespace;
mod delete_old_pending_sessions;
mod delete_old_pending_users;
mod delete_pending_email;
mod delete_pending_session;
mod delete_pending_user;
mod delete_session;
mod delete_upload;
mod detach_uploads_for_namespace;
mod find_all_groups;
mod find_all_users;
mod find_customer_by_namespace_id;
mod find_customer_by_stripe_customer_id;
mod find_group_by_id;
mod find_group_by_namespace_id;
mod find_group_by_path;
mod find_group_invitation_by_id;
mod find_group_invitations_for_group;
mod find_group_invitations_for_invitee;
mod find_group_invitees;
mod find_group_members;
mod find_group_membership;
mod find_group_membership_by_username;
mod find_groups_for_user;
mod find_namespace_by_id;
mod find_namespace_by_path;
mod find_namespace_group_membership;
mod find_old_uploads;
mod find_pending_email_by_id;
mod find_pending_session_by_id;
mod find_pending_user_by_id;
mod find_session_by_id;
mod find_sessions_by_user_id;
mod find_upload_by_id;
mod find_user_by_email;
mod find_user_by_email_or_username;
mod find_user_by_id;
mod find_user_by_namespace_id;
mod find_users_by_usernames;
mod get_group_admins_count;
mod get_users_count;
mod update_customer;
mod update_group;
mod update_namespace;
mod update_pending_email;
mod update_pending_session;
mod update_pending_user;
mod update_upload;
mod update_user;

#[derive(Debug, Clone)]
pub struct Repository {}

impl Repository {
    pub fn new() -> Repository {
        Repository {}
    }
}
