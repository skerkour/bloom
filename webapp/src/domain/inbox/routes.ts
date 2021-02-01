/* eslint-disable */
const prefix = 'inbox';
const commands = 'commands';
const queries = 'queries';

export const Commands = {
  createContact: `/${prefix}/${commands}/create_contact`,
  deleteContact: `/${prefix}/${commands}/delete_contact`,
  importContacts: `/${prefix}/${commands}/import_contacts`,
  sendMessage: `/${prefix}/${commands}/send_message`,
  subscribeToList: `/${prefix}/${queries}/subscribe_to_list`,
  unsubscribeFromList: `/${prefix}/${queries}/unsubscribe_from_list`,
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
