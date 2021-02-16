/* eslint-disable */
import { Option } from '../kernel/model';

export type Contact = {
  id: string;
  created_at: string;
  name: string;
  birthday: Option<string>,
  email: string;
  pgp_key: string;
  phone: string;
  address: string;
  website: string;
  twitter: string;
  instagram: string;
  facebook: string;
  linkedin: string;
  skype: string;
  telegram: string;
  bloom: string;
  notes: string;
  country: string;
  country_code: string;
  plan: string;
  user_id: string;
  avatar_url: string;
};

export type CreateContact = {
  namespace_id: string;
  name: string;
  birthday: Option<string>;
  email: string;
  pgp_key: string;
  phone: string;
  address: string;
  website: string;
  twitter: string;
  instagram: string;
  facebook: string;
  linkedin: string;
  skype: string;
  telegram: string;
  bloom: string;
  notes: string;
  plan: string;
  user_id: string;
};

export type ChatboxPreferences = {
  color: string;
  name: string;
  avatar_url: string;
  show_branding: boolean;
  welcome_message: string;
  base_url: string;
  twitter: string;
  twitter_url: string;
  facebook_url: string;
  instagram: string;
  instagram_url: string;
  whatsapp_number: string;
  whatsapp_url: string;
  mastodon_url: string;
  website_url: string;
  telegram: string;
  telegram_url: string;
};

export type Conversation = {
  id: string;
  created_at: string;
  archived_at: Option<string>;
  trashed_at: Option<string>;
  last_message_at: string;
  is_spam: boolean;
  name: string;
  description: string;
};

export type ConversationWithContactsAndMessages = {
  conversation: Conversation,
  contacts: Contact[],
  messages: Message[],
};

export type DeleteContact = {
  contact_id: string;
};

export type GetArchive = {
  namespace_id: string;
  after: Option<string>,
};

export type GetContact = {
  contact_id: string;
};

export type GetContacts = {
  namespace_id: string;
};

export type GetChatboxPreferences = {
  namespace_id: string;
};

export type GetInbox = {
  namespace_id: string;
  after: Option<string>,
};

export type GetSpam = {
  namespace_id: string;
  after: Option<string>,
};

export type GetTrash = {
  namespace_id: string;
  after: Option<string>,
};

export type ImportContacts = {
  namespace_id: string;
  list_id: Option<string>;
  contacts_csv: string;
};

export type Inbox = {
  conversations: ConversationWithContactsAndMessages[];
};

export type Message = {
  id: string;
  received_at: string;
  body_html: string;
  from_operator: boolean;
  conversation_id: string;
};

export type MoveConversation = {
  conversation_id: string;
};

export type SendMessage = {
  conversation_id: string;
  body: string;
};

export type UpdateChatboxPreferences = {
  namespace_id: string;
  color: string;
  name: string;
  show_branding: boolean;
  welcome_message: string;
  twitter: string;
  facebook_url: string;
  instagram: string;
  whatsapp_number: string;
  mastodon_url: string;
  website_url: string;
  telegram: string;
};

export type UpdateContact = {
  contact_id: string;
  name: string;
  birthday: Option<string>,
  email: string;
  pgp_key: string;
  phone: string;
  address: string;
  website: string;
  twitter: string;
  instagram: string;
  facebook: string;
  linkedin: string;
  skype: string;
  telegram: string;
  bloom: string;
  notes: string;
  plan: string;
  user_id: string;
};
