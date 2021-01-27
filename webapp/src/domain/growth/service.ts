import ApiClient from '@/api/client';
import {
  ConfirmListSubscriptionInput,
  UnsubscribeFromListInput,
} from '@/api/graphql/model';

export class GrowthService {
  private apiClient: ApiClient;

  constructor(apiClient: ApiClient) {
    this.apiClient = apiClient;
  }

  async confirmListSubscription(input: ConfirmListSubscriptionInput): Promise<void> {
    const query = `
    mutation($input: ConfirmListSubscriptionInput!) {
      confirmListSubscription(input: $input)
    }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
  }

  async unsubscribeFromList(input: UnsubscribeFromListInput): Promise<void> {
    const query = `
    mutation($input: UnsubscribeFromListInput!) {
      unsubscribeFromList(input: $input)
    }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
  }
}

export const GrowthServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: GrowthService) {
    Vue.prototype.$growthService = service;
  },
};
