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
import { NamespacesService, NamespacesServiceInjector } from './domain/namespaces/service';
import { OperationsService, OperationsServiceInjector } from './domain/operations/service';
import { ToolsService, ToolsServiceInjector } from './domain/tools/service';
import { KernelService, KernelServiceInjector } from './domain/kernel/service';
import { InboxService, InboxServiceInjector } from './domain/inbox/service';
import { AnalyticsService, AnalyticsServiceInjector } from './domain/analytics/service';
import { FilesService, FilesServiceInjector } from './domain/files/service';
import { NewsletterService, NewsletterServiceInjector } from './domain/newsletter/service';
import { CalendarService, CalendarServiceInjector } from './domain/calendar/service';

const config = new Config();
const storage = new Storage();
const store = newStore(storage, vuetify);
const router = new Router(config, routes, store);
const apiClient = new ApiClient(config, store, router);

const kernelService = new KernelService(apiClient, store, router);
const analyticsService = new AnalyticsService(apiClient, store);
const inboxService = new InboxService(apiClient, store, router);
const usersService = new UsersService(apiClient, store, router);
const namespacesService = new NamespacesService(apiClient, store);
const operationsService = new OperationsService(apiClient, router);
const toolsService = new ToolsService(apiClient);
const filesService = new FilesService(apiClient, store);
const newsletterService = new NewsletterService(apiClient, store, router);
const calendarService = new CalendarService(apiClient, store);

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
    autoSessionTracking: false,
  });
}

Vue.use(VueRouter);
Vue.use(ConfigServiceInjector, config);
Vue.use(UsersServiceInjector, usersService);
Vue.use(InboxServiceInjector, inboxService);
Vue.use(AnalyticsServiceInjector, analyticsService);
Vue.use(KernelServiceInjector, kernelService);
Vue.use(NamespacesServiceInjector, namespacesService);
Vue.use(OperationsServiceInjector, operationsService);
Vue.use(ToolsServiceInjector, toolsService);
Vue.use(FilesServiceInjector, filesService);
Vue.use(NewsletterServiceInjector, newsletterService);
Vue.use(CalendarServiceInjector, calendarService);


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
