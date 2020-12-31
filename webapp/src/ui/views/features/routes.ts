import { RouteConfig } from 'vue-router';

const Features = () => import(/* webpackChunkName: "chunk-features" */ './features.vue');

const routes: Array<RouteConfig> = [
  {
    path: '/features',
    component: Features,
    meta: { auth: false },
  },

  {
    path: '/features/*',
    redirect: '/features',
    meta: { auth: false },
  },
];

export default routes;
