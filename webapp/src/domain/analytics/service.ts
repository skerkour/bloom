
/* eslint-disable class-methods-use-this */
import ApiClient from '@/api/client';
import { AppState } from '@/app/store';
import { Store } from 'vuex';
import { Queries } from './routes';
import { Analytics, GetAnalytics } from './model';

export class AnalyticsService {
  private apiClient: ApiClient;
  private store: Store<AppState>;

  constructor(apiClient: ApiClient, store: Store<AppState>) {
    this.apiClient = apiClient;
    this.store = store;
  }


  async fetchAnalytics(): Promise<Analytics> {
    const input: GetAnalytics = {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      namespace_id: this.store.state.currentNamespace!.id,
    };
    const analytics: Analytics = await this.apiClient.post(Queries.analytics, input);

    return analytics;
  }
}

export const AnalyticsServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: AnalyticsService) {
    Vue.prototype.$analyticsService = service;
  },
};
