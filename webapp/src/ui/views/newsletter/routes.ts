import { RouteConfig } from 'vue-router';

const Message = () => import(/* webpackChunkName: "chunk-newsletter" */ './lists/messages/message.vue');
const NewMessage = () => import(/* webpackChunkName: "chunk-newsletter" */ './lists/messages/new.vue');

const Lists = () => import(/* webpackChunkName: "chunk-newsletter" */ './lists/lists.vue');
const List = () => import(/* webpackChunkName: "chunk-newsletter" */ './lists/list.vue');
const NewList = () => import(/* webpackChunkName: "chunk-newsletter" */ './lists/new.vue');


const routes: Array<RouteConfig> = [
  {
    path: '/newsletter',
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
  {
    path: '/newsletter/lists/:listId/messages/new',
    component: NewMessage,
  },
  {
    path: '/newsletter/lists/:listId/messages/:messageId',
    component: Message,
  },

  { path: '/newsletter*', redirect: '/newsletter' },
];

export default routes;
