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
  ASK_FOR_EMAIL = 'ASK_FOR_EMAIL',
  CLOSE_ASK_FOR_EMAIL = 'CLOSE_ASK_FOR_EMAIL',
  EMAIL_ASKED = 'EMAIL_ASKED',
}

export interface AppState {
  isOpen: boolean;
  preferences: ChatboxPreferences | null;
  messages: ChatboxMessage[];
  showFooter: boolean;
  reconnecting: boolean;
  askForEmail: boolean;
  emailAsked: boolean;
  lastReceivedMessageId: string | null;
}

const seenMessages = new Set<string>();

export default new Vuex.Store<AppState>({
  state: {
    isOpen: false,
    showFooter: true,
    preferences: null,
    messages: [],
    reconnecting: false,
    askForEmail: false,
    emailAsked: false,
    lastReceivedMessageId: null,
  },
  mutations: {
    [Mutation.OPEN](state: AppState) {
      state.isOpen = true;
    },
    [Mutation.CLOSE](state: AppState) {
      state.isOpen = false;
    },
    [Mutation.MESSAGE_RECEIVED](state: AppState, message: ChatboxMessage) {
      if (message.body_html.includes('My email is')) {
        state.emailAsked = true;
      }

      if (!seenMessages.has(message.id)) {
        state.messages.push(message);
        seenMessages.add(message.id);
        state.lastReceivedMessageId = message.id;
      }
    },
    [Mutation.CHATBOX_FETCHED](state: AppState, chatbox: Chatbox) {
      state.preferences = chatbox.preferences;
      chatbox.messages.forEach((message: ChatboxMessage) => {
        if (message.body_html.includes('My email is')) {
          state.emailAsked = true;
        }

        if (!seenMessages.has(message.id)) {
          state.messages.push(message);
          seenMessages.add(message.id);
          state.lastReceivedMessageId = message.id;
        }
      });

      if ((seenMessages.size === 1 && state.preferences.welcome_message.length === 0)
        || (state.preferences.welcome_message.length !== 0 && seenMessages.size === 2)) {
        state.emailAsked = false;
        state.askForEmail = true;
      }
    },
    [Mutation.CONNECTED](state: AppState) {
      state.reconnecting = false;
    },
    [Mutation.DISCONNECTED](state: AppState) {
      state.reconnecting = true;
    },
    [Mutation.ASK_FOR_EMAIL](state: AppState) {
      state.askForEmail = true;
    },
    [Mutation.CLOSE_ASK_FOR_EMAIL](state: AppState) {
      state.askForEmail = false;
    },
    [Mutation.EMAIL_ASKED](state: AppState) {
      state.emailAsked = true;
    },
  },
  actions: {
  },
  modules: {
  },
});
