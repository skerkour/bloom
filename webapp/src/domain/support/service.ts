import ApiClient from '@/api/client';
import {
  Project, SendMessageToConversationInput, UpdateChatboxPreferencesInput,
  ChatboxPreferences, InboxConversation, InboxMessage,
} from '@/api/graphql/model';
import { Store } from 'vuex';
import { AppState } from '@/app/store';
import Router from '@/app/router';


const DEFAULT_MESSAGES_TIMEOUT = 2000; // 2 secs

export interface InboxSubscriptionOptions {
  projectFullPath: string;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
 onData: (data: any) => void;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
 onError: (err: any) => void;
 onDisconnected?: () => void;
 onConnected?: () => void;
}

export type ProjectAndBaseUrl = {
  project: Project;
  baseUrl: string;
};


export class SupportService {
  private apiClient: ApiClient;
  private messagesTimeout: number;

  constructor(apiClient: ApiClient, private store: Store<AppState>, private router: Router) {
    this.apiClient = apiClient;
    this.messagesTimeout = DEFAULT_MESSAGES_TIMEOUT;
  }

  async sendMessageToConversation(input: SendMessageToConversationInput): Promise<InboxMessage> {
    const query = `
      mutation($input: SendMessageToConversationInput!) {
        sendMessageToConversation(input: $input) {
          id
          createdAt
          conversationId
          author {
            name
            username
            avatarUrl
          }
          bodyHtml
        }
      }
    `;
    const variables = { input };

    // eslint-disable-next-line max-len
    const res: { sendMessageToConversation: InboxMessage } = await this.apiClient.query(query, variables);
    return res.sendMessageToConversation;
  }

  async findProjectConversationsWithMessages(projectFullPath: string): Promise<ProjectAndBaseUrl> {
    const query = `
      query($input: String!) {
        baseUrl
        project(fullPath: $input) {
          id
          path
          name

          conversations {
            id
            createdAt
            lastMessageReceivedAt
            contact {
              id
              avatarUrl
              name
              email
              notes
            }
            messages {
              id
              createdAt
              conversationId
              author {
                name
                username
                avatarUrl
              }
              bodyHtml
            }
          }
        }
      }
    `;
    const variables = { input: projectFullPath };

    const res: ProjectAndBaseUrl = await this.apiClient.query(query, variables);
    return res;
  }

  subscribeToInboxMessages(options: InboxSubscriptionOptions): void {
    this.messagesTimeout = DEFAULT_MESSAGES_TIMEOUT;
    this.fetchMessages(options);
  }

  unsubscribeFromInboxMessages(): void {
    this.messagesTimeout = 0;
  }

  async fetchMessages(options: InboxSubscriptionOptions): Promise<void> {
    let res: { project: Project } | null = null;

    if (this.messagesTimeout === 0) {
      return;
    }

    try {
      const query = `
        query($input: String!) {
          project(fullPath: $input) {
            id
            path
            name

            conversations {
              id
              createdAt
              lastMessageReceivedAt
              contact {
                id
                avatarUrl
                name
                email
                notes
              }
              messages {
                id
                createdAt
                conversationId
                author {
                  name
                  username
                  avatarUrl
                }
                bodyHtml
              }
            }
          }
        }
      `;
      const variables = { input: options.projectFullPath };

      res = await this.apiClient.query(query, variables);
      res?.project.conversations.forEach((conversation: InboxConversation) => {
        // conversation.messages.forEach((message: InboxMessage) => {
        options?.onData(conversation);
        // });
      });
    } catch (err) {
      options.onError(err);
    }

    // recursive call
    if (this.messagesTimeout !== 0) {
      setTimeout(() => {
        this.fetchMessages(options);
      }, this.messagesTimeout);
    }
  }

  // subscribeToChatboxMessages(projectFullPath: string, options: GraphqlSubscriptionOptions):
  //   GraphqlSubscription {
  //   const query = `subscription($input: InboxesSubscriptionInput!) {
  //     inboxes(input: $input) {
  //       __typename
  //      ... on InboxMessage {
  //         id
  //         createdAt
  //         conversationId
  //         author {
  //           id
  //           name
  //           username
  //           avatarUrl
  //         }

  //         bodyHtml
  //       }
  //     }
  //   }`;
  //   const input: InboxesSubscriptionInput = {
  //     // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  //     token: this.store.state.session?.token!,
  //     projectFullPath,
  //   };
  //   const variables = { input };

  //   return this.apiClient.subscribe(query, variables, options);
  // }

  // eslint-disable-next-line max-len
  async updateChatboxPreferences(input: UpdateChatboxPreferencesInput): Promise<ChatboxPreferences> {
    const query = `
      mutation($input: UpdateChatboxPreferencesInput!) {
        updateChatboxPreferences(input: $input) {
          name
          color
          avatarUrl
        }
      }
    `;
    const variables = { input };

    // eslint-disable-next-line max-len
    const res: { updateChatboxPreferences: ChatboxPreferences } = await this.apiClient.query(query, variables);

    return res.updateChatboxPreferences;
  }
}


export const SupportServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: SupportService): void {
    Vue.prototype.$supportService = service;
  },
};
