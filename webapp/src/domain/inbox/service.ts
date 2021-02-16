/* eslint-disable no-shadow */
/* eslint-disable @typescript-eslint/no-non-null-assertion */
/* eslint-disable class-methods-use-this */
import ApiClient from '@/api/client';
import { Store } from 'vuex';
import { AppState } from '@/app/store';
import Router from '@/app/router';
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
  UpdateContact,
  DeleteContact,
  CreateContact,
  MoveConversation,
} from './model';


const DEFAULT_MESSAGES_TIMEOUT = 2000; // 2 secs

export enum InboxType {
  Inbox,
  Archive,
  Spam,
  Trash,
}

export type InboxSubscriptionOptions = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
 onData: (data: ConversationWithContactsAndMessages) => void;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
 onError: (err: any) => void;
 onDisconnected?: () => void;
 onConnected?: () => void;
 inboxType: InboxType,
}

export class InboxService {
  private apiClient: ApiClient;
  private store: Store<AppState>;
  private subscriptionTimeout: number;
  private router: Router;
  private subscriptionType: InboxType;
  private lastMessageId: string | null;

  constructor(apiClient: ApiClient, store: Store<AppState>, router: Router) {
    this.apiClient = apiClient;
    this.store = store;
    this.router = router;
    this.subscriptionTimeout = 0;
    this.subscriptionType = InboxType.Inbox;
    this.lastMessageId = null;
  }

  async createContact(input: CreateContact): Promise<void> {
    const contact: Contact = await this.apiClient.post(Commands.createContact, input);

    this.router.push({ path: `/inbox/contacts/${contact.id}` });
  }

  async deleteContact(input: DeleteContact): Promise<void> {
    await this.apiClient.post(Commands.deleteContact, input);

    this.router.push({ path: '/inbox/contacts' });
  }

  async fetchArchive(after: string | null): Promise<Inbox> {
    this.lastMessageId = after;
    const input: GetArchive = {
      namespace_id: this.store.state.currentNamespace!.id!,
      after,
    };
    const res: Inbox = await this.apiClient.post(Queries.archive, input);

    return res;
  }

  async fetchChatboxPreferences(): Promise<ChatboxPreferences> {
    const input: GetChatboxPreferences = {
      namespace_id: this.store.state.currentNamespace!.id!,
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
      namespace_id: this.store.state.currentNamespace!.id!,
    };
    const res: Contact[] = await this.apiClient.post(Queries.contacts, input);

    return res;
  }

  async fetchInbox(after: string | null): Promise<Inbox> {
    this.lastMessageId = after;
    const input: GetInbox = {
      namespace_id: this.store.state.currentNamespace!.id!,
      after,
    };
    const res: Inbox = await this.apiClient.post(Queries.inbox, input);

    return res;
  }

  async fetchInboxMessages(options: InboxSubscriptionOptions): Promise<void> {
    if (this.subscriptionTimeout === 0) {
      return;
    }

    let inbox = null;

    try {
      switch (this.subscriptionType) {
        case InboxType.Archive:
          inbox = await this.fetchArchive(this.lastMessageId);
          break;
        case InboxType.Inbox:
          inbox = await this.fetchInbox(this.lastMessageId);
          break;
        case InboxType.Spam:
          inbox = await this.fetchSpam(this.lastMessageId);
          break;
        case InboxType.Trash:
          inbox = await this.fetchTrash(this.lastMessageId);
          break;
        default:
          inbox = await this.fetchInbox(this.lastMessageId);
      }
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

  async fetchSpam(after: string | null): Promise<Inbox> {
    this.lastMessageId = after;
    const input: GetSpam = {
      namespace_id: this.store.state.currentNamespace!.id!,
      after,
    };
    const res: Inbox = await this.apiClient.post(Queries.spam, input);

    return res;
  }

  async fetchTrash(after: string | null): Promise<Inbox> {
    this.lastMessageId = after;
    const input: GetTrash = {
      namespace_id: this.store.state.currentNamespace!.id!,
      after,
    };
    const res: Inbox = await this.apiClient.post(Queries.trash, input);

    return res;
  }

  async importContacts(input: ImportContacts): Promise<Contact[]> {
    const res: Contact[] = await this.apiClient.post(Commands.importContacts, input);

    return res;
  }

  async moveConversationToArchive(conversationId: string): Promise<void> {
    const input: MoveConversation = {
      conversation_id: conversationId,
    };
    await this.apiClient.post(Commands.moveConversationToArchive, input);
  }

  async moveConversationToInbox(conversationId: string): Promise<void> {
    const input: MoveConversation = {
      conversation_id: conversationId,
    };
    await this.apiClient.post(Commands.moveConversationToInbox, input);
  }

  async moveConversationToSpam(conversationId: string): Promise<void> {
    const input: MoveConversation = {
      conversation_id: conversationId,
    };
    await this.apiClient.post(Commands.moveConversationToSpam, input);
  }

  async moveConversationToTrash(conversationId: string): Promise<void> {
    const input: MoveConversation = {
      conversation_id: conversationId,
    };
    await this.apiClient.post(Commands.moveConversationToTrash, input);
  }

  async sendMessage(input: SendMessage): Promise<Message> {
    const res: Message = await this.apiClient.post(Commands.sendMessage, input);

    return res;
  }

  setLastMessageId(messageId: string | null) {
    this.lastMessageId = messageId;
  }

  subscribeToInbox(options: InboxSubscriptionOptions): void {
    // eslint-disable-next-line no-unneeded-ternary
    const startPolling = this.subscriptionTimeout === 0 ? true : false;
    this.subscriptionTimeout = DEFAULT_MESSAGES_TIMEOUT;
    this.subscriptionType = options.inboxType;

    if (startPolling) {
      this.fetchInboxMessages(options);
    }
  }

  unsubscribeFromInbox(): void {
    this.subscriptionTimeout = 0;
  }

  async updateChatboxPreferences(input: UpdateChatboxPreferences): Promise<ChatboxPreferences> {
    // eslint-disable-next-line max-len
    const res: ChatboxPreferences = await this.apiClient.post(Commands.updateChetboxPreferences, input);

    return res;
  }

  async updateContact(input: UpdateContact): Promise<Contact> {
    const res: Contact = await this.apiClient.post(Commands.updateContact, input);

    return res;
  }
}

export const InboxServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: InboxService) {
    Vue.prototype.$inboxService = service;
  },
};
