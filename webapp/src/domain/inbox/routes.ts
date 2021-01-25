/* eslint-disable */
const prefix = 'inbox';
const commands = 'commands';
const queries = 'queries';

export const Commands = {
  sendMessage: `/${prefix}/${queries}/send_message`,
}

export const Queries = {
  inbox: `/${prefix}/${queries}/inbox`,
}
