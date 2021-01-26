import { RouteConfig } from 'vue-router';

const Files = () => import(/* webpackChunkName: "chunk-files" */ './files.vue');
const Trash = () => import(/* webpackChunkName: "chunk-files" */ './trash.vue');

const routes: Array<RouteConfig> = [
  {
    path: '/files',
    component: Files,
  },
  {
    path: '/files/trash',
    component: Trash,
  },
  {
    path: '/files/:fileId',
    component: Files,
  },

  {
    path: '/files/*',
    redirect: '/files',
  },
];

export default routes;
