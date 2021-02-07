import { RouteConfig } from 'vue-router';

const Index = () => import(/* webpackChunkName: "chunk-admin" */ './index.vue');

const Groups = () => import(/* webpackChunkName: "chunk-admin" */ './groups/groups.vue');
const Group = () => import(/* webpackChunkName: "chunk-admin" */ './groups/group.vue');

const Users = () => import(/* webpackChunkName: "chunk-admin" */ './users/users.vue');
const User = () => import(/* webpackChunkName: "chunk-admin" */ './users/user.vue');


const routes: Array<RouteConfig> = [
  {
    path: '/admin',
    component: Index,
  },

  {
    path: '/admin/groups',
    component: Groups,
  },
  {
    path: '/admin/groups/:groupId',
    component: Group,
  },

  {
    path: '/admin/users',
    component: Users,
  },
  {
    path: '/admin/users/:userId',
    component: User,
  },

  { path: '/admin*', redirect: '/admin' },
];

export default routes;
