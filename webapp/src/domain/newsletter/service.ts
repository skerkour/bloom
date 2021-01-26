
/* eslint-disable class-methods-use-this */
import ApiClient from '@/api/client';
import { AppState } from '@/app/store';
import { Store } from 'vuex';
import { GetList, GetLists, List } from './model';
import { Queries } from './routes';

export class NewsletterService {
  private apiClient: ApiClient;
  private store: Store<AppState>;

  constructor(apiClient: ApiClient, store: Store<AppState>) {
    this.apiClient = apiClient;
    this.store = store;
  }

  async fetchList(input: GetList): Promise<List> {
    const res: List = await this.apiClient.post(Queries.list, input);

    return res;
  }

  async fetchLists(): Promise<List[]> {
    const input: GetLists = {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      namespace_id: this.store.state.currentNamespaceId!,
    };
    const res: List[] = await this.apiClient.post(Queries.lists, input);

    return res;
  }
}

export const NewsletterServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: NewsletterService) {
    Vue.prototype.$newsletterService = service;
  },
};
