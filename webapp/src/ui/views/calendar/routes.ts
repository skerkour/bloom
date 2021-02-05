import { RouteConfig } from 'vue-router';

const Calendar = () => import(/* webpackChunkName: "chunk-calendar" */ './calendar.vue');

const routes: Array<RouteConfig> = [
  {
    path: '/calendar',
    component: Calendar,
  },

  { path: '/calendar*', redirect: '/calendar' },
];

export default routes;
