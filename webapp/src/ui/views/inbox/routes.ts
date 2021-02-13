import { RouteConfig } from 'vue-router';

const Inbox = () => import(/* webpackChunkName: "chunk-inbox" */ './inbox.vue');
const Chatbox = () => import(/* webpackChunkName: "chunk-inbox" */ './chatbox.vue');

const Contacts = () => import(/* webpackChunkName: "chunk-inbox" */ './contacts/contacts.vue');
const NewContact = () => import(/* webpackChunkName: "chunk-inbox" */ './contacts/new.vue');
const Contact = () => import(/* webpackChunkName: "chunk-inbox" */ './contacts/contact.vue');

const routes: Array<RouteConfig> = [
  {
    path: '/inbox',
    component: Inbox,
  },
  {
    path: '/inbox/done',
    component: Inbox,
  },
  {
    path: '/inbox/trash',
    component: Inbox,
  },
  {
    path: '/inbox/spam',
    component: Inbox,
  },
  {
    path: '/inbox/chatbox',
    component: Chatbox,
  },
  {
    path: '/inbox/contacts',
    component: Contacts,
  },
  {
    path: '/inbox/contacts/new',
    component: NewContact,
  },
  {
    path: '/inbox/contacts/:contactId',
    component: Contact,
  },

  { path: '/inbox*', redirect: '/inbox' },
];

export default routes;
