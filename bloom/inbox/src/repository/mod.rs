mod create_contact;
mod create_newsletter_list;
mod create_newsletter_message;
mod delete_contact;
mod delete_newsletter_list;
mod delete_newsletter_message;
mod find_archived_conversations;
mod find_contact_by_id;
mod find_inbox_conversations;
mod find_newsletter_list_by_id;
mod find_newsletter_message_by_id;
mod find_spam_conversations;
mod find_trashed_conversations;

#[derive(Debug)]
pub struct Repository {}

impl Repository {
    pub fn new() -> Repository {
        Repository {}
    }
}
