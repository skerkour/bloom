/* eslint-disable max-classes-per-file */

import APIClient from '@/api/client';
import { AppState, Mutation } from '@/app/store';
import { Store } from 'vuex';
import { SendChatboxMessageInput, Chatbox, ChatboxMessage } from '@/api/graphql/model';
import { BloomService } from '../bloom';

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

  async fetchChatbox(): Promise<Chatbox> {
    const query = `query($projectId: ID!) {
      chatbox(projectId: $projectId) {
        preferences {
          color
          name
          avatarUrl
          twitterUrl
          facebookUrl
          publicEmail
          instagramUrl
          whatsappNumber
          mastodonUrl
          homepageUrl
          branding
          welcomeMessage
        }
        messages {
          id
          createdAt
          author {
            name
            avatarUrl
          }
          bodyHtml
        }
      }
    }`;
    const variables = { projectId: this.bloomService.projectId };

    const res: { chatbox: Chatbox } = await this.apiClient.query(query, variables);
    return res.chatbox;
  }

  async sendMessage(messageBody: string): Promise<void> {
    const query = `
      mutation($input:SendChatboxMessageInput!) {
        sendChatboxMessage(input:$input) {
          id
          createdAt
          author {
            name
            avatarUrl
          }
          bodyHtml
        }
      }
    `;
    const input: SendChatboxMessageInput = {
      body: messageBody,
      projectId: this.bloomService.projectId,
    };
    const variables = { input };

    // eslint-disable-next-line max-len
    const message: { sendChatboxMessage: ChatboxMessage } = await this.apiClient.query(query, variables);
    this.store.commit(Mutation.MESSAGE_RECEIVED, message.sendChatboxMessage);
  }

  subscribeToChatboxMessages(): void {
    this.messagesTimeout = CLOSED_MESSAGES_TIMEOUT;
    this.fetchMessages();
  }

  unsubscribeFromChatboxMessages(): void {
    this.messagesTimeout = 0;
  }

  async fetchMessages(): Promise<void> {
    let res: { chatbox: Chatbox } | null = null;

    if (this.messagesTimeout === 0) {
      return;
    }

    try {
      const query = `query($projectId: ID!) {
        chatbox(projectId: $projectId) {
          messages {
            id
            createdAt
            author {
              name
              avatarUrl
            }
            bodyHtml
          }
        }
      }`;
      const variables = { projectId: this.bloomService.projectId };

      res = await this.apiClient.query(query, variables);
    } catch (err) {
      console.error(err);
    }
    res?.chatbox.messages.forEach((message: ChatboxMessage) => {
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
}

export const ChatboxServiceProvider = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: ChatboxService) {
    Vue.prototype.$chatbox = service;
  },
};
