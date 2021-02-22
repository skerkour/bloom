ALTER TABLE kernel_users RENAME COLUMN avatar_storage_key TO avatar_id;
ALTER TABLE kernel_groups RENAME COLUMN avatar_storage_key TO avatar_id;
ALTER TABLE inbox_contacts RENAME COLUMN avatar_storage_key TO avatar_id;
ALTER TABLE inbox_chatbox_preferences RENAME COLUMN avatar_storage_key TO avatar_id;
