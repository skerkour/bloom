/* eslint-disable */
const prefix = 'calendar';
const commands = 'commands';
const queries = 'queries';

export const Commands = {
  createEvent: `/${prefix}/${commands}/create_event`,
  deleteEvent: `/${prefix}/${commands}/delete_event`,
  updateEvent: `/${prefix}/${commands}/update_event`,
}

export const Queries = {
  events: `/${prefix}/${queries}/events`,
}
