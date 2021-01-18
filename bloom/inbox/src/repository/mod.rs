mod create_chatbox_preferences;
mod create_contact;
mod create_newsletter_list;
mod create_newsletter_list_subscription;
mod create_newsletter_message;
mod delete_contact;
mod delete_newsletter_list;
mod delete_newsletter_list_subscription;
mod delete_newsletter_message;
mod find_archived_conversations;
mod find_chatbox_preferences_for_namespace;
mod find_contact_by_email;
mod find_contact_by_id;
mod find_contacts_for_namespace;
mod find_inbox_conversations;
mod find_newsletter_list_by_id;
mod find_newsletter_lists_for_namespace;
mod find_newsletter_message_by_id;
mod find_newsletter_messages_for_namespace;
mod find_newsletter_subscription_by_contact_id_and_list_id;
mod find_spam_conversations;
mod find_trashed_conversations;
mod update_chatbox_preferences;
mod update_contact;
mod update_newsletter_list;
mod update_newsletter_message;

#[derive(Debug)]
pub struct Repository {}

impl Repository {
    pub fn new() -> Repository {
        Repository {}
    }
}
