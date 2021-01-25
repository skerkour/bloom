import Vue from 'vue';
import VueRouter from 'vue-router';
import * as Sentry from '@sentry/browser';
import { Vue as VueIntegration } from '@sentry/integrations';
import { ConfigServiceInjector, Config } from '@/app/config';
import Router from '@/app/router';
import { Storage } from '@/app/storage';
// import registerServiceWorker from '@/app/service_worker';
import App from '@/ui/app.vue';
import routes from '@/ui/views/routes';
import { Mutation, newStore } from '@/app/store';
import vuetify from '@/plugins/vuetify';
import Vuetify from 'vuetify/lib';
import { UsersService, UsersServiceInjector } from '@/domain/users/service';
import ApiClient from '@/api/client';
import { GroupsService, GroupsServiceInjector } from './domain/groups/service';
import { ProjectsService, ProjectsServiceInjector } from './domain/projects/service';
import { NamespacesService, NamespacesServiceInjector } from './domain/namespaces/service';
import { SupportService, SupportServiceInjector } from './domain/support/service';
import { CollaborationService, CollaborationServiceInjector } from './domain/collaboration/service';
import { GrowthService, GrowthServiceInjector } from './domain/growth/service';
import { OperationsService, OperationsServiceInjector } from './domain/operations/service';
import { ToolsService, ToolsServiceInjector } from './domain/tools/service';
import { KernelService, KernelServiceInjector } from './domain/kernel/service';
import { InboxService, InboxServiceInjector } from './domain/inbox/service';

const config = new Config();
const storage = new Storage();
const store = newStore(storage);
const router = new Router(config, routes, store);
const apiClient = new ApiClient(config, store, router);

const kernelService = new KernelService(config, apiClient, store, router);
const inboxService = new InboxService(apiClient, store);
const usersService = new UsersService(apiClient, store, router);
const groupsService = new GroupsService(apiClient, router, config);
const projectsService = new ProjectsService(apiClient, router, kernelService);
const namespacesService = new NamespacesService(apiClient, store);
const supportService = new SupportService(apiClient, store, router);
const collaborationService = new CollaborationService(apiClient, router);
const growthService = new GrowthService(apiClient, router);
const operationsService = new OperationsService(apiClient, router);
const toolsService = new ToolsService(apiClient);


if (config.env === 'production') {
  Vue.config.productionTip = false;
  Vue.config.devtools = false;
  Vuetify.config.silent = true;
} else {
  Vue.config.productionTip = true;
}


if (config.sentryDsn) {
  Sentry.init({
    dsn: config.sentryDsn,
    integrations: [new VueIntegration({ Vue, attachProps: false })],
    environment: config.env,
  });
}

Vue.use(VueRouter);
Vue.use(ConfigServiceInjector, config);
Vue.use(UsersServiceInjector, usersService);
Vue.use(InboxServiceInjector, inboxService);
Vue.use(KernelServiceInjector, kernelService);
Vue.use(GroupsServiceInjector, groupsService);
Vue.use(ProjectsServiceInjector, projectsService);
Vue.use(NamespacesServiceInjector, namespacesService);
Vue.use(SupportServiceInjector, supportService);
Vue.use(CollaborationServiceInjector, collaborationService);
Vue.use(GrowthServiceInjector, growthService);
Vue.use(OperationsServiceInjector, operationsService);
Vue.use(ToolsServiceInjector, toolsService);


async function main() {
  if (store.state.sessionToken !== null) {
    const me = await kernelService.fetchMe();

    store.commit(Mutation.INIT, me);
  }

  new Vue({
    router,
    store,
    vuetify,
    render: (h) => h(App),
  }).$mount('#app');
}

main();
