mod create_contact;
mod create_newsletter_list;
mod create_newsletter_message;
mod delete_contact;
mod delete_newsletter_list;
mod find_contact_by_id;
mod find_newsletter_list_by_id;

#[derive(Debug)]
pub struct Repository {}

impl Repository {
    pub fn new() -> Repository {
        Repository {}
    }
}
