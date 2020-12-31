import { RouteConfig } from 'vue-router';
import Register from './register.vue';
import CompleteRegistration from './complete.vue';


const routes: Array<RouteConfig> = [
  {
    path: '/register',
    component: Register,
    meta: {
      layout: 'auth',
      auth: false,
    },
  },

  {
    path: '/register/complete',
    component: CompleteRegistration,
    meta: {
      layout: 'auth',
      auth: false,
    },
  },

  { path: '/signup', redirect: '/register' },
  { path: '/sign-up', redirect: '/register' },
  { path: '/sign_up', redirect: '/register' },
  { path: '/registration', redirect: '/register' },
  { path: '/welcome', redirect: '/register' },
];

export default routes;
