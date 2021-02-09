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
  from_operator: boolean;
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

export type GetChatboxPreferences = {
  namespace_id: string;
};

export type GetChatboxMessages = {
  namespace_id: string;
  after: Option<string>,
};


export type LinkChatboxContact = {
  namespace_id: string;
  email: string;
};

export type SendChatboxMessage = {
  namespace_id: string;
  body: String;
};
