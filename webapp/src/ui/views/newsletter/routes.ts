import { RouteConfig } from 'vue-router';

const Messages = () => import(/* webpackChunkName: "chunk-newsletter" */ './messages/messages.vue');
const Message = () => import(/* webpackChunkName: "chunk-newsletter" */ './messages/message.vue');
const NewMessage = () => import(/* webpackChunkName: "chunk-newsletter" */ './messages/new.vue');

const Lists = () => import(/* webpackChunkName: "chunk-newsletter" */ './lists/lists.vue');
const List = () => import(/* webpackChunkName: "chunk-newsletter" */ './lists/list.vue');
const NewList = () => import(/* webpackChunkName: "chunk-newsletter" */ './lists/new.vue');


const routes: Array<RouteConfig> = [
  {
    path: '/newsletter/messages',
    component: Messages,
  },
  {
    path: '/newsletter/messages/new',
    component: NewMessage,
  },
  {
    path: '/newsletter/messages/:messageId',
    component: Message,
  },

  {
    path: '/newsletter/lists',
    component: Lists,
  },
  {
    path: '/newsletter/lists/new',
    component: NewList,
  },
  {
    path: '/newsletter/lists/:listId',
    component: List,
  },

  { path: '/newsletter*', redirect: '/newsletter/messages' },
];

export default routes;
