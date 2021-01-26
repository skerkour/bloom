import { RouteConfig } from 'vue-router';

const Inbox = () => import(/* webpackChunkName: "chunk-inbox" */ './inbox.vue');
const Chatbox = () => import(/* webpackChunkName: "chunk-inbox" */ './chatbox.vue');

const Contacts = () => import(/* webpackChunkName: "chunk-inbox" */ './contacts/contacts.vue');
const NewContact = () => import(/* webpackChunkName: "chunk-inbox" */ './contacts/new.vue');
const Contact = () => import(/* webpackChunkName: "chunk-inbox" */ './contacts/contact.vue');

const NewsletterMessages = () => import(/* webpackChunkName: "chunk-inbox" */ './newsletter/messages/messages.vue');
const NewsletterMessage = () => import(/* webpackChunkName: "chunk-inbox" */ './newsletter/messages/message.vue');
const NewNewsletterMessage = () => import(/* webpackChunkName: "chunk-inbox" */ './newsletter/messages/new.vue');

const NewsletterLists = () => import(/* webpackChunkName: "chunk-inbox" */ './newsletter/lists/lists.vue');
const NewsletterList = () => import(/* webpackChunkName: "chunk-inbox" */ './newsletter/lists/list.vue');
const NewNewsletterList = () => import(/* webpackChunkName: "chunk-inbox" */ './newsletter/lists/new.vue');


const routes: Array<RouteConfig> = [
  {
    path: '/inbox',
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


  {
    path: '/inbox/newsletter/messages',
    component: NewsletterMessages,
  },
  {
    path: '/inbox/newsletter/messages/new',
    component: NewNewsletterMessage,
  },
  {
    path: '/inbox/newsletter/messages/:messageId',
    component: NewsletterMessage,
  },

  {
    path: '/inbox/newsletter/lists',
    component: NewsletterLists,
  },
  {
    path: '/inbox/newsletter/lists/new',
    component: NewNewsletterList,
  },
  {
    path: '/inbox/newsletter/lists/:listId',
    component: NewsletterList,
  },

  { path: '/inbox*', redirect: '/inbox' },
];

export default routes;
