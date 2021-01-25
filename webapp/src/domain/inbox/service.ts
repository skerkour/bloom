/* eslint-disable class-methods-use-this */
import ApiClient from '@/api/client';
import { Store } from 'vuex';
import { AppState } from '@/app/store';
import { Commands, Queries } from './routes';
import {
  GetInbox, Inbox, GetArchive, GetSpam, GetTrash, SendMessage, Message,
} from './model';

export type StorageSignedUploadUrlInput = {
  size: number;
}

export class InboxService {
  private apiClient: ApiClient;
  private store: Store<AppState>;

  constructor(apiClient: ApiClient, store: Store<AppState>) {
    this.apiClient = apiClient;
    this.store = store;
  }

  async fetchArchive(input: GetArchive): Promise<Inbox> {
    const res: Inbox = await this.apiClient.post(Queries.inbox, input);

    return res;
  }

  async fetchInbox(input: GetInbox): Promise<Inbox> {
    const res: Inbox = await this.apiClient.post(Queries.inbox, input);

    return res;
  }

  async fetchSpam(input: GetSpam): Promise<Inbox> {
    const res: Inbox = await this.apiClient.post(Queries.inbox, input);

    return res;
  }

  async fetchTrash(input: GetTrash): Promise<Inbox> {
    const res: Inbox = await this.apiClient.post(Queries.inbox, input);

    return res;
  }

  async sendMessage(input: SendMessage): Promise<Message> {
    const res: Message = await this.apiClient.post(Commands.sendMessage, input);

    return res;
  }
}

export const InboxServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: InboxService) {
    Vue.prototype.$inboxService = service;
  },
};
