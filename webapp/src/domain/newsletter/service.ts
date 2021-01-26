
/* eslint-disable class-methods-use-this */
import ApiClient from '@/api/client';
import { AppState } from '@/app/store';
import { Store } from 'vuex';

export class NewsletterService {
  private apiClient: ApiClient;
  private store: Store<AppState>;

  constructor(apiClient: ApiClient, store: Store<AppState>) {
    this.apiClient = apiClient;
    this.store = store;
  }
}

export const NewsletterServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: NewsletterService) {
    Vue.prototype.$newsletterService = service;
  },
};
