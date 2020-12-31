import { RouteConfig } from 'vue-router';
import Namespace from './namespace.vue';
import ProjectRoutes from './project/routes';

const routes: Array<RouteConfig> = [
  {
    path: '/:namespacePath',
    component: Namespace,
  },

  ...ProjectRoutes,
];

export default routes;
