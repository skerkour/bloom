/* eslint-disable */
import { Option } from '../kernel/model';

export type Contact = {
  id: string;
  name: string;
  email: string;
};

export type CreateList = {
  namespace_id: string;
  name: string;
  description: string;
};

export type CreateMessage = {
  list_id: string;
  name: string;
  subject: string;
  body: string;
};

export type DeleteList = {
  list_id: string;
};

export type DeleteMessage = {
  message_id: string;
};

export type GetList = {
  list_id: string;
};

export type GetLists = {
  namespace_id: string;
};

export type GetMessage = {
  message_id: string;
};

export type List = {
  id: string;
  created_at: string;
  name: string;
  description: string;
};

export type ListAcquisition = {
  date: string;
  new_contacts: number;
};

export type ListWithDetails = {
  list: List;
  contacts: Contact[];
  messages: Message[];
  acquisition: ListAcquisition[];
};

export type Message = {
  id: string;
  created_at: string;
  name: string;
  subject: string;
  body: string;
  body_html: string;
  status: string;
  scheduled_for: Option<string>,
  last_sent_at: Option<string>,
  sent_count: number;
  error_count: number;
};

export type MessageWithLists = {
  message: Message;
  list: List;
  lists: List[];
};

export type RemoveContactFromList = {
  list_id: string;
  contact_id: string;
};


export type SendMessage = {
  message_id: string;
};

export type SendTestMessage = {
  message_id: string;
};

export type SubscribeToList = {
  name: Option<string>;
  email: string;
  list_id: string;
};

export type UnsubscribeFromList = {
  subscription_id: string;
};

export type UpdateList = {
  list_id: string;
  name: string;
  description: string;
};

export type UpdateMessage = {
  message_id: string;
  list_id: string;
  name: string;
  subject: string;
  body: string;
};
