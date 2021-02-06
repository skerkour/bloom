import { RouteConfig } from 'vue-router';

const NewProject = () => import(/* webpackChunkName: "chunk-projects" */ './new.vue');

const routes: Array<RouteConfig> = [
  {
    path: '/projects/new',
    component: NewProject,
  },
];

export default routes;
