import ApiClient from '@/api/client';
import {
  CreateGroupInput,
  Group,
  DeleteGroupInput, CustomerPortalUrlInput, UpdateBillingInformationInput, CheckoutSessionInput,
  SyncBillingWithProviderInput, User, UpdateGroupProfileInput, RemoveMemberFromGroupInput,
  CancelGroupInvitationInput,
  QuitGroupInput,
  InvitePeopleInGroupInput,
} from '@/api/graphql/model';
import { Config } from '@/app/config';
import Router from '@/app/router';
import { loadStripe } from '@stripe/stripe-js';

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
  private router: Router;
  private config: Config;

  constructor(apiClient: ApiClient, router: Router, config: Config) {
    this.apiClient = apiClient;
    this.router = router;
    this.config = config;
  }

  async createGroup(input: CreateGroupInput): Promise<void> {
    const query = `
      mutation($input: CreateGroupInput!) {
        createGroup(input: $input) {
          path
        }
      }
    `;
    const variables = { input };

    const res: { createGroup: Group } = await this.apiClient.query(query, variables);
    this.router.push({ path: `/${res.createGroup.path}` });
  }

  async deleteGroup(groupPath: string) {
    const query = `
      mutation($input: DeleteGroupInput!) {
        deleteGroup(input: $input)
      }
    `;
    const input: DeleteGroupInput = {
      fullPath: groupPath,
    };
    const variables = { input };

    await this.apiClient.query(query, variables);
    this.router.push({ path: '/' });
  }

  async fetchGroup(fullPath: string): Promise<Group> {
    const query = `
      query($fullPath: String!) {
        group(fullPath: $fullPath) {
          id
          createdAt
          name
          path
          avatarUrl
          description
        }
      }
    `;
    const variables = { fullPath };

    const res: { group: Group } = await this.apiClient.query(query, variables);
    return res.group;
  }

  async fetchGroupMembers(fullPath: string): Promise<Group> {
    const query = `
      query($fullPath: String!) {
        group(fullPath: $fullPath) {
          id
          createdAt
          name
          path
          avatarUrl
          description

          members {
            username
            name
            avatarUrl
            role
          }
          invitations {
            id
            inviter {
              username
              avatarUrl
              name
            }
            invitee {
              username
              avatarUrl
              name
            }
          }
        }
      }
    `;
    const variables = { fullPath };

    const res: { group: Group } = await this.apiClient.query(query, variables);
    return res.group;
  }

  async updateGroupProfile(input: UpdateGroupProfileInput): Promise<Group> {
    const query = `
      mutation($input: UpdateGroupProfileInput!) {
        updateGroupProfile(input: $input) {
          id
          createdAt
          name
          path
          avatarUrl
          description
        }
      }
    `;
    const variables = { input };

    const res: { updateGroupProfile: Group } = await this.apiClient.query(query, variables);
    return res.updateGroupProfile;
  }

  async fetchGroupBilling(fullPath: string): Promise<GroupeAndMe> {
    const query = `
      query($fullPath: String!) {
        me {
          id
          username
          email
        }
        group(fullPath: $fullPath) {
          id
          createdAt
          name
          path
          avatarUrl

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
        }
      }
    `;
    const variables = { fullPath };

    const res: GroupeAndMe = await this.apiClient.query(query, variables);
    return res;
  }

  async removeMemberFromGroup(input: RemoveMemberFromGroupInput): Promise<void> {
    const query = `
      mutation($input: RemoveMemberFromGroupInput!) {
        removeMemberFromGroup(input: $input) {
          id
        }
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
  }

  async cancelInvitation(input: CancelGroupInvitationInput): Promise<void> {
    const query = `
      mutation($input: CancelGroupInvitationInput!) {
        cancelGroupInvitation(input: $input)
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
  }

  async quitGroup(input: QuitGroupInput): Promise<void> {
    const query = `
      mutation($input: QuitGroupInput!) {
        quitGroup(input: $input)
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
    this.router.push({ path: '/' });
  }

  async invitePeopleInGroup(input: InvitePeopleInGroupInput): Promise<Group> {
    const query = `
      mutation($input: InvitePeopleInGroupInput!) {
        invitePeopleInGroup(input: $input) {
          id
          createdAt
          name
          path
          avatarUrl
          description

          members {
            username
            name
            avatarUrl
            role
          }
          invitations {
            id
            inviter {
              username
              avatarUrl
              name
            }
            invitee {
              username
              avatarUrl
              name
            }
          }
        }
      }
    `;
    const variables = { input };

    const res: { invitePeopleInGroup: Group } = await this.apiClient.query(query, variables);
    return res.invitePeopleInGroup;
  }

  async gotoBillingPortal(namespace: string) {
    const query = `
      query($input: CustomerPortalUrlInput!) {
        customerPortalUrl(input: $input)
      }
    `;
    const input: CustomerPortalUrlInput = {
      namespace,
    };
    const variables = { input };

    const res: { customerPortalUrl: string } = await this.apiClient.query(query, variables);
    window.location.href = res.customerPortalUrl;
  }

  // eslint-disable-next-line max-len
  async updateBillingInformation(input: UpdateBillingInformationInput): Promise<BillingInformation> {
    const query = `
      mutation($input: UpdateBillingInformationInput!) {
        updateBillingInformation(input: $input) {
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
      }
    `;
    const variables = { input };

    // eslint-disable-next-line max-len
    const res: { updateBillingInformation: BillingInformation } = await this.apiClient.query(query, variables);
    return res.updateBillingInformation;
  }

  async gotoCheckoutSession(input: CheckoutSessionInput): Promise<void> {
    const query = `
      query($input: CheckoutSessionInput!) {
        stripePublicKey
        checkoutSession(input: $input)
      }
    `;
    const variables = { input };

    // eslint-disable-next-line max-len
    const res: { checkoutSession: string, stripePublicKey: string } = await this.apiClient.query(query, variables);
    const stripe = await loadStripe(res.stripePublicKey);
    stripe?.redirectToCheckout({ sessionId: res.checkoutSession });
  }

  async syncBillingWithProvider(input: SyncBillingWithProviderInput): Promise<void> {
    const query = `
      mutation($input: SyncBillingWithProviderInput!) {
        syncBillingWithProvider(input: $input)
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
    this.router.push({ path: `/groups/${input.namespace}/-/billing` });
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
