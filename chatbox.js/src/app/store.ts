import Vue from 'vue';
import Vuex from 'vuex';
import { ChatboxMessage, ChatboxPreferences, Chatbox } from '@/domain/chatbox/model';

Vue.use(Vuex);

// eslint-disable-next-line no-shadow
export enum Mutation {
  OPEN = 'OPEN',
  CLOSE = 'CLOSE',
  MESSAGE_SENT = 'MESSAGE_SENT',
  MESSAGE_RECEIVED = 'MESSAGE_RECEIVED',
  CHATBOX_FETCHED = 'CHATBOX_FETCHED',
  CONNECTED = 'CONNECTED',
  DISCONNECTED = 'DISCONNECTED',
}

export interface AppState {
  isOpen: boolean;
  preferences: ChatboxPreferences | null;
  messages: ChatboxMessage[];
  showFooter: boolean;
  reconnecting: boolean;
}

const seenMessages = new Set<string>();

export default new Vuex.Store<AppState>({
  state: {
    isOpen: false,
    showFooter: true,
    preferences: null,
    messages: [],
    reconnecting: false,
  },
  mutations: {
    [Mutation.OPEN](state: AppState) {
      state.isOpen = true;
    },
    [Mutation.CLOSE](state: AppState) {
      state.isOpen = false;
    },
    [Mutation.MESSAGE_RECEIVED](state: AppState, message: ChatboxMessage) {
      if (!seenMessages.has(message.id)) {
        state.messages.push(message);
        seenMessages.add(message.id);
      }
    },
    [Mutation.CHATBOX_FETCHED](state: AppState, chatbox: Chatbox) {
      state.preferences = chatbox.preferences;
      chatbox.messages.forEach((message: ChatboxMessage) => {
        if (!seenMessages.has(message.id)) {
          state.messages.push(message);
          seenMessages.add(message.id);
        }
      });
    },
    [Mutation.CONNECTED](state: AppState) {
      state.reconnecting = false;
    },
    [Mutation.DISCONNECTED](state: AppState) {
      state.reconnecting = true;
    },
  },
  actions: {
  },
  modules: {
  },
});
