/* eslint-disable no-underscore-dangle */
/* eslint-disable class-methods-use-this */
/* eslint-disable max-len */
import ApiClient from '@/api/client';
import {
  Group, StatusPage, UpdateGroupProfileInput, UpdateMyProfileInput,
} from '@/api/graphql/model';
import { AppState, Mutation } from '@/app/store';
import { Store } from 'vuex';
import Router from '@/app/router';
import { Commands, Queries } from './routes';
import {
  AcceptGroupInvitation,
  CancelGroupInvitation,
  CompleteRegistration, CompleteSignIn, CompleteTwoFaChallenge, CompleteTwoFaSetup, DeclineGroupInvitation, DeleteMyAccount, DisableTwoFa, GenerateQrCode, GetSignedUploadUrl, GroupInvitation, Me, QrCode, Register, RegistrationStarted, RevokeSession, Session, SetupTwoFa, SignedIn, SignedUploadUrl, SignIn, SignInStarted, UpdateMyProfile,
  User,
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

  async cancelGroupInvitation(invitationId: string): Promise<void> {
    const input: CancelGroupInvitation = {
      invitation_id: invitationId,
    };
    await this.apiClient.post(Commands.cancelGroupInvitation, input);
  }

  async completeRegistration(input: CompleteRegistration): Promise<void> {
    const res: SignedIn = await this.apiClient.post(Commands.completeRegistration, input);

    this.store.commit(Mutation.SIGN_IN, res);
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
    this.router.push({ path: '/' });
  }

  async completeTwoFaChallenge(input: CompleteTwoFaChallenge): Promise<void> {
    const res: SignedIn = await this.apiClient.post(Commands.completeTwoFaChallenge, input);

    // complete sign-in flow
    this.store.commit(Mutation.SIGN_IN, res);
    this.router.push({ path: '/' });
  }

  async completeTwoFaSetup(code: string): Promise<void> {
    const input: CompleteTwoFaSetup = {
      code,
    };
    await this.apiClient.post(Commands.completeTwoFaSetup, input);
  }

  async declineGroupInvitation(invitationId: string): Promise<void> {
    const input: DeclineGroupInvitation = {
      invitation_id: invitationId,
    };
    await this.apiClient.post(Commands.declineGroupInvitation, input);
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

  async register(input: Register): Promise<void> {
    const res: RegistrationStarted = await this.apiClient.post(Commands.register, input);

    this.store.commit(Mutation.SET_PENDING_USER_ID, res.pending_user_id);
    this.router.push({ path: '/register/complete' });
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
      namespace_id: this.store.state.currentNamespaceId!,
      filesize,
    };
    const res: SignedUploadUrl = await this.apiClient.post(Queries.signedUploadUrl, input);

    return res;
  }

  async updateMyProfile(input: UpdateMyProfile): Promise<User> {
    const res: User = await this.apiClient.post(Commands.updateMyProfile, input);

    this.store.commit(Mutation.UPDATE_MY_PROFILE, res);

    return res;
  }

  // eslint-disable-next-line spaced-comment
  ////////////////////////////////////////////////////////////////////////////

  validateAvatar(file: File) {
    if (file.type !== 'image/jpeg' && file.type !== 'image/png') {
      throw new Error('Image format must be png, jpg or jpeg');
    }

    // 2 MB
    if (file.size > 2000000) {
      throw new Error('File size must be less or equal to 2MB');
    }
  }

  async updateMyAvatar(file: File): Promise<string> {
    this.validateAvatar(file);

    const query = `
      mutation($input: UpdateMyProfileInput!) {
        updateMyProfile(input: $input) {
          id
          avatarUrl
        }
      }
    `;
    const input: UpdateMyProfileInput = {};
    const variables = { input };
    const operations = { query, variables };
    const map = {
      0: ['variables.input.avatar'],
    };

    const formData = new FormData();
    formData.append('operations', JSON.stringify(operations));
    formData.append('map', JSON.stringify(map));
    formData.append('0', file);

    const res: { updateMyProfile: User } = await this.apiClient.upload(formData);
    this.store.commit(Mutation.UPDATE_MY_PROFILE, res.updateMyProfile);
    return res.updateMyProfile.avatar_url;
  }

  async updateGroupAvatar(groupId: string, file: File): Promise<string> {
    this.validateAvatar(file);

    const query = `
      mutation($input: UpdateGroupProfileInput!) {
        updateGroupProfile(input: $input) {
          id
          avatarUrl
        }
      }
    `;
    const input: UpdateGroupProfileInput = {
      id: groupId,
    };
    const variables = { input };
    const operations = { query, variables };
    const map = {
      0: ['variables.input.avatar'],
    };

    const formData = new FormData();
    formData.append('operations', JSON.stringify(operations));
    formData.append('map', JSON.stringify(map));
    formData.append('0', file);

    const res: { updateGroupProfile: Group } = await this.apiClient.upload(formData);
    return res.updateGroupProfile.avatarUrl;
  }

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
