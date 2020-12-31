import { RouteConfig } from 'vue-router';
import Login from './login.vue';
import Complete from './complete.vue';
import TwoFA from './two_fa.vue';


const routes: Array<RouteConfig> = [
  {
    path: '/login',
    component: Login,
    meta: {
      layout: 'auth',
      auth: false,
    },
  },

  {
    path: '/login/complete',
    component: Complete,
    meta: {
      layout: 'auth',
      auth: false,
    },
  },

  {
    path: '/login/2fa',
    component: TwoFA,
    meta: {
      layout: 'auth',
      auth: false,
    },
  },

  { path: '/log-in', redirect: '/login' },
  { path: '/log_in', redirect: '/login' },
  { path: '/sign-in', redirect: '/login' },
  { path: '/signin', redirect: '/login' },
  { path: '/sign_in', redirect: '/login' },
];

export default routes;
