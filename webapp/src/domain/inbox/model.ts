/* eslint-disable */
import { Option } from '../kernel/model';

export type Contact = {
  id: string;
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

export type GetArchive = {
  namespace_id: string;
};

export type GetInbox = {
  namespace_id: string;
};

export type GetSpam = {
  namespace_id: string;
};

export type GetTrash = {
  namespace_id: string;
};

export type Inbox = {
  conversations: ConversationWithContactsAndMessages[];
};

export type Message = {
  id: string;
  conversation_id: string;
};

export type SendMessage = {
  conversation_id: string;
  body: string;
};
