-- #################################################################################################
-- Kernel
-- #################################################################################################
CREATE TABLE kernel_queue (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  scheduled_for TIMESTAMP WITH TIME ZONE NOT NULL,
  failed_attempts INT NOT NULL,
  status INT NOT NULL,
  message JSONB NOT NULL
);
CREATE INDEX index_kernel_queue_on_scheduled_for ON kernel_queue (scheduled_for);
CREATE INDEX index_kernel_queue_on_status ON kernel_queue (status);


CREATE TABLE kernel_namespaces (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  path TEXT NOT NULL,
  type TEXT NOT NULL,
  used_storage BIGINT NOT NULL,
  plan TEXT NOT NULL,

  parent_id UUID REFERENCES kernel_namespaces(id)
);
CREATE INDEX index_kernel_namespaces_on_parent_id ON kernel_namespaces (parent_id);
CREATE UNIQUE INDEX index_kernel_namespaces_on_path ON kernel_namespaces (path);


CREATE TABLE kernel_users (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
  blocked_at TIMESTAMP WITH TIME ZONE,

  username TEXT NOT NULL,
  email TEXT NOT NULL,
  is_admin BOOLEAN NOT NULL,
  two_fa_enabled BOOLEAN NOT NULL,
  two_fa_method TEXT,
  encrypted_totp_secret BYTEA,
  totp_secret_nonce BYTEA,

  -- namespace fields
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  avatar_storage_key TEXT,

  namespace_id UUID NOT NULL REFERENCES kernel_namespaces(id) ON DELETE CASCADE
);
CREATE UNIQUE INDEX index_kernel_users_on_username ON kernel_users (username);
CREATE UNIQUE INDEX index_kernel_users_on_email ON kernel_users (email);
CREATE UNIQUE INDEX index_kernel_users_on_namespace_id ON kernel_users (namespace_id);


CREATE TABLE kernel_pending_users (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  email TEXT NOT NULL,
  username TEXT NOT NULL,
  code_hash TEXT NOT NULL,
  failed_attempts BIGINT NOT NULL
);


CREATE TABLE kernel_pending_emails (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  email TEXT NOT NULL,
  code_hash BYTEA NOT NULL,
  failed_attempts BIGINT NOT NULL,

  user_id UUID NOT NULL REFERENCES kernel_users(id) ON DELETE CASCADE
);


CREATE TABLE kernel_sessions (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  secret_hash BYTEA NOT NULL,

  user_id UUID NOT NULL REFERENCES kernel_users(id) ON DELETE CASCADE
);
CREATE INDEX index_kernel_sessions_on_user_id ON kernel_sessions (user_id);


CREATE TABLE kernel_pending_sessions (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  code_hash TEXT NOT NULL,
  failed_attempts BIGINT NOT NULL,
  two_fa_verified BOOLEAN NOT NULL,

  user_id UUID NOT NULL REFERENCES kernel_users(id) ON DELETE CASCADE
);


CREATE TABLE kernel_groups (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  path TEXT NOT NULL,

  -- namespace fields
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  avatar_storage_key TEXT,

  namespace_id UUID NOT NULL REFERENCES kernel_namespaces(id) ON DELETE CASCADE
);
CREATE UNIQUE INDEX index_kernel_groups_on_namespace_id ON kernel_groups (namespace_id);


CREATE TABLE kernel_groups_members (
  joined_at TIMESTAMP WITH TIME ZONE NOT NULL,

  role TEXT NOT NULL,

  group_id UUID NOT NULL REFERENCES kernel_groups(id) ON DELETE CASCADE,
  user_id UUID NOT NULL REFERENCES kernel_users(id)
);
CREATE INDEX index_kernel_groups_members_on_group_id ON kernel_groups_members (group_id);
CREATE INDEX index_kernel_groups_members_on_user_id ON kernel_groups_members (user_id);


CREATE TABLE kernel_group_invitations (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  group_id UUID NOT NULL REFERENCES kernel_groups(id) ON DELETE CASCADE,
  inviter_id UUID NOT NULL REFERENCES kernel_users(id) ON DELETE CASCADE,
  invitee_id UUID NOT NULL REFERENCES kernel_users(id) ON DELETE CASCADE
);
CREATE INDEX index_kernel_group_invitations_on_group_id ON kernel_group_invitations (group_id);
CREATE INDEX index_kernel_group_invitations_on_invitee_id ON kernel_group_invitations (invitee_id);


CREATE TABLE kernel_uploads (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  size BIGINT NOT NULL,
  completed BOOLEAN NOT NULL,

  namespace_id UUID REFERENCES kernel_namespaces(id)
);
CREATE INDEX index_kernel_uploads_on_created_at ON kernel_uploads (created_at);


CREATE TABLE kernel_customers (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  subscription_updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
  plan TEXT NOT NULL,
  name TEXT NOT NULL,
  email TEXT NOT NULL,
  country TEXT NOT NULL,
  country_code TEXT NOT NULL,
  city TEXT NOT NULL,
  postal_code TEXT NOT NULL,
  address_line1 TEXT NOT NULL,
  address_line2 TEXT NOT NULL,
  state TEXT NOT NULL,
  tax_id_type TEXT,
  tax_id TEXT,

  stripe_customer_id TEXT NOT NULL,
  stripe_subscription_id TEXT,
  stripe_product_id TEXT,
  stripe_price_id TEXT,
  stripe_tax_id TEXT,
  stripe_default_payment_method_id TEXT,

  namespace_id UUID REFERENCES kernel_namespaces(id)
);
CREATE INDEX index_kernel_customers_on_namespace_id ON kernel_customers (namespace_id);
CREATE INDEX index_kernel_customers_on_stripe_customer_id ON kernel_customers (stripe_customer_id);
CREATE INDEX index_kernel_customers_on_plan ON kernel_customers (plan);


-- #################################################################################################
-- Files
-- #################################################################################################
CREATE TABLE files (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  name TEXT NOT NULL,
  size BIGINT NOT NULL,
  type TEXT NOT NULL,
  explicitly_trashed BOOLEAN NOT NULL,
  trashed_at TIMESTAMP WITH TIME ZONE,

  namespace_id UUID REFERENCES kernel_namespaces (id), -- no on delete cascade, because need to be removed from storage
  parent_id UUID REFERENCES files (id)
);
CREATE INDEX index_files_on_namespace_id ON files (namespace_id);
CREATE INDEX index_files_on_parent_id ON files (parent_id);
CREATE INDEX index_files_on_type ON files (type);
CREATE INDEX index_files_on_name ON files (name);
CREATE INDEX index_files_on_explicitly_trashed ON files (explicitly_trashed);


-- #################################################################################################
-- Analytics
-- #################################################################################################
CREATE TABLE analytics_visitors (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  anonymous_id UUID NOT NULL,

  -- contact_id UUID REFERENCES contacts (id) ON DELETE CASCADE,
  namespace_id UUID NOT NULL REFERENCES kernel_namespaces (id) ON DELETE CASCADE
);
CREATE INDEX index_analytics_visitors_on_anonymous_id ON analytics_visitors (anonymous_id);
CREATE INDEX index_analytics_visitors_on_namespace_id ON analytics_visitors (namespace_id);


CREATE TABLE analytics_page_events (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
  sent_at TIMESTAMP WITH TIME ZONE NOT NULL,
  received_at TIMESTAMP WITH TIME ZONE NOT NULL,

  page_name TEXT NOT NULL,
  url TEXT NOT NULL,
  user_agent TEXT NOT NULL,
  referrer TEXT NOT NULL,
  device_type TEXT NOT NULL,
  country TEXT NOT NULL,
  country_code TEXT NOT NULL,
  os_name TEXT NOT NULL,
  os_version TEXT NOT NULL,
  browser_name TEXT NOT NULL,
  browser_version TEXT NOT NULL,
  path TEXT NOT NULL,
  screen_width BIGINT NOT NULL,
  screen_height BIGINT NOT NULL,

  visitor_id UUID NOT NULL REFERENCES analytics_visitors (id) ON DELETE CASCADE,
  namespace_id UUID NOT NULL REFERENCES kernel_namespaces (id) ON DELETE CASCADE
);
CREATE INDEX index_analytics_page_events_on_namespace_id ON analytics_page_events (namespace_id);
CREATE INDEX index_analytics_page_events_on_visitor_id ON analytics_page_events (visitor_id);
CREATE INDEX index_analytics_page_events_on_timestamp ON analytics_page_events (timestamp);


CREATE TABLE analytics_track_events (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
  sent_at TIMESTAMP WITH TIME ZONE NOT NULL,
  received_at TIMESTAMP WITH TIME ZONE NOT NULL,

  event_name TEXT NOT NULL,
  properties JSONB NOT NULL,

  page_name TEXT NOT NULL,
  url TEXT NOT NULL,
  user_agent TEXT NOT NULL,
  referrer TEXT NOT NULL,
  device_type TEXT NOT NULL,
  country TEXT NOT NULL,
  country_code TEXT NOT NULL,
  os_name TEXT NOT NULL,
  os_version TEXT NOT NULL,
  browser_name TEXT NOT NULL,
  browser_version TEXT NOT NULL,
  path TEXT NOT NULL,
  screen_width BIGINT NOT NULL,
  screen_height BIGINT NOT NULL,

  visitor_id UUID NOT NULL REFERENCES analytics_visitors (id) ON DELETE CASCADE,
  namespace_id UUID NOT NULL REFERENCES kernel_namespaces (id) ON DELETE CASCADE
);
CREATE INDEX index_analytics_track_events_on_namespace_id ON analytics_track_events (namespace_id);
CREATE INDEX index_analytics_track_events_on_visitor_id ON analytics_track_events (visitor_id);
CREATE INDEX index_analytics_track_events_on_timestamp ON analytics_track_events (timestamp);



-- #################################################################################################
-- Inbox
-- #################################################################################################
CREATE TABLE inbox_conversations (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  archived_at TIMESTAMP WITH TIME ZONE,
  trashed_at TIMESTAMP WITH TIME ZONE,
  last_message_at TIMESTAMP WITH TIME ZONE NOT NULL,
  is_spam BOOLEAN NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  anonymous_id UUID,

  namespace_id UUID NOT NULL REFERENCES kernel_namespaces (id) ON DELETE CASCADE
);
CREATE INDEX index_inbox_conversations_on_namespace_id ON inbox_conversations (namespace_id);
CREATE INDEX index_inbox_conversations_on_last_message_at ON inbox_conversations (last_message_at);
CREATE INDEX index_inbox_conversations_on_archived_at ON inbox_conversations (archived_at);
CREATE INDEX index_inbox_conversations_on_is_spam ON inbox_conversations (is_spam);
CREATE INDEX index_inbox_conversations_on_trashed_at ON inbox_conversations (trashed_at);
CREATE UNIQUE INDEX index_inbox_conversations_on_anonymous_id ON inbox_conversations (anonymous_id);



CREATE TABLE inbox_messages (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  received_at TIMESTAMP WITH TIME ZONE NOT NULL,
  body_html TEXT NOT NULL,
  from_operator BOOLEAN NOT NULL,

  conversation_id UUID NOT NULL REFERENCES inbox_conversations (id) ON DELETE CASCADE,
  author_id UUID REFERENCES kernel_users (id)
);
CREATE INDEX index_inbox_messages_on_conversation_id ON inbox_messages (conversation_id);


CREATE TABLE inbox_contacts (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  name TEXT NOT NULL,
  birthday TIMESTAMP WITH TIME ZONE,
  email TEXT NOT NULL,
  pgp_key TEXT NOT NULL,
  phone TEXT NOT NULL,
  address TEXT NOT NULL,
  website TEXT NOT NULL,
  twitter TEXT NOT NULL,
  instagram TEXT NOT NULL,
  facebook TEXT NOT NULL,
  linkedin TEXT NOT NULL,
  skype TEXT NOT NULL,
  telegram TEXT NOT NULL,
  bloom TEXT NOT NULL,
  notes TEXT NOT NULL,
  country TEXT NOT NULL,
  country_code TEXT NOT NULL,
  plan TEXT NOT NULL,
  user_id TEXT NOT NULL,
  avatar_storage_key TEXT,

  namespace_id UUID NOT NULL REFERENCES kernel_namespaces (id) ON DELETE CASCADE
);
CREATE INDEX index_inbox_contacts_on_namespace_id ON inbox_contacts (namespace_id);
CREATE INDEX index_inbox_contacts_on_email ON inbox_contacts (email);
CREATE INDEX index_inbox_contacts_on_name ON inbox_contacts (name);


CREATE TABLE inbox_chatbox_preferences (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  name TEXT NOT NULL,
  color TEXT NOT NULL,
  avatar_storage_key TEXT,
  show_branding BOOLEAN NOT NULL,
  welcome_message TEXT NOT NULL,

  namespace_id UUID NOT NULL REFERENCES kernel_namespaces (id) ON DELETE CASCADE
);
CREATE INDEX index_inbox_chatbox_preferences_on_namespace_id ON inbox_chatbox_preferences (namespace_id);


CREATE TABLE inbox_conversations_contacts (
  contact_id UUID NOT NULL REFERENCES inbox_contacts (id) ON DELETE CASCADE,
  conversation_id UUID NOT NULL REFERENCES inbox_conversations (id) ON DELETE CASCADE,

  PRIMARY KEY (contact_id, conversation_id)
);
CREATE INDEX index_inbox_conversations_contacts_on_contact_id ON inbox_conversations_contacts (contact_id);
CREATE INDEX index_inbox_conversations_contacts_on_conversation_id ON inbox_conversations_contacts (conversation_id);


CREATE TABLE inbox_contacts_anonymous_ids (
  anonymous_id UUID PRIMARY KEY,
  contact_id UUID NOT NULL REFERENCES inbox_contacts (id) ON DELETE CASCADE
);


-- #################################################################################################
-- Newsletter
-- #################################################################################################
CREATE TABLE newsletter_lists (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  name TEXT NOT NULL,
  description TEXT NOT NULL,

  namespace_id UUID NOT NULL REFERENCES kernel_namespaces (id) ON DELETE CASCADE
);
CREATE INDEX index_newsletter_lists_on_namespace_id ON newsletter_lists (namespace_id);


CREATE TABLE newsletter_lists_subscriptions (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  list_id UUID NOT NULL REFERENCES newsletter_lists (id) ON DELETE CASCADE,
  contact_id UUID NOT NULL REFERENCES inbox_contacts (id) ON DELETE CASCADE
);
CREATE INDEX index_newsletter_lists_subscriptions_on_list_id ON newsletter_lists_subscriptions (list_id);
CREATE INDEX index_newsletter_lists_subscriptions_on_contact_id ON newsletter_lists_subscriptions (contact_id);


CREATE TABLE newsletter_messages (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

  name TEXT NOT NULL,
  subject TEXT NOT NULL,
  body TEXT NOT NULL,
  body_html TEXT NOT NULL,
  status TEXT NOT NULL,
  scheduled_for TIMESTAMP WITH TIME ZONE,
  last_sent_at TIMESTAMP WITH TIME ZONE,
  sent_count BIGINT NOT NULL,
  error_count BIGINT NOT NULL,

  namespace_id UUID NOT NULL REFERENCES kernel_namespaces (id) ON DELETE CASCADE,
  list_id UUID NOT NULL REFERENCES newsletter_lists (id) ON DELETE CASCADE
);
CREATE INDEX index_newsletter_messages_on_list_id ON newsletter_messages (list_id);
CREATE INDEX index_newsletter_messages_on_namespace_id ON newsletter_messages (namespace_id);
