import ApiClient from '@/api/client';
import { Store } from 'vuex';
import { AppState } from '@/app/store';

export class NamespacesService {
  private apiClient: ApiClient;
  private store: Store<AppState>;

  constructor(apiClient: ApiClient, store: Store<AppState>) {
    this.apiClient = apiClient;
    this.store = store;
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  async fetchNamespace(fullPath: string): Promise<any> {
    const query = `
      query($input: String!) {
        namespace(fullPath: $input) {
          __typename
          id
          createdAt
          name
          path
          avatarUrl
          description

          projects {
            id
            avatarUrl
            path
            name
            description
          }

          ... on User {
            username
            avatarUrl
            description
          }
        }
      }
    `;
    const variables = { input: fullPath };

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const res: { namespace: any } = await this.apiClient.query(query, variables);
    // eslint-disable-next-line no-underscore-dangle

    return res.namespace;
  }
}

export const NamespacesServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: NamespacesService) {
    Vue.prototype.$namespacesService = service;
  },
};
