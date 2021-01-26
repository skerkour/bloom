import { RouteConfig } from 'vue-router';

const Analytics = () => import(/* webpackChunkName: "chunk-analytics" */ './analytics.vue');

const routes: Array<RouteConfig> = [
  {
    path: '/analytics',
    component: Analytics,
  },

  {
    path: '/analytics/*',
    redirect: '/analytics',
  },
];

export default routes;
