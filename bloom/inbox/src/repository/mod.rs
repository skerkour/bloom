mod create_chatbox_preferences;
mod create_contact;
mod create_contact_anonymous_id_relation;
mod create_conversation;
mod create_conversation_contact_relation;
mod create_inbox_message;
mod create_newsletter_list;
mod create_newsletter_list_subscription;
mod create_newsletter_message;
mod delete_contact;
mod delete_newsletter_list;
mod delete_newsletter_list_subscription;
mod delete_newsletter_message;
mod detach_messages_from_user;
mod find_archived_conversations;
mod find_chatbox_preferences_for_namespace;
mod find_contact_by_anonymous_id;
mod find_contact_by_email;
mod find_contact_by_id;
mod find_contacts_for_list;
mod find_contacts_for_namespace;
mod find_conversation_by_id;
mod find_inbox_conversation_for_anonymous_id;
mod find_inbox_conversations_by_namespace_id;
mod find_inbox_messages_for_conversation;
mod find_newsletter_list_acquisition;
mod find_newsletter_list_by_id;
mod find_newsletter_lists_for_namespace;
mod find_newsletter_message_by_id;
mod find_newsletter_messages_by_list_id;
mod find_newsletter_messages_for_namespace;
mod find_newsletter_subscription_by_contact_id_and_list_id;
mod find_spam_conversations;
mod find_subscriptions_for_list;
mod find_trashed_conversations;
mod update_chatbox_preferences;
mod update_contact;
mod update_conversation;
mod update_newsletter_list;
mod update_newsletter_message;

#[derive(Debug)]
pub struct Repository {}

impl Repository {
    pub fn new() -> Repository {
        Repository {}
    }
}
