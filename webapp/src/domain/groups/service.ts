import ApiClient from '@/api/client';
import {
  Group,
  CustomerPortalUrlInput, UpdateBillingInformationInput, CheckoutSessionInput,
  SyncBillingWithProviderInput, User,
} from '@/api/graphql/model';
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

  constructor(apiClient: ApiClient, router: Router) {
    this.apiClient = apiClient;
    this.router = router;
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
