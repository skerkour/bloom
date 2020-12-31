import APIClient from '@/api/client';
import { Store } from 'vuex';
import { AppState, Mutation } from '@/app/store';

export class NamespacesService {
  private apiClient: APIClient;
  private store: Store<AppState>;

  constructor(apiClient: APIClient, store: Store<AppState>) {
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
    if (res.namespace.__typename === 'Group') {
      this.store.commit(Mutation.SET_NAMESPACE_IS_GROUP, true);
    }

    return res.namespace;
  }

  leaveNamespaceView() {
    this.store.commit(Mutation.SET_NAMESPACE_IS_GROUP, false);
  }
}

export const NamespacesServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: NamespacesService) {
    Vue.prototype.$namespacesService = service;
  },
};
