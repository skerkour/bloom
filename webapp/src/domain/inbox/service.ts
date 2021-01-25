/* eslint-disable @typescript-eslint/no-non-null-assertion */
/* eslint-disable class-methods-use-this */
import ApiClient from '@/api/client';
import { Store } from 'vuex';
import { AppState } from '@/app/store';
import { Commands, Queries } from './routes';
import {
  GetInbox, Inbox, GetArchive, GetSpam, GetTrash, SendMessage, Message,
  ConversationWithContactsAndMessages,
  ChatboxPreferences,
  GetChatboxPreferences,
  Contact,
  GetContacts,
  GetContact,
  ImportContacts,
  UpdateChatboxPreferences,
} from './model';


const DEFAULT_MESSAGES_TIMEOUT = 2000; // 2 secs


export interface InboxSubscriptionOptions {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
 onData: (data: ConversationWithContactsAndMessages) => void;
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

  async fetchArchive(): Promise<Inbox> {
    const input: GetArchive = {
      namespace_id: this.store.state.currentNamespaceId!,
    };
    const res: Inbox = await this.apiClient.post(Queries.archive, input);

    return res;
  }

  async fetchChatboxPreferences(): Promise<ChatboxPreferences> {
    const input: GetChatboxPreferences = {
      namespace_id: this.store.state.currentNamespaceId!,
    };
    const res: ChatboxPreferences = await this.apiClient.post(Queries.chatboxPreferences, input);

    return res;
  }

  async fetchContact(input: GetContact): Promise<Contact> {
    const res: Contact = await this.apiClient.post(Queries.contact, input);

    return res;
  }

  async fetchContacts(): Promise<Contact[]> {
    const input: GetContacts = {
      namespace_id: this.store.state.currentNamespaceId!,
    };
    const res: Contact[] = await this.apiClient.post(Queries.contacts, input);

    return res;
  }

  async fetchInbox(): Promise<Inbox> {
    const input: GetInbox = {
      namespace_id: this.store.state.currentNamespaceId!,
    };
    const res: Inbox = await this.apiClient.post(Queries.inbox, input);

    return res;
  }

  async fetchInboxMessages(options: InboxSubscriptionOptions): Promise<void> {
    if (this.subscriptionTimeout === 0) {
      return;
    }

    try {
      const inbox = await this.fetchInbox();
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

  async fetchSpam(): Promise<Inbox> {
    const input: GetSpam = {
      namespace_id: this.store.state.currentNamespaceId!,
    };
    const res: Inbox = await this.apiClient.post(Queries.spam, input);

    return res;
  }

  async fetchTrash(): Promise<Inbox> {
    const input: GetTrash = {
      namespace_id: this.store.state.currentNamespaceId!,
    };
    const res: Inbox = await this.apiClient.post(Queries.trash, input);

    return res;
  }

  async importContacts(input: ImportContacts): Promise<Contact[]> {
    const res: Contact[] = await this.apiClient.post(Commands.importContacts, input);

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

  async updateChatboxPreferences(input: UpdateChatboxPreferences): Promise<ChatboxPreferences> {
    // eslint-disable-next-line max-len
    const res: ChatboxPreferences = await this.apiClient.post(Commands.updateChetboxPreferences, input);

    return res;
  }
}

export const InboxServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: InboxService) {
    Vue.prototype.$inboxService = service;
  },
};
