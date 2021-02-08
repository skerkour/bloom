/* eslint-disable */
const prefix = 'inbox';
const commands = 'commands';
const queries = 'queries';

export const Commands = {
  sendChatboxMessages: `/${prefix}/${commands}/send_chatbox_message`,
  linkContact: `/${prefix}/${commands}/link_chatbox_contact`,
}

export const Queries = {
  chatboxPreferences: `/${prefix}/${queries}/chatbox_preferences`,
  chatboxMessages: `/${prefix}/${queries}/chatbox_messages`
}
