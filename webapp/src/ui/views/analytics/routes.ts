import { RouteConfig } from 'vue-router';

const Analytics = () => import(/* webpackChunkName: "chunk-analytics" */ './analytics.vue');

const routes: Array<RouteConfig> = [
  {
    path: '/analytics',
    component: Analytics,
  },
];

export default routes;
