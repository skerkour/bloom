/* eslint-disable */
const prefix = 'inbox';
const commands = 'commands';
const queries = 'queries';

export const Commands = {
  createContact: `/${prefix}/${commands}/create_contact`,
  deleteContact: `/${prefix}/${commands}/delete_contact`,
  importContacts: `/${prefix}/${commands}/import_contacts`,
  moveConversationToArchive: `/${prefix}/${commands}/move_conversation_to_archive`,
  moveConversationToInbox: `/${prefix}/${commands}/move_conversation_to_inbox`,
  moveConversationToSpam: `/${prefix}/${commands}/move_conversation_to_spam`,
  moveConversationToTrash: `/${prefix}/${commands}/move_conversation_to_trash`,
  sendMessage: `/${prefix}/${commands}/send_message`,
  updateChetboxPreferences:  `/${prefix}/${commands}/update_chatbox_preferences`,
  updateContact:  `/${prefix}/${commands}/update_contact`,
}

export const Queries = {
  archive: `/${prefix}/${queries}/archive`,
  chatboxPreferences: `/${prefix}/${queries}/chatbox_preferences`,
  contact: `/${prefix}/${queries}/contact`,
  contacts: `/${prefix}/${queries}/contacts`,
  inbox: `/${prefix}/${queries}/inbox`,
  spam: `/${prefix}/${queries}/spam`,
  trash: `/${prefix}/${queries}/trash`,
}
