
/* eslint-disable class-methods-use-this */
import ApiClient from '@/api/client';
import Router from '@/app/router';
import { AppState } from '@/app/store';
import { Store } from 'vuex';
import {
  CreateList,
  CreateMessage,
  DeleteList,
  DeleteMessage,
  GetList, GetLists, GetMessage, List, ListWithDetails,
  Message, MessageWithLists,
  SendMessage, SendTestMessage, SubscribeToList, UpdateList, UpdateMessage, UnsubscribeFromList,
  RemoveContactFromList,
} from './model';
import { Commands, Queries } from './routes';

export class NewsletterService {
  private apiClient: ApiClient;
  private store: Store<AppState>;
  private router: Router;

  constructor(apiClient: ApiClient, store: Store<AppState>, router: Router) {
    this.apiClient = apiClient;
    this.store = store;
    this.router = router;
  }

  async createMessage(input: CreateMessage): Promise<void> {
    const message: Message = await this.apiClient.post(Commands.createMessage, input);

    this.router.push({ path: `/newsletter/lists/${input.list_id}/messages/${message.id}` });
  }

  async createList(input: CreateList): Promise<void> {
    const list: List = await this.apiClient.post(Commands.createList, input);

    this.router.push({ path: `/newsletter/lists/${list.id}` });
  }

  async deleteList(listId: string): Promise<void> {
    const input: DeleteList = {
      list_id: listId,
    };
    await this.apiClient.post(Commands.deleteList, input);

    this.router.push({ path: '/newsletter/lists' });
  }

  async deleteMessage(messageId: string, listId: string): Promise<void> {
    const input: DeleteMessage = {
      message_id: messageId,
    };
    await this.apiClient.post(Commands.deleteMessage, input);

    this.router.push({ path: `/newsletter/lists/${listId}` });
  }

  async fetchList(listId: string): Promise<ListWithDetails> {
    const input: GetList = {
      list_id: listId,
    };
    const res: ListWithDetails = await this.apiClient.post(Queries.list, input);

    return res;
  }

  async fetchLists(): Promise<List[]> {
    const input: GetLists = {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      namespace_id: this.store.state.currentNamespace!.id,
    };
    const res: List[] = await this.apiClient.post(Queries.lists, input);

    return res;
  }

  async fetchMessage(messageId: string): Promise<MessageWithLists> {
    const input: GetMessage = {
      message_id: messageId,
    };
    const res: MessageWithLists = await this.apiClient.post(Queries.message, input);

    return res;
  }

  async removeContactFromList(input: RemoveContactFromList): Promise<void> {
    await this.apiClient.post(Commands.removeContactFromList, input);
  }

  async sendMessage(messageId: string): Promise<void> {
    const input: SendMessage = {
      message_id: messageId,
    };
    await this.apiClient.post(Commands.sendMessage, input);
  }

  async sendTestMessage(messageId: string): Promise<void> {
    const input: SendTestMessage = {
      message_id: messageId,
    };
    await this.apiClient.post(Commands.sendTestMessage, input);
  }

  async subscribeToList(input: SubscribeToList): Promise<void> {
    await this.apiClient.post(Commands.subscribeToList, input);
  }

  async unsubscribeFromList(input: UnsubscribeFromList): Promise<void> {
    await this.apiClient.post(Commands.unsubscribeFromList, input);
  }

  async updateList(input: UpdateList): Promise<List> {
    const list: List = await this.apiClient.post(Commands.updateList, input);
    return list;
  }

  async updateMessage(input: UpdateMessage): Promise<MessageWithLists> {
    const message: MessageWithLists = await this.apiClient.post(Commands.updateMessage, input);
    return message;
  }
}

export const NewsletterServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: NewsletterService) {
    Vue.prototype.$newsletterService = service;
  },
};
