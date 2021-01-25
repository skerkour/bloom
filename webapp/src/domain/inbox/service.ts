/* eslint-disable class-methods-use-this */
import ApiClient from '@/api/client';
import { Store } from 'vuex';
import { AppState } from '@/app/store';
import { Commands, Queries } from './routes';
import {
  GetInbox, Inbox, GetArchive, GetSpam, GetTrash, SendMessage, Message,
  ConversationWithConatctsAndMessages,
} from './model';


const DEFAULT_MESSAGES_TIMEOUT = 2000; // 2 secs


export interface InboxSubscriptionOptions {
  // eslint-disable-next-line camelcase
  namespace_id: string;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
 onData: (data: ConversationWithConatctsAndMessages) => void;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
 onError: (err: any) => void;
 onDisconnected?: () => void;
 onConnected?: () => void;
}

export class InboxService {
  private apiClient: ApiClient;
  private store: Store<AppState>;
  private subscriptionTimeout: number;


  constructor(apiClient: ApiClient, store: Store<AppState>) {
    this.apiClient = apiClient;
    this.store = store;
    this.subscriptionTimeout = DEFAULT_MESSAGES_TIMEOUT;
  }

  async fetchArchive(input: GetArchive): Promise<Inbox> {
    const res: Inbox = await this.apiClient.post(Queries.inbox, input);

    return res;
  }

  async fetchInbox(input: GetInbox): Promise<Inbox> {
    const res: Inbox = await this.apiClient.post(Queries.inbox, input);

    return res;
  }

  async fetchInboxMessages(options: InboxSubscriptionOptions): Promise<void> {
    if (this.subscriptionTimeout === 0) {
      return;
    }

    try {
      const input: GetInbox = {
        namespace_id: options.namespace_id,
      };
      const inbox = await this.fetchInbox(input);
      inbox.conversations.forEach((conversation) => {
        // conversation.messages.forEach((message: InboxMessage) => {
        options?.onData(conversation);
        // });
      });
    } catch (err) {
      options.onError(err);
    }

    // recursive call
    if (this.subscriptionTimeout !== 0) {
      setTimeout(() => {
        this.fetchInboxMessages(options);
      }, this.subscriptionTimeout);
    }
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

  subscribeToInbox(options: InboxSubscriptionOptions): void {
    this.subscriptionTimeout = DEFAULT_MESSAGES_TIMEOUT;
    this.fetchInboxMessages(options);
  }

  unsubscribeFromInbox(): void {
    this.subscriptionTimeout = 0;
  }
}

export const InboxServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: InboxService) {
    Vue.prototype.$inboxService = service;
  },
};
