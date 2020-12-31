import { RouteConfig, Route } from 'vue-router';

const NewGroup = () => import(/* webpackChunkName: "chunk-groups" */ './new.vue');
const Preferences = () => import(/* webpackChunkName: "chunk-groups" */ './group/preferences/preferences.vue');
const Members = () => import(/* webpackChunkName: "chunk-groups" */ './group/members.vue');
const Billing = () => import(/* webpackChunkName: "chunk-groups" */ './group/billing/billing.vue');
const BillingSync = () => import(/* webpackChunkName: "chunk-groups" */ './group/billing/sync.vue');
const BillingPortal = () => import(/* webpackChunkName: "chunk-groups" */ './group/billing/portal.vue');


const routes: Array<RouteConfig> = [
  {
    path: '/groups/new',
    component: NewGroup,
  },

  {
    path: '/groups/:groupPath/-/members',
    component: Members,
    meta: {
      groupDrawer: true,
    },
  },

  {
    path: '/groups/:groupPath/-/billing',
    component: Billing,
    meta: {
      groupDrawer: true,
    },
  },
  {
    path: '/groups/:groupPath/-/billing/sync',
    component: BillingSync,
    meta: {
      groupDrawer: true,
    },
  },
  {
    path: '/groups/:groupPath/-/billing/portal',
    component: BillingPortal,
    meta: {
      groupDrawer: true,
    },
  },

  {
    path: '/groups/:groupPath/-/preferences',
    component: Preferences,
    meta: {
      groupDrawer: true,
    },
  },

  {
    path: '/groups/:groupPath',
    redirect: (to: Route) => to.params.groupPath,
  },
];

export default routes;
