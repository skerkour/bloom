/* eslint-disable */
export type Option<T> = T | null;

export type Chatbox = {
  preferences: ChatboxPreferences;
  messages: ChatboxMessage[];
};

export type ChatboxMessage = {
  id: string;
  received_at: string;
  body_html: string;
  author: null; // TODO
};

export type ChatboxPreferences = {
  color: string;
  name: string;
  avatar: string;
  show_branding: boolean;
  welcome_message: string;
  base_url: string;
};

export type GetChatboxPreferences = {
  namespace_id: string;
};

export type GetChatboxMessages = {
  namespace_id: string;
};

export type SendChatboxMessage = {
  namespace_id: string;
  body: String;
};
