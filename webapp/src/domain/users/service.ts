import ApiClient from '@/api/client';
import { AppState } from '@/app/store';
import { Store } from 'vuex';
import {
  Group,
  User, VerifyEmailInput,
} from '@/api/graphql/model';
import Router from '@/app/router';

export class UsersService {
  private apiClient: ApiClient;
  private store: Store<AppState>;
  private router: Router;

  constructor(apiClient: ApiClient, store: Store<AppState>, router: Router) {
    this.apiClient = apiClient;
    this.store = store;
    this.router = router;
  }

  async fetchMyGroupsWithProjects(): Promise<Group[]> {
    const query = `
      query {
        me {
          groups {
            id
            createdAt
            path
            name
            description
            avatarUrl

            projects {
              id
              path
              name
              createdAt
              avatarUrl
              description
            }
          }
        }
      }
    `;
    const variables = {};

    const res: { me: User } = await this.apiClient.query(query, variables);
    return res.me.groups;
  }

  async verifyEmail(input: VerifyEmailInput): Promise<void> {
    const query = `
      mutation($input: VerifyEmailInput!) {
        verifyEmail(input: $input)
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
  }
}


export const UsersServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: UsersService) {
    Vue.prototype.$usersService = service;
  },
};
