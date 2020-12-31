import VueRouter, { RouteConfig } from 'vue-router';
import { Store } from 'vuex';
import { Config } from './config';
import { AppState } from './store';


export default class Router extends VueRouter {
  private store: Store<AppState>;

  constructor(config: Config, routes: RouteConfig[], store: Store<AppState>) {
    super({
      mode: 'history',
      base: config.baseURL,
      routes,
      scrollBehavior() {
        return { x: 0, y: 0 };
      },
    });

    this.store = store;

    this.beforeEach((to, _, next) => {
      if (to.meta.auth !== false) {
        if (this.store.state.session === null) {
          next({ path: '/login' });
        } else {
          next();
        }
      } else {
        next();
      }
    });
  }
}
