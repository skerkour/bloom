/* eslint-disable @typescript-eslint/no-non-null-assertion */
import Vue from 'vue';
import Vuex, { Store } from 'vuex';
import { Storage } from '@/app/storage';
import {
  SignedIn, User, Session, Me, Namespace,
} from '@/domain/kernel/model';

Vue.use(Vuex);

export interface AppState {
  darkMode: boolean;
  me: User | null;
  session: Session | null;
  sessionToken: string | null;
  pendingUserId: string | null;
  pendingSessionId: string | null;
  namespaceIsGroup: boolean;
  drawer: boolean;
  currentNamespaceId: string | null,
  namespaces: Namespace[],

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  [state: string]: any;
}

// eslint-disable-next-line
export enum Mutation {
  SIGN_IN = 'SIGN_IN',
  INIT = 'INIT',
  SIGN_OUT = 'SIGN_OUT',
  SET_PENDING_USER_ID = 'SET_PENDING_USER_ID',
  SET_PENDING_SESSION_ID = 'SET_PENDING_SESSION_ID',
  UPDATE_MY_PROFILE = 'UPDATE_MY_PROFILE',
  SET_CURRENT_NAMESPACE_ID = 'SET_CURRENT_NAMESPACE_ID',
  SET_DRAWER = 'SET_DRAWER',
  ADD_NAMESPACE = 'ADD_NAMESPACE',
  REMOVE_NAMESPACE = 'REMOVE_NAMESPACE',
  UPDATE_NAMESPACE = 'UPDATE_NAMESPACE',
}

function defaultAppState(): AppState {
  return {
    darkMode: false,
    me: null,
    session: null,
    sessionToken: null,
    pendingUserId: null,
    pendingSessionId: null,
    namespaceIsGroup: false,
    drawer: true,
    currentNamespaceId: null,
    namespaces: [],
  };
}


export function newStore(storage: Storage): Store<AppState> {
  const baseAppState = defaultAppState();

  const storedDarkMode = storage.get(storage.keyDarkMode);
  if (storedDarkMode) {
    baseAppState.darkMode = storedDarkMode;
  }

  // const storedMe = storage.get(storage.keyMe);
  // if (storedMe) {
  //   baseAppState.me = storedMe;
  // }

  const storedToken = storage.get(storage.keyToken);
  if (storedToken) {
    baseAppState.sessionToken = storedToken;
  }

  return new Store<AppState>({
    state: baseAppState,
    mutations: {
      [Mutation.SIGN_IN](state: AppState, params: SignedIn) {
        state.session = params.me.session;
        state.sessionToken = params.token;
        state.me = params.me.user;
        state.groups = params.me.groups;
        state.currentNamespaceId = params.me.user.namespace_id;

        // storage.set(storage.keyMe, state.me);
        storage.set(storage.keyToken, state.sessionToken);

        state.pendingSessionId = null;
        state.pendingUserId = null;
      },
      [Mutation.INIT](state: AppState, me: Me) {
        state.session = me.session;
        state.me = me.user;
        state.groups = me.groups;
        state.currentNamespaceId = me.user.namespace_id;

        const namespaces: Namespace[] = [{
          namespace_id: me.user.namespace_id!,
          name: me.user.name,
          path: me.user.username,
          avatar_url: me.user.avatar_url,
        }];

        me.groups.forEach((group) => {
          const namespace: Namespace = {
            namespace_id: group.namespace_id!,
            name: group.name,
            path: group.path,
            avatar_url: group.avatar_url,
          };
          namespaces.push(namespace);
        });

        state.namespaces = namespaces;

        // storage.set(storage.keyMe, state.me);
      },
      [Mutation.SIGN_OUT](state: AppState) {
        const emptyState = defaultAppState();
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        Object.entries(emptyState).forEach(([key, value]: [string, any]) => {
          state[key] = value;
        });

        storage.clear();
      },
      [Mutation.SET_PENDING_USER_ID](state: AppState, pendingUserId: string) {
        state.pendingUserId = pendingUserId;
      },
      [Mutation.SET_PENDING_SESSION_ID](state: AppState, pendingSessionId: string) {
        state.pendingSessionId = pendingSessionId;
      },
      [Mutation.UPDATE_MY_PROFILE](state: AppState, me: User) {
        state.me!.username = me.username;
        state.me!.name = me.name;
        state.me!.avatar_url = me.avatar_url;
        // storage.set(storage.keyMe, state.me);
      },
      [Mutation.SET_DRAWER](state: AppState, value: boolean) {
        state.drawer = value;
      },
      [Mutation.SET_CURRENT_NAMESPACE_ID](state: AppState, namespaceId: string) {
        state.currentNamespaceId = namespaceId;
      },
      [Mutation.ADD_NAMESPACE](state: AppState, namespace: Namespace) {
        state.namespaces.push(namespace);
      },
      [Mutation.REMOVE_NAMESPACE](state: AppState, path: string) {
        state.namespaces = state.namespaces.filter((namespace) => namespace.path !== path);
      },
      [Mutation.UPDATE_NAMESPACE](state: AppState, namespace: Namespace) {
        state.namespaces = state.namespaces.map((n) => {
          if (n.namespace_id === namespace.namespace_id) {
            return namespace;
          }
          return n;
        });
      },
    },
    actions: {
    },
    modules: {
    },
  });
}
