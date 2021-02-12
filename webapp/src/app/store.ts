/* eslint-disable @typescript-eslint/no-non-null-assertion */
import Vue from 'vue';
import Vuex, { Store } from 'vuex';
import { Storage } from '@/app/storage';
import {
  SignedIn, User, Session, Me, Namespace,
} from '@/domain/kernel/model';
import Vuetify from 'vuetify';

Vue.use(Vuex);

export interface AppState {
  darkMode: boolean;
  me: User | null;
  session: Session | null;
  sessionToken: string | null;
  pendingUserId: string | null;
  pendingSessionId: string | null;
  drawer: boolean;
  currentNamespace: Namespace | null,
  namespaces: Namespace[],
  isAdmin: boolean,

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
  SET_CURRENT_NAMESPACE = 'SET_CURRENT_NAMESPACE',
  SET_DRAWER = 'SET_DRAWER',
  ADD_NAMESPACE = 'ADD_NAMESPACE',
  REMOVE_NAMESPACE = 'REMOVE_NAMESPACE',
  UPDATE_NAMESPACE = 'UPDATE_NAMESPACE',
}

function defaultAppState(vuetify?: Vuetify): AppState {
  // eslint-disable-next-line no-restricted-globals
  const width = window.innerWidth ?? screen.width;
  let drawer = true;
  if (vuetify) {
    // eslint-disable-next-line no-unneeded-ternary
    drawer = (width < vuetify.framework.breakpoint.thresholds.sm) ? false : true;
  }

  return {
    darkMode: false,
    me: null,
    session: null,
    sessionToken: null,
    pendingUserId: null,
    pendingSessionId: null,
    drawer,
    currentNamespace: null,
    namespaces: [],
    isAdmin: false,
  };
}


export function newStore(storage: Storage, vuetify: Vuetify): Store<AppState> {
  const baseAppState = defaultAppState(vuetify);

  const storedDarkMode = storage.get(storage.keyDarkMode);
  if (storedDarkMode) {
    baseAppState.darkMode = storedDarkMode;
  }

  const storedToken = storage.get(storage.keyToken);
  if (storedToken) {
    baseAppState.sessionToken = storedToken;
  }

  const storedNamespace = storage.get(storage.keyCurrentNamespace);

  return new Store<AppState>({
    state: baseAppState,
    mutations: {
      [Mutation.SIGN_IN](state: AppState, params: SignedIn) {
        state.session = params.me.session;
        state.sessionToken = params.token;
        state.isAdmin = params.me.user.is_admin!;

        const namespaces: Namespace[] = [{
          id: params.me.user.namespace_id!,
          name: params.me.user.name,
          path: params.me.user.username,
          avatar_url: params.me.user.avatar_url,
        }];
        state.namespaces = namespaces;
        [state.currentNamespace] = namespaces;
        storage.set(storage.keyCurrentNamespace, state.currentNamespace.path);

        storage.set(storage.keyToken, state.sessionToken);

        state.pendingSessionId = null;
        state.pendingUserId = null;
      },
      [Mutation.INIT](state: AppState, me: Me) {
        state.session = me.session;
        state.me = me.user;
        state.isAdmin = me.user.is_admin!;

        const namespaces: Namespace[] = [{
          id: me.user.namespace_id!,
          name: me.user.name,
          path: me.user.username,
          avatar_url: me.user.avatar_url,
        }];

        me.groups.forEach((group) => {
          const namespace: Namespace = {
            id: group.namespace_id!,
            name: group.name,
            path: group.path,
            avatar_url: group.avatar_url,
          };
          if (storedNamespace === namespace.path) {
            state.currentNamespace = namespace;
          }
          namespaces.push(namespace);
        });
        state.namespaces = namespaces;
        if (!state.currentNamespace) {
          [state.currentNamespace] = namespaces;
          storage.set(storage.keyCurrentNamespace, state.currentNamespace.path);
        }
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
      [Mutation.SET_DRAWER](state: AppState, value: boolean) {
        state.drawer = value;
      },
      [Mutation.SET_CURRENT_NAMESPACE](state: AppState, namespace: Namespace) {
        state.currentNamespace = namespace;
        storage.set(storage.keyCurrentNamespace, namespace.path);
      },
      [Mutation.ADD_NAMESPACE](state: AppState, namespace: Namespace) {
        state.namespaces.push(namespace);
      },
      [Mutation.REMOVE_NAMESPACE](state: AppState, path: string) {
        state.namespaces = state.namespaces.filter((namespace) => namespace.path !== path);
        if (state.currentNamespace!.path === path) {
          [state.currentNamespace] = state.namespaces;
          storage.set(storage.keyCurrentNamespace, state.currentNamespace.path);
        }
      },
      [Mutation.UPDATE_NAMESPACE](state: AppState, namespace: Namespace) {
        state.namespaces = state.namespaces.map((n) => {
          if (n.id === namespace.id) {
            if (namespace.id === state.currentNamespace!.id) {
              state.currentNamespace = namespace;
              storage.set(storage.keyCurrentNamespace, namespace.path);
            }
            return namespace;
          }
          return n;
        });

        // profile updated
        if (namespace.id === state.me?.namespace_id) {
          state.me.name = namespace.name;
          state.me.username = namespace.path;
          state.me.avatar_url = namespace.avatar_url;
        }
      },
    },
    actions: {
    },
    modules: {
    },
  });
}
