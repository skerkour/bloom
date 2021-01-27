import { RouteConfig } from 'vue-router';
import Namespace from './namespace.vue';

const routes: Array<RouteConfig> = [
  {
    path: '/:namespacePath',
    component: Namespace,
  },
];

export default routes;
