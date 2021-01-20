import ApiClient from '@/api/client';
import { AppState, Mutation } from '@/app/store';
import { Store } from 'vuex';
import {
  SignInInput, SignInStarted, CompleteRegistrationInput,
  CompleteSignInInput,
  SignedIn,
  RevokeSessionInput,
  Group,
  User, UpdateMyProfileInput, VerifyEmailInput, AcceptGroupInvitationInput,
  DeclineGroupInvitationInput,
  CompleteTwoFaInput,
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

  async signIn(input: SignInInput): Promise<void> {
    const query = `
      mutation($input: SignInInput!) {
        signIn(input: $input) {
          pendingSessionId
        }
      }
    `;
    const variables = { input };

    const res: { signIn: SignInStarted } = await this.apiClient.query(query, variables);
    this.store.commit(Mutation.SET_PENDING_SESSION_ID, res.signIn.pendingSessionId);
    this.router.push({ path: '/login/complete' });
  }

  async completeRegistration(input: CompleteRegistrationInput): Promise<void> {
    const query = `
      mutation($input: CompleteRegistrationInput!) {
        completeRegistration(input: $input) {
          me {
            id
            createdAt
            username
            email
            name
            description
            isAdmin
            avatarUrl
          }
          session {
            id
            createdAt
            token
          }
        }
      }
    `;
    const variables = { input };

    const res: { completeRegistration: SignedIn } = await this.apiClient.query(query, variables);
    this.store.commit(Mutation.SIGN_IN, res.completeRegistration);
    this.router.push({ path: '/' });
  }

  async completeSignIn(input: CompleteSignInInput): Promise<void> {
    const query = `
      mutation($input: CompleteSignInInput!) {
        completeSignIn(input: $input) {
          twoFaMethod
          me {
            id
            createdAt
            username
            email
            name
            description
            isAdmin
            avatarUrl
          }
          session {
            id
            createdAt
            token
          }
        }
      }
    `;
    const variables = { input };

    const res: { completeSignIn: SignedIn } = await this.apiClient.query(query, variables);

    // if 2fa is enabled
    if (res.completeSignIn.twoFaMethod) {
      this.router.push({ path: '/login/2fa' });
      return;
    }

    // otherwise, complete sign-in flow
    this.store.commit(Mutation.SIGN_IN, res.completeSignIn);
    this.router.push({ path: '/' });
  }

  async completeTwoFA(input: CompleteTwoFaInput): Promise<void> {
    const query = `
      mutation($input: CompleteTwoFAInput!) {
        completeTwoFA(input: $input) {
          me {
            id
            createdAt
            username
            email
            name
            description
            isAdmin
            avatarUrl
          }
          session {
            id
            createdAt
            token
          }
        }
      }
    `;
    const variables = { input };

    const res: { completeTwoFA: SignedIn } = await this.apiClient.query(query, variables);

    // complete sign-in flow
    this.store.commit(Mutation.SIGN_IN, res.completeTwoFA);
    this.router.push({ path: '/' });
  }


  async revokeSession(sessionId: string): Promise<void> {
    const input: RevokeSessionInput = { sessionId };
    const query = `
      mutation($input: RevokeSessionInput!) {
        revokeSession(input: $input)
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
    if (this.store.state.session?.id === sessionId) {
      this.store.commit(Mutation.SIGN_OUT);
      this.router.push({ path: '/' });
    }
  }

  async fetchMyGroups(): Promise<Group[]> {
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
          }
        }
      }
    `;
    const variables = {};

    const res: { me: User } = await this.apiClient.query(query, variables);
    return res.me.groups;
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

  async fetchMe(): Promise<User> {
    const query = `
      query {
        me {
          id
          name
          description
          username
          email
          avatarUrl
          twoFAEnabled
        }
      }
    `;
    const variables = {};

    const res: { me: User } = await this.apiClient.query(query, variables);
    return res.me;
  }

  async updateMyProfile(input: UpdateMyProfileInput): Promise<User> {
    const query = `
      mutation($input: UpdateMyProfileInput!) {
        updateMyProfile(input: $input) {
          id
          name
          description
          username
          email
          avatarUrl
          twoFAEnabled
        }
      }
    `;
    const variables = { input };

    const res: { updateMyProfile: User } = await this.apiClient.query(query, variables);
    this.store.commit(Mutation.UPDATE_MY_PROFILE, res.updateMyProfile);
    return res.updateMyProfile;
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

  async fetchMySessions(): Promise<User> {
    const query = `
      query {
        me {
          id
          name
          description
          username
          email
          avatarUrl

          sessions {
            id
            createdAt
          }
        }
      }
    `;
    const variables = {};

    const res: { me: User } = await this.apiClient.query(query, variables);
    return res.me;
  }

  async fetchMyGroupInvitations(): Promise<User> {
    const query = `
      query {
        me {
          id
          name
          description
          username
          email
          avatarUrl

          invitations {
            id

            group {
              name
              avatarUrl
            }
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
    const variables = {};

    const res: { me: User } = await this.apiClient.query(query, variables);
    return res.me;
  }

  async acceptInvitation(input: AcceptGroupInvitationInput): Promise<void> {
    const query = `
      mutation($input: AcceptGroupInvitationInput!) {
        acceptGroupInvitation(input: $input) {
          id
        }
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
  }

  async declineInvitation(input: DeclineGroupInvitationInput): Promise<void> {
    const query = `
      mutation($input: DeclineGroupInvitationInput!) {
        declineGroupInvitation(input: $input)
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
  }

  async adminFetchAllUsers(): Promise<User[]> {
    const query = `
      query {
        adminUsers {
          id
          name
          description
          username
          email
          isAdmin
          disabledAt
          avatarUrl
        }
      }
    `;
    const variables = {};

    const res: { adminUsers: User[] } = await this.apiClient.query(query, variables);
    return res.adminUsers;
  }

  async adminFetchUser(username: string): Promise<User> {
    const query = `
      query($username: String!) {
        adminUser(username: $username) {
          id
          createdAt
          name
          path
          avatarUrl
          description
          username
          email
          isAdmin
          disabledAt

          groups {
            id
            createdAt
            name
            path
          }
        }
      }
    `;
    const variables = { username };

    const res: { adminUser: User } = await this.apiClient.query(query, variables);
    return res.adminUser;
  }

  async adminDisableUser(username: string): Promise<void> {
    const query = `
      mutation($username: String!) {
        adminDisableUser(username: $username)
      }
    `;
    const variables = { username };

    await this.apiClient.query(query, variables);
  }

  async adminEnableUser(username: string): Promise<void> {
    const query = `
      mutation($username: String!) {
        adminEnableUser(username: $username)
      }
    `;
    const variables = { username };

    await this.apiClient.query(query, variables);
  }
}


export const UsersServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: UsersService) {
    Vue.prototype.$usersService = service;
  },
};
