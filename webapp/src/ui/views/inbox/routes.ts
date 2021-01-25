import { RouteConfig } from 'vue-router';

const Inbox = () => import(/* webpackChunkName: "chunk-inbox" */ './inbox.vue');
const Chatbox = () => import(/* webpackChunkName: "chunk-inbox" */ './chatbox.vue');

const routes: Array<RouteConfig> = [
  {
    path: '/inbox',
    component: Inbox,
  },
  {
    path: '/inbox/chatbox',
    component: Chatbox,
  },
];

export default routes;
