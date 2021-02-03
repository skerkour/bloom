import { RouteConfig } from 'vue-router';

const Sessions = () => import(/* webpackChunkName: "chunk-users" */ './sessions.vue');
const Profile = () => import(/* webpackChunkName: "chunk-users" */ './profile.vue');
const Invitations = () => import(/* webpackChunkName: "chunk-users" */ './invitations.vue');
const Billing = () => import(/* webpackChunkName: "chunk-users" */ './billing/billing.vue');
const BillingSync = () => import(/* webpackChunkName: "chunk-users" */ './billing/sync.vue');
const BillingPortal = () => import(/* webpackChunkName: "chunk-users" */ './billing/portal.vue');

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
    path: '/preferences/billing/sync',
    component: BillingSync,
  },
  {
    path: '/preferences/billing/portal',
    component: BillingPortal,
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
