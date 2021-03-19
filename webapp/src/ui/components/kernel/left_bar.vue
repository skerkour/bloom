<template>
  <div>
    <v-navigation-drawer
      mini-variant
      permanent
      absolute
      fill-height
      color="#24292e"
      class="b-left-bar overflow-y-auto"
      v-if="authenticated"
    >

      <v-list-item>
        <v-list-item-avatar
          class="bloom-pointer"
        >
          <v-menu left bottom v-if="authenticated" v-model="namespaceMenu">
            <template v-slot:activator="{ on }">
              <v-btn icon v-on="on">
              <v-avatar>
                <v-img :src="avatarUrl" />
              </v-avatar>
              </v-btn>
            </template>

            <v-card>
              <v-list>
                <v-list-item-group :value="currentNamespaceIndex" color="primary">
                  <v-list-item class="pt-3 pb-3"
                    v-for="(namespace, index) in $store.state.namespaces" :key="namespace.id"
                    color="primary" @click="setCurrentNamespace(namespace, index)">
                    <v-list-item-avatar>
                      <img :src="namespace.avatar_url">
                    </v-list-item-avatar>

                    <v-list-item-content>
                      <v-list-item-title>
                        {{ namespace.name }}</v-list-item-title>
                      <v-list-item-subtitle>
                        @{{ namespace.path }}</v-list-item-subtitle>
                    </v-list-item-content>
                  </v-list-item>
                </v-list-item-group>

                <v-divider />

                <v-list-item to="/groups/new">
                  <v-list-item-icon>
                    <v-icon>mdi-plus</v-icon>
                  </v-list-item-icon>
                  <v-list-item-content>
                    <v-list-item-title>New Workspace</v-list-item-title>
                  </v-list-item-content>
                </v-list-item>

                <v-divider v-if="$store.state.isAdmin" />

                <v-list-item to="/admin" v-if="$store.state.isAdmin">
                  <v-list-item-icon>
                    <v-icon>mdi-shield-account</v-icon>
                  </v-list-item-icon>
                  <v-list-item-content>
                    <v-list-item-title>Admin</v-list-item-title>
                  </v-list-item-content>
                </v-list-item>
              </v-list>
            </v-card>
          </v-menu>
        </v-list-item-avatar>
      </v-list-item>

      <v-list-item>
        <v-list-item-avatar
          class="bloom-pointer"
          @click="goToCurrentNamespacePreferences"
        >
          <v-tooltip bottom>
            <template v-slot:activator="{ on }">
              <v-avatar v-on="on" color="white">
                <v-icon>
                  mdi-cog
                </v-icon>
              </v-avatar>
            </template>
            <span>Preferences</span>
          </v-tooltip>
        </v-list-item-avatar>
      </v-list-item>

      <v-list-item>
        <v-list-item-avatar
          class="bloom-pointer"
          @click="goToCurrentNamespaceBilling"
        >
          <v-tooltip bottom>
            <template v-slot:activator="{ on }">
              <v-avatar v-on="on" color="white">
                <v-icon>
                  mdi-credit-card
                </v-icon>
              </v-avatar>
            </template>
            <span>Billing</span>
          </v-tooltip>
        </v-list-item-avatar>
      </v-list-item>

      <v-divider class="b-left-bar-divider" />

      <v-list-item
        v-for="(app, index) in apps"
        :key="index"
      >
        <v-list-item-avatar
          class="bloom-pointer"
          @click="goto(app.url)"
        >
          <v-tooltip bottom>
            <template v-slot:activator="{ on }">
              <v-img
                :src="app.icon"
                v-on="on"
              />
            </template>
            <span>{{ app.name }}</span>
          </v-tooltip>
        </v-list-item-avatar>
      </v-list-item>

    </v-navigation-drawer>
  </div>
</template>


<script lang="ts">
/* eslint-disable @typescript-eslint/no-explicit-any */
import { Mutation } from '@/app/store';
import { VueApp } from '@/app/vue';
import { apps } from '@/domain/kernel/apps';
import { Namespace } from '@/domain/kernel/model';


export default VueApp.extend({
  name: 'BLeftBar',
  computed: {
    authenticated(): boolean {
      return this.$store.state.session !== null;
    },
    avatarUrl(): string {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      return this.$store.state.currentNamespace!.avatar_url;
    },
    currentNamespaceIndex(): number {
      const currentPath = this.$store.state.currentNamespace?.path;
      // eslint-disable-next-line no-plusplus
      for (let i = 0; i < this.$store.state.namespaces.length; i++) {
        if (this.$store.state.namespaces[i].path === currentPath) {
          return i;
        }
      }
      return 0;
    },
    apps(): any[] {
      return apps;
    },
  },
  data() {
    return {
      namespaceMenu: false,
    };
  },
  methods: {
    goto(path: string) {
      // eslint-disable-next-line @typescript-eslint/no-empty-function
      this.$router.push({ path }).catch(() => {});
    },
    setCurrentNamespace(namespace: Namespace) {
      this.$store.commit(Mutation.SET_CURRENT_NAMESPACE, namespace);
      this.$router.push({ path: '/' });
    },
    goToCurrentNamespacePreferences() {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const namespace = this.$store.state.currentNamespace!;
      const route = namespace.path === this.$store.state.me?.username ? '/preferences' : `/groups/${namespace.path}/preferences`;
      this.$router.push({ path: route });
      this.namespaceMenu = false;
    },
    goToCurrentNamespaceBilling() {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const namespace = this.$store.state.currentNamespace!;
      const route = namespace.path === this.$store.state.me?.username ? '/preferences/billing' : `/groups/${namespace.path}/billing`;
      this.$router.push({ path: route });
      this.namespaceMenu = false;
    },
  },
});
</script>


<style lang="scss" scoped>
@import '~vuetify/src/styles/styles.sass';

.b-left-bar {
  @media #{map-get($display-breakpoints, 'sm-and-down')} {
    width: 0px;
    display: none;
    height: 0px;
  }
  @media #{map-get($display-breakpoints, 'md-and-up')} {
    z-index: 100;
    overflow-y: auto;
    width: 72px !important;
    height: 100vh;
    position: fixed !important;
  }
}

.b-left-bar-divider {
  border-color: #7b7b7b !important;
}
</style>
