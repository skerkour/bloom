/* eslint-disable max-len */
/* eslint-disable max-classes-per-file */

import APIClient from '@/api/client';
import { AppState, Mutation } from '@/app/store';
import { Store } from 'vuex';
import { BloomService } from '../bloom';
import {
  ChatboxPreferences, GetChatboxMessages, GetChatboxPreferences, Chatbox,
  ChatboxMessage, SendChatboxMessage, LinkChatboxContact,
} from './model';
import { Commands, Queries } from './routes';

const CLOSED_MESSAGES_TIMEOUT = 15000; // 15 secs
const LIVE_MESSAGES_TIMEOUT = 1500; // 1.5 secs

export class ChatboxService {
  private apiClient: APIClient;
  private store: Store<AppState>;
  private bloomService: BloomService;
  private messagesTimeout: number;

  constructor(apiClient: APIClient, store: Store<AppState>, bloomService: BloomService) {
    this.apiClient = apiClient;
    this.store = store;
    this.bloomService = bloomService;
    this.messagesTimeout = CLOSED_MESSAGES_TIMEOUT;
  }

  closeAskEmailMessage() {
    this.store.commit(Mutation.CLOSE_ASK_FOR_EMAIL);
  }

  async fetchChatbox(): Promise<Chatbox> {
    const inputPref: GetChatboxPreferences = {
      namespace_id: this.bloomService.namespaceId,
    };
    const chatboxPrefPromise: Promise<ChatboxPreferences> = this.apiClient.post(Queries.chatboxPreferences, inputPref);

    const inputMessages: GetChatboxMessages = {
      namespace_id: this.bloomService.namespaceId,
      after: this.store.state.lastReceivedMessageId,
    };
    const chatboxMessagesPromise: Promise<ChatboxMessage[]> = this.apiClient.post(Queries.chatboxMessages, inputMessages);

    const res = await Promise.all([chatboxPrefPromise, chatboxMessagesPromise]);

    const ret: Chatbox = {
      messages: res[1],
      preferences: res[0],
    };
    return ret;
  }

  async fetchMessages(): Promise<void> {
    let messages: ChatboxMessage[] = [];

    if (this.messagesTimeout === 0) {
      return;
    }

    try {
      const input: GetChatboxMessages = {
        namespace_id: this.bloomService.namespaceId,
        after: this.store.state.lastReceivedMessageId,
      };
      messages = await this.apiClient.post(Queries.chatboxMessages, input);
    } catch (err) {
      console.error(err);
    }
    messages.forEach((message) => {
      this.store.commit(Mutation.MESSAGE_RECEIVED, message);
    });

    // recursive call
    if (this.messagesTimeout !== 0) {
      if (this.store.state.isOpen) {
        this.messagesTimeout = LIVE_MESSAGES_TIMEOUT;
      } else {
        this.messagesTimeout = CLOSED_MESSAGES_TIMEOUT;
      }
      setTimeout(() => {
        this.fetchMessages();
      }, this.messagesTimeout);
    }
  }

  async linkContact(email: string): Promise<void> {
    const input: LinkChatboxContact = {
      email,
      namespace_id: this.bloomService.namespaceId,
    };
    await this.apiClient.post(Commands.linkContact, input);

    this.store.commit(Mutation.EMAIL_ASKED);
    this.store.commit(Mutation.CLOSE_ASK_FOR_EMAIL);


    this.sendMessage(`My email is ${email}`);
  }

  async sendMessage(body: string): Promise<void> {
    const input: SendChatboxMessage = {
      body,
      namespace_id: this.bloomService.namespaceId,
    };
    const message: ChatboxMessage = await this.apiClient.post(Commands.sendChatboxMessages, input);

    this.store.commit(Mutation.MESSAGE_RECEIVED, message);

    if (!this.store.state.emailAsked) {
      this.store.commit(Mutation.ASK_FOR_EMAIL);
    }
  }

  subscribeToChatboxMessages(): void {
    this.messagesTimeout = CLOSED_MESSAGES_TIMEOUT;
    this.fetchMessages();
  }

  unsubscribeFromChatboxMessages(): void {
    this.messagesTimeout = 0;
  }
}

export const ChatboxServiceProvider = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: ChatboxService) {
    Vue.prototype.$chatbox = service;
  },
};
