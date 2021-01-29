import { RouteConfig } from 'vue-router';

const Sessions = () => import(/* webpackChunkName: "chunk-users" */ './sessions.vue');
const Profile = () => import(/* webpackChunkName: "chunk-users" */ './profile.vue');
const Invitations = () => import(/* webpackChunkName: "chunk-users" */ './invitations.vue');
const Billing = () => import(/* webpackChunkName: "chunk-users" */ './billing.vue');

const routes: Array<RouteConfig> = [
  {
    path: '/preferences/profile',
    component: Profile,
  },
  {
    path: '/preferences/billing',
    component: Billing,
  },
  {
    path: '/preferences/invitations',
    component: Invitations,
  },
  {
    path: '/preferences/sessions',
    component: Sessions,
  },

  { path: '/preferences*', redirect: '/preferences/profile' },
];

export default routes;
