use serde::{Deserialize, Serialize};

use crate::api::scalars::{Id, Time};

pub mod input;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: Id,
    pub created_at: Time,
    pub name: String,
    pub birthday: Option<Time>,
    pub email: String,
    pub pgp_key: String,
    pub phone: String,
    pub address: String,
    pub website: String,
    pub twitter: String,
    pub instagram: String,
    pub facebook: String,
    pub linkedin: String,
    pub skype: String,
    pub telegram: String,
    pub bloom: String,
    pub notes: String,
    pub country: String,
    pub country_code: String,
    pub plan: String,
    pub user_id: String,
    pub avatar_url: String,
}

pub fn convert_contact(kernel: &kernel::Service, contact: inbox::entities::Contact) -> Contact {
    Contact {
        avatar_url: kernel.get_avatar_url(contact.avatar_id.as_ref()),
        id: contact.id,
        created_at: contact.created_at,
        name: contact.name,
        birthday: contact.birthday,
        email: contact.email,
        pgp_key: contact.pgp_key,
        phone: contact.phone,
        address: contact.address,
        website: contact.website,
        twitter: contact.twitter,
        instagram: contact.instagram,
        facebook: contact.facebook,
        linkedin: contact.linkedin,
        skype: contact.skype,
        telegram: contact.telegram,
        bloom: contact.bloom,
        notes: contact.notes,
        country: contact.country,
        country_code: contact.country_code,
        plan: contact.plan,
        user_id: contact.user_id,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Id,
    pub received_at: Time,
    pub body_html: String,
    pub from_operator: bool,
    pub conversation_id: Id,
}

impl From<inbox::entities::Message> for Message {
    fn from(message: inbox::entities::Message) -> Self {
        Message {
            id: message.id,
            received_at: message.received_at,
            body_html: message.body_html,
            from_operator: message.from_operator,
            conversation_id: message.conversation_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatboxMessage {
    pub id: Id,
    pub received_at: Time,
    pub body_html: String,
    pub from_operator: bool,
}

impl From<inbox::entities::Message> for ChatboxMessage {
    fn from(message: inbox::entities::Message) -> Self {
        ChatboxMessage {
            id: message.id,
            received_at: message.received_at,
            body_html: message.body_html,
            from_operator: message.from_operator,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Id,
    pub created_at: Time,
    pub archived_at: Option<Time>,
    pub trashed_at: Option<Time>,
    pub last_message_at: Time,
    pub is_spam: bool,
    pub name: String,
    pub description: String,
}

impl From<inbox::entities::Conversation> for Conversation {
    fn from(conversation: inbox::entities::Conversation) -> Self {
        Conversation {
            id: conversation.id,
            created_at: conversation.created_at,
            archived_at: conversation.archived_at,
            trashed_at: conversation.trashed_at,
            last_message_at: conversation.last_message_at,
            is_spam: conversation.is_spam,
            name: conversation.name,
            description: conversation.description,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationWithContactsAndMessages {
    pub conversation: Conversation,
    pub contacts: Vec<Contact>,
    pub messages: Vec<Message>,
}

pub fn convert_conversation_with_messages_and_contacts(
    kernel: &kernel::Service,
    input: inbox::service::ConversationWithMessageAndContacts,
) -> ConversationWithContactsAndMessages {
    ConversationWithContactsAndMessages {
        conversation: input.conversation.into(),
        contacts: input.contacts.into_iter().map(|c| convert_contact(kernel, c)).collect(),
        messages: input.messages.into_iter().map(Into::into).collect(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatboxPreferences {
    pub color: String,
    pub name: String,
    pub avatar_url: String,
    pub show_branding: bool,
    pub welcome_message: String,
    pub base_url: String,
    pub twitter: String,
    pub twitter_url: String,
    pub facebook_url: String,
    pub instagram: String,
    pub instagram_url: String,
    pub whatsapp_number: String,
    pub whatsapp_url: String,
    pub mastodon_url: String,
    pub website_url: String,
    pub telegram: String,
    pub telegram_url: String,
}

pub fn convert_chatbox_preferences(
    kernel: &kernel::Service,
    input: inbox::service::DetailedChatboxPreferences,
) -> ChatboxPreferences {
    let twitter_url = if input.preferences.twitter.is_empty() {
        String::new()
    } else {
        format!("https://twitter.com/@{}", &input.preferences.twitter)
    };

    let instagram_url = if input.preferences.instagram.is_empty() {
        String::new()
    } else {
        format!("https://instagram.com/{}", &input.preferences.instagram)
    };

    let whatsapp_url = if input.preferences.whatsapp_number.is_empty() {
        String::new()
    } else {
        format!(
            "https://api.whatsapp.com/send?phone={}",
            &input.preferences.whatsapp_number
        )
    };

    let telegram_url = if input.preferences.telegram.is_empty() {
        String::new()
    } else {
        format!("https://t.me/@{}", &input.preferences.telegram)
    };

    ChatboxPreferences {
        avatar_url: kernel.get_avatar_url(input.preferences.avatar_id.as_ref()),
        color: input.preferences.color,
        name: input.preferences.name,
        show_branding: input.preferences.show_branding,
        welcome_message: input.preferences.welcome_message,
        base_url: input.base_url,
        twitter: input.preferences.twitter,
        twitter_url,
        facebook_url: input.preferences.facebook_url,
        instagram: input.preferences.instagram,
        instagram_url,
        whatsapp_number: input.preferences.whatsapp_number,
        whatsapp_url,
        mastodon_url: input.preferences.mastodon_url,
        website_url: input.preferences.website_url,
        telegram: input.preferences.telegram,
        telegram_url,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbox {
    pub conversations: Vec<ConversationWithContactsAndMessages>,
}
