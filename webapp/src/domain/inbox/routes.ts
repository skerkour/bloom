/* eslint-disable */
const prefix = 'inbox';
const commands = 'commands';
const queries = 'queries';

export const Commands = {
  sendMessage: `/${prefix}/${queries}/send_message`,
}

export const Queries = {
  archive: `/${prefix}/${queries}/archive`,
  trash: `/${prefix}/${queries}/trash`,
  inbox: `/${prefix}/${queries}/inbox`,
  spam: `/${prefix}/${queries}/spam`,
  chatboxPreferences: `/${prefix}/${queries}/chatbox_preferences`,
}
