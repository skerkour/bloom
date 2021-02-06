import ApiClient from '@/api/client';
import {
  Group,
  User,
} from '@/api/graphql/model';

export interface BillingInformation {
  plan: string;
  name: string;
  email: string;
  country: string;
  countryCode: string;
  city: string;
  postalCode: string;
  addressLine1: string;
  addressLine2: string;
  state: string;
  taxId: string;
  usedStorage: number;
  totalStorage: number;
}

export type GroupeAndMe = {
  group: Group;
  me: User;
};

export class GroupsService {
  private apiClient: ApiClient;

  constructor(apiClient: ApiClient) {
    this.apiClient = apiClient;
  }

  async adminFetchAllGroups(): Promise<Group[]> {
    const query = `
      query {
        adminGroups {
          id
          createdAt
          name
          path
          avatarUrl
        }
      }
    `;
    const variables = {};

    const res: { adminGroups: Group[] } = await this.apiClient.query(query, variables);
    return res.adminGroups;
  }

  async adminFetchGroup(groupId: string): Promise<Group> {
    const query = `
      query($groupId: ID!) {
        adminGroup(groupId: $groupId) {
          id
          createdAt
          name
          path
          avatarUrl
          description

          projects {
            id
            createdAt
            avatarUrl
            path
            name
            description
          }

          billing {
            plan
            name
            email
            country
            countryCode
            city
            postalCode
            addressLine1
            addressLine2
            state
            taxId
            usedStorage
            totalStorage
          }

          members {
            username
            name
            avatarUrl
            role
          }
        }
      }
    `;
    const variables = { groupId };

    const res: { adminGroup: Group } = await this.apiClient.query(query, variables);
    return res.adminGroup;
  }
}

export const GroupsServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: GroupsService) {
    Vue.prototype.$groupsService = service;
  },
};
