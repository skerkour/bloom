/* eslint-disable camelcase */
/* eslint-disable no-underscore-dangle */
/* eslint-disable class-methods-use-this */
/* eslint-disable max-len */
import ApiClient from '@/api/client';
import {
  StatusPage,
} from '@/api/graphql/model';
import { AppState, Mutation } from '@/app/store';
import { Store } from 'vuex';
import Router from '@/app/router';
import { loadStripe } from '@stripe/stripe-js';
import { Admin, Commands, Queries } from './routes';
import {
  AcceptGroupInvitation,
  CancelGroupInvitation,
  CompleteRegistration, CompleteSignIn, CompleteTwoFaChallenge, CompleteTwoFaSetup, CreateGroup, DeclineGroupInvitation, DeleteMyAccount, DisableTwoFa, GenerateQrCode, GetSignedUploadUrl, GroupInvitation, Markdown, MarkdownHtml, Me, QrCode, Register, RegistrationStarted, RevokeSession, Session, SetupTwoFa, SignedIn, SignedUploadUrl, SignIn, SignInStarted, UpdateMyProfile,
  User, Group, GetGroup, UpdateGroupProfile, GroupWithMembersAndInvitations, RemoveMemberFromGroup, QuitGroup, InvitePeopleInGroup, DeleteGroup, Namespace, BillingInformation, GetBillingInformation, UpdateBillingInformation, SyncCustomerWithProvider, GetCheckoutSession, CheckoutSession, GetCustomerPortal, CustomerPortal, AdminBlockUser, AdminGroup, AdminUnblockUser, AdminUser,
} from './model';

export type StorageSignedUploadUrlInput = {
  size: number;
}

export class KernelService {
  private apiClient: ApiClient;
  private store: Store<AppState>;
  private router: Router;

  constructor(apiClient: ApiClient, store: Store<AppState>, router: Router) {
    this.apiClient = apiClient;
    this.store = store;
    this.router = router;
  }

  async acceptGroupInvitation(invitationId: string): Promise<void> {
    const input: AcceptGroupInvitation = {
      invitation_id: invitationId,
    };
    await this.apiClient.post(Commands.acceptGroupInvitation, input);
  }

  async adminBlockUser(userId: string): Promise<User> {
    const input: AdminBlockUser = {
      user_id: userId,
    };

    const res: User = await this.apiClient.post(Admin.blockUser, input);
    return res;
  }

  async adminFetchGroup(groupId: string): Promise<Group> {
    const input: AdminGroup = {
      group_id: groupId,
    };

    const res: Group = await this.apiClient.post(Admin.group, input);
    return res;
  }

  async adminFetchGroups(): Promise<Group[]> {
    const res: Group[] = await this.apiClient.post(Admin.groups, {});
    return res;
  }

  async adminUnblockUser(userId: string): Promise<User> {
    const input: AdminUnblockUser = {
      user_id: userId,
    };

    const res: User = await this.apiClient.post(Admin.unblockUser, input);
    return res;
  }

  async adminFetchUser(userId: string): Promise<User> {
    const input: AdminUser = {
      user_id: userId,
    };

    const res: User = await this.apiClient.post(Admin.user, input);
    return res;
  }

  async adminFetchUsers(): Promise<User[]> {
    const res: User[] = await this.apiClient.post(Admin.users, {});
    return res;
  }

  async cancelGroupInvitation(invitationId: string): Promise<void> {
    const input: CancelGroupInvitation = {
      invitation_id: invitationId,
    };
    await this.apiClient.post(Commands.cancelGroupInvitation, input);
  }

  async completeRegistration(input: CompleteRegistration): Promise<void> {
    const res: SignedIn = await this.apiClient.post(Commands.completeRegistration, input);

    this.store.commit(Mutation.SIGN_IN, res);
    const me = await this.fetchMe();
    this.store.commit(Mutation.INIT, me);

    window.location.href = '/';
  }

  async completeSignIn(input: CompleteSignIn): Promise<void> {
    const res: SignedIn = await this.apiClient.post(Commands.completeSignIn, input);

    // if 2fa is enabled
    if (res.two_fa_method) {
      this.router.push({ path: '/login/2fa' });
      return;
    }


    // otherwise, complete sign-in flow
    this.store.commit(Mutation.SIGN_IN, res);
    const me = await this.fetchMe();
    this.store.commit(Mutation.INIT, me);

    this.router.push({ path: '/' });
  }

  async completeTwoFaChallenge(input: CompleteTwoFaChallenge): Promise<void> {
    const res: SignedIn = await this.apiClient.post(Commands.completeTwoFaChallenge, input);

    // complete sign-in flow
    this.store.commit(Mutation.SIGN_IN, res);
    const me = await this.fetchMe();
    this.store.commit(Mutation.INIT, me);

    this.router.push({ path: '/' });
  }

  async completeTwoFaSetup(code: string): Promise<void> {
    const input: CompleteTwoFaSetup = {
      code,
    };
    await this.apiClient.post(Commands.completeTwoFaSetup, input);
  }

  async createGroup(input: CreateGroup): Promise<void> {
    const group: Group = await this.apiClient.post(Commands.createGroup, input);

    const namespace: Namespace = {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      id: group.namespace_id!,
      name: group.name,
      path: group.path,
      avatar_url: group.avatar_url,
    };
    this.store.commit(Mutation.ADD_NAMESPACE, namespace);
    this.store.commit(Mutation.SET_CURRENT_NAMESPACE, namespace);

    this.router.push({ path: `/groups/${group.path}` });
  }

  async declineGroupInvitation(invitationId: string): Promise<void> {
    const input: DeclineGroupInvitation = {
      invitation_id: invitationId,
    };
    await this.apiClient.post(Commands.declineGroupInvitation, input);
  }

  async deleteGroup(groupId: string, groupPath: string): Promise<void> {
    const input: DeleteGroup = {
      group_id: groupId,
    };
    await this.apiClient.post(Commands.deleteGroup, input);

    this.store.commit(Mutation.REMOVE_NAMESPACE, groupPath);

    this.router.push({ path: '/' });
  }

  async deleteMyAccount(twoFaCode: string | null): Promise<void> {
    const input: DeleteMyAccount = {
      two_fa_code: twoFaCode,
    };
    await this.apiClient.post(Commands.deleteMyAccount, input);

    this.store.commit(Mutation.SIGN_OUT);
    window.location.href = '/';
  }

  async disableTwoFa(code: string): Promise<void> {
    const input: DisableTwoFa = {
      code,
    };
    await this.apiClient.post(Commands.disableTwoFa, input);
  }

  async fetchBillingInformation(namespace_id: string): Promise<BillingInformation> {
    const input: GetBillingInformation = {
      namespace_id,
    };
    const res: BillingInformation = await this.apiClient.post(Queries.billingInformation, input);

    return res;
  }

  async fetchGroup(path: string): Promise<Group> {
    const input: GetGroup = {
      path,
    };
    const res: Group = await this.apiClient.post(Queries.group, input);

    return res;
  }

  async fetchGroupWithMembersAndInvitations(path: string): Promise<GroupWithMembersAndInvitations> {
    const input: GetGroup = {
      path,
    };
    const res: GroupWithMembersAndInvitations = await this.apiClient.post(Queries.groupWithMembersAndInvitations, input);

    return res;
  }

  async fetchMe(): Promise<Me> {
    const res: Me = await this.apiClient.post(Queries.me, {});

    return res;
  }

  async fetchMyGroupInvitations(): Promise<GroupInvitation[]> {
    const res: GroupInvitation[] = await this.apiClient.post(Queries.myGroupInvitations, {});

    return res;
  }

  async fetchMySessions(): Promise<Session[]> {
    const res: Session[] = await this.apiClient.post(Queries.mySessions, {});
    return res;
  }

  async generateQrCode(input: string): Promise<QrCode> {
    const apiInput: GenerateQrCode = {
      input,
    };
    const res: QrCode = await this.apiClient.post(Queries.generateQrCode, apiInput);

    return res;
  }

  async gotoCheckoutSession(input: GetCheckoutSession): Promise<void> {
    const res: CheckoutSession = await this.apiClient.post(Queries.checkoutSession, input);

    const stripe = await loadStripe(res.stripe_public_key);
    stripe?.redirectToCheckout({ sessionId: res.checkout_session_id });
  }

  async gotoCustomerPortal(namespaceId: string): Promise<void> {
    const input: GetCustomerPortal = {
      namespace_id: namespaceId,
    };
    const res: CustomerPortal = await this.apiClient.post(Queries.customerPortal, input);

    window.location.href = res.customer_portal_url;
  }

  async invitePeopleInGroup(input: InvitePeopleInGroup): Promise<GroupInvitation[]> {
    const res: GroupInvitation[] = await this.apiClient.post(Commands.invitePeopleInGroup, input);
    return res;
  }

  async markdown(markdown: string): Promise<string> {
    const input: Markdown = {
      markdown,
    };
    const res: MarkdownHtml = await this.apiClient.post(Queries.markdown, input);

    return res.html;
  }

  async quitGroup(groupId: string, groupPath: string): Promise<void> {
    const input: QuitGroup = {
      group_id: groupId,
    };
    await this.apiClient.post(Commands.quitGroup, input);

    this.store.commit(Mutation.REMOVE_NAMESPACE, groupPath);

    this.router.push({ path: '/' });
  }

  async register(input: Register): Promise<void> {
    const res: RegistrationStarted = await this.apiClient.post(Commands.register, input);

    this.store.commit(Mutation.SET_PENDING_USER_ID, res.pending_user_id);
    this.router.push({ path: '/register/complete' });
  }

  async removeMemberFromGroup(input: RemoveMemberFromGroup): Promise<void> {
    await this.apiClient.post(Commands.removeMemberFromGroup, input);
  }

  async revokeSession(sessionId: string): Promise<void> {
    const input: RevokeSession = {
      session_id: sessionId,
    };
    await this.apiClient.post(Commands.revokeSession, input);

    if (this.store.state.session?.id === sessionId) {
      this.store.commit(Mutation.SIGN_OUT);
      this.router.push({ path: '/' });
    }
  }

  async setupTwoFa(): Promise<string> {
    const res: SetupTwoFa = await this.apiClient.post(Commands.setupTwoFa, {});

    return res.base64_qr_code;
  }

  async signIn(input: SignIn): Promise<void> {
    const res: SignInStarted = await this.apiClient.post(Commands.signIn, input);

    this.store.commit(Mutation.SET_PENDING_SESSION_ID, res.pending_session_id);
    this.router.push({ path: '/login/complete' });
  }

  async signedUploadUrl(filesize: number): Promise<SignedUploadUrl> {
    const input: GetSignedUploadUrl = {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      namespace_id: this.store.state.currentNamespace!.id!,
      filesize,
    };
    const res: SignedUploadUrl = await this.apiClient.post(Queries.signedUploadUrl, input);

    return res;
  }

  async syncCustomerWithProvider(input: SyncCustomerWithProvider, returnUrl: string): Promise<void> {
    await this.apiClient.post(Commands.syncCustomerWithProvider, input);
    this.router.push({ path: returnUrl });
  }

  async updateBillingInformation(input: UpdateBillingInformation): Promise<BillingInformation> {
    const info: BillingInformation = await this.apiClient.post(Commands.updateBillingInformation, input);
    return info;
  }

  async updateGroupAvatar(groupId: string, file: File): Promise<Group> {
    this.validateAvatar(file);


    const renamedFile = new File([file], groupId, {
      type: file.type,
      lastModified: file.lastModified,
    });
    const formData = new FormData();
    formData.append('file', renamedFile);
    const group: Group = await this.apiClient.upload(Commands.updateGroupAvatar, formData);

    const namespace: Namespace = {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      id: group.namespace_id!,
      name: group.name,
      path: group.path,
      avatar_url: group.avatar_url,
    };
    this.store.commit(Mutation.UPDATE_NAMESPACE, namespace);

    return group;
  }

  async updateGroupProfile(input: UpdateGroupProfile): Promise<Group> {
    const group: Group = await this.apiClient.post(Commands.updateGroupProfile, input);

    const namespace: Namespace = {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      id: group.namespace_id!,
      name: group.name,
      path: group.path,
      avatar_url: group.avatar_url,
    };
    this.store.commit(Mutation.UPDATE_NAMESPACE, namespace);

    return group;
  }

  async updateMyAvatar(file: File): Promise<User> {
    this.validateAvatar(file);

    const formData = new FormData();
    formData.append('file', file);
    const user: User = await this.apiClient.upload(Commands.updateMyAvatar, formData);

    const namespace: Namespace = {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      id: user.namespace_id!,
      name: user.name,
      path: user.username,
      avatar_url: user.avatar_url,
    };
    this.store.commit(Mutation.UPDATE_NAMESPACE, namespace);


    return user;
  }

  async updateMyProfile(input: UpdateMyProfile): Promise<User> {
    const user: User = await this.apiClient.post(Commands.updateMyProfile, input);

    const namespace: Namespace = {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      id: user.namespace_id!,
      name: user.name,
      path: user.username,
      avatar_url: user.avatar_url,
    };
    this.store.commit(Mutation.UPDATE_NAMESPACE, namespace);

    return user;
  }


  validateAvatar(file: File) {
    if (file.type !== 'image/jpeg' && file.type !== 'image/png') {
      throw new Error('Image format must be png, jpg or jpeg');
    }

    // 2 MB
    if (file.size > 2000000) {
      throw new Error('File size must be less or equal to 2MB');
    }
  }
  // eslint-disable-next-line spaced-comment
  ////////////////////////////////////////////////////////////////////////////

  // async updateGroupAvatar(groupId: string, file: File): Promise<string> {
  //   this.validateAvatar(file);

  //   const query = `
  //     mutation($input: UpdateGroupProfileInput!) {
  //       updateGroupProfile(input: $input) {
  //         id
  //         avatarUrl
  //       }
  //     }
  //   `;
  //   const input: UpdateGroupProfileInput = {
  //     id: groupId,
  //   };
  //   const variables = { input };
  //   const operations = { query, variables };
  //   const map = {
  //     0: ['variables.input.avatar'],
  //   };

  //   const formData = new FormData();
  //   formData.append('operations', JSON.stringify(operations));
  //   formData.append('map', JSON.stringify(map));
  //   formData.append('0', file);

  //   const res: { updateGroupProfile: Group } = await this.apiClient.upload(formData);
  //   return res.updateGroupProfile.avatar_url;
  // }

  async fetchStatusPage(projectFullPath: string): Promise<StatusPage> {
    const query = `
      query($projectFullPath: String!) {
        statusPage(projectFullPath: $projectFullPath) {
          name
          avatarUrl
          twitterUrl
          facebookUrl
          publicEmail
          instagramUrl
          whatsappNumber
          mastodonUrl
          homepageUrl

          monitors {
            name
            status
          }
        }
      }
    `;
    const variables = { projectFullPath };

    const res: { statusPage: StatusPage } = await this.apiClient.query(query, variables);
    return res.statusPage;
  }
}

export const KernelServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: KernelService) {
    Vue.prototype.$kernelService = service;
  },
};
