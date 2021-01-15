use crate::repository::Repository;
use kernel::{db::DB, drivers};
use std::sync::Arc;

mod create_contact;
mod create_newsletter_list;
mod create_newsletter_message;
mod delete_contact;
mod delete_newsletter_list;
mod delete_newsletter_message;
mod find_archive;
mod find_chatbox_preferences;
mod find_contact;
mod find_contacts;
mod find_inbox;
mod find_newsletter_list;
mod find_newsletter_lists;
mod find_newsletter_message;
mod find_newsletter_messages;
mod find_spam;
mod find_trash;
mod import_contacts;
mod init_namespace;
mod send_chatbox_message;
mod send_message;
mod send_newsletter_message;
mod send_test_newsletter_message;
mod update_chatbox_preferences;
mod update_contact;
mod update_newsletter_list;
mod update_newsletter_message;

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: DB,
    kernel_service: Arc<kernel::Service>,
    queue: Arc<dyn drivers::Queue>,
}

impl Service {
    pub fn new(kernel_service: Arc<kernel::Service>, db: DB, queue: Arc<dyn drivers::Queue>) -> Service {
        let repo = Repository::new();

        Service {
            db,
            repo,
            kernel_service,
            queue,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateContactInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct CreateNewsletterListInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct CreateNewsletterMessageInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct DeleteContactInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct DeleteNewsletterListInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct DeleteNewsletterMessageInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct ImportContactsInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct SendNewsletterMessageInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct SendTestNewsletterMessageInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct UpdateContactInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct UpdateNewsletterListInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct UpdateNewsletterMessageInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct FindContactInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct FindContactsInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct FindNewsletterListInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct FindNewsletterListsInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct FindNewsletterMessageInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct FindNewsletterMessagesInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct SendMessageInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct SendChatboxMessageInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct UpdateChatboxPreferencesInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct FindChatboxPreferencesInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct FindChatboxMessagesInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct InitNamespaceInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct FindInboxInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct FindTrashInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct FindArchiveInput {
    // TODO
}

#[derive(Debug, Clone)]
pub struct FindSpamInput {
    // TODO
}
