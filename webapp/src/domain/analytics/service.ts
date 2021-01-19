
/* eslint-disable class-methods-use-this */
import ApiClient from '@/api/client';

export class AnalyticsService {
  private apiClient: ApiClient;

  constructor(apiClient: ApiClient) {
    this.apiClient = apiClient;
  }
}

export const AnalyticsServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: AnalyticsService) {
    Vue.prototype.$botsService = service;
  },
};
