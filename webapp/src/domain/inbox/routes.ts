/* eslint-disable */
const prefix = 'inbox';
const commands = 'commands';
const queries = 'queries';

export const Commands = {
  importContacts: `/${prefix}/${commands}/import_contacts`,
  sendMessage: `/${prefix}/${commands}/send_message`,
  updateChetboxPreferences:  `/${prefix}/${commands}/update_chatbox_preferences`,
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
