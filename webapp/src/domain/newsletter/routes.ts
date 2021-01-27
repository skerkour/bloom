/* eslint-disable */
const prefix = 'newsletter';
const commands = 'commands';
const queries = 'queries';

export const Commands = {
  createList: `/${prefix}/${commands}/create_list`,
  createMessage: `/${prefix}/${commands}/create_message`,
  deleteList: `/${prefix}/${commands}/delete_list`,
  deleteMessage: `/${prefix}/${commands}/delete_message`,
  sendMessage: `/${prefix}/${commands}/send_message`,
  sendTestMessage: `/${prefix}/${commands}/send_test_message`,
  updateList: `/${prefix}/${commands}/update_list`,
  updateMessage: `/${prefix}/${commands}/update_message`,
}

export const Queries = {
  list: `/${prefix}/${queries}/list`,
  lists: `/${prefix}/${queries}/lists`,
  message: `/${prefix}/${queries}/message`,
  messages: `/${prefix}/${queries}/messages`,
}
