<template>
  <v-app :dark="darkMode">
    <b-left-bar />

    <v-navigation-drawer app clipped v-model="drawer"
      :mobile-breakpoint="this.$vuetify.breakpoint.thresholds.sm"
      class="b-navigation-drawer">
      <b-group-drawer v-if="groupDrawer" />
      <b-user-preferences-drawer v-else-if="userPreferencesDrawer" />
      <b-tools-drawer v-else-if="toolsDrawer" />
      <b-admin-drawer v-else-if="adminDrawer" />
      <b-inbox-drawer v-else-if="inboxDrawer" />
      <b-files-drawer v-else-if="filesDrawer" />
    </v-navigation-drawer>

    <v-app-bar app color="#24292e" dark elevation="0" dense fixed clipped-left>
      <div class="b-app-bar-left-spacer" />

      <v-app-bar-nav-icon  @click.stop="toggleDrawer" v-if="showDrawerButton"/>
      <router-link to="/">
        <v-toolbar-title class="headline " to="/">
          Bloom
        </v-toolbar-title>
      </router-link>

      <v-btn to="/pricing" text v-if="!authenticated" class="ml-3 mr-1 d-none d-sm-flex">
        Pricing
      </v-btn>
      <v-btn to="/features" text v-if="!authenticated" class="mr-1 d-none d-sm-flex">
        Features
      </v-btn>
      <v-btn to="/about" text v-if="!authenticated" class="d-none d-sm-flex">
        About
      </v-btn>

      <v-spacer />

      <v-btn to="/login" text v-if="!authenticated">
        Sign In
      </v-btn>

      <v-btn to="/register" depressed color="success" v-if="!authenticated">
        Register for free
      </v-btn>

      <v-tooltip bottom>
        <template v-slot:activator="{ on, attrs }">
          <v-btn icon to="/contact" v-bind="attrs" v-on="on">
            <v-icon>mdi-help-circle-outline</v-icon>
          </v-btn>
        </template>
        <span>Help</span>
      </v-tooltip>

      <v-menu left bottom v-if="showAppBarAvatar" v-model="namespaceMenu">
        <template v-slot:activator="{ on }">
          <v-btn icon v-on="on">
          <v-avatar size="35">
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

                  <div class="text-xs-center">
                    <v-btn depressed small
                      @click.stop="goToNamespacePreferences(index, namespace)">
                      <v-icon small left>mdi-cog</v-icon>
                      Preferences
                    </v-btn>
                  </div>

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
    </v-app-bar>

    <v-main>
      <router-view />

      <b-footer v-if="showFooter" />
    </v-main>

    <b-bottom-nav-bar :show="showBottomNav" />
  </v-app>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';
import BGroupDrawer from '@/ui/components/groups/drawer.vue';
import BUserPreferencesDrawer from '@/ui/components/kernel/preferences_drawer.vue';
import BFooter from '@/ui/components/kernel/footer.vue';
import BToolsDrawer from '@/ui/components/tools/drawer.vue';
import BAdminDrawer from '@/ui/components/kernel/admin_drawer.vue';
import BInboxDrawer from '@/ui/components/inbox/drawer.vue';
import BFilesDrawer from '@/ui/components/files/drawer.vue';
import BBottomNavBar from '@/ui/components/kernel/bottom_nav_bar.vue';
import BLeftBar from '@/ui/components/kernel/left_bar.vue';
import { Mutation } from '@/app/store';
import { Namespace } from '@/domain/kernel/model';
import { apps } from '@/domain/kernel/apps';

export default VueApp.extend({
  name: 'BDefaultLayout',
  components: {
    BGroupDrawer,
    BUserPreferencesDrawer,
    BFooter,
    BToolsDrawer,
    BAdminDrawer,
    BInboxDrawer,
    BBottomNavBar,
    BFilesDrawer,
    BLeftBar,
  },
  computed: {
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
    darkMode(): boolean {
      return this.$store.state.darkMode;
    },
    authenticated(): boolean {
      return this.$store.state.session !== null;
    },
    avatarUrl(): string {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      return this.$store.state.currentNamespace!.avatar_url;
    },
    groupDrawer(): boolean {
      return this.showGroupDrawer();
    },
    userPreferencesDrawer(): boolean {
      return this.showUserPreferencesDrawer();
    },
    toolsDrawer(): boolean {
      return this.showToolsDrawer();
    },
    adminDrawer(): boolean {
      return this.showAdminDrawer();
    },
    inboxDrawer(): boolean {
      return this.showInboxDrawer();
    },
    filesDrawer(): boolean {
      return this.showFilesDrawer();
    },
    showFooter(): boolean {
      return this.$route.meta.auth === false && this.$route.path !== '/';
    },
    showAppBarAvatar(): boolean {
      return this.authenticated && this.$vuetify.breakpoint.smAndDown;
    },
    showDrawerButton(): boolean {
      // eslint-disable-next-line no-unneeded-ternary
      return (this.$store.state.session ? true : false)
        // eslint-disable-next-line max-len
        && (this.groupDrawer || this.userPreferencesDrawer || this.toolsDrawer || this.adminDrawer
          || this.inboxDrawer || this.filesDrawer);
    },
    showBottomNav(): boolean {
      return this.authenticated && this.$vuetify.breakpoint.smAndDown;
    },
    drawer: {
      get(): boolean {
        return this.$store.state.drawer
          && (this.groupDrawer || this.userPreferencesDrawer || this.toolsDrawer || this.adminDrawer
            || this.inboxDrawer || this.filesDrawer);
      },
      set(value: boolean) {
        this.$store.commit(Mutation.SET_DRAWER, value);
      },
    },
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    apps(): any[] {
      return apps;
    },
  },
  data() {
    return {
      namespaceMenu: false,
    };
  },
  watch: {
    $route(to) {
      if (!this.drawer && this.$vuetify.breakpoint.mdAndUp) {
        const path = to.path as string;
        if (this.showGroupDrawer(path)
          || this.showUserPreferencesDrawer(path)
          || this.showToolsDrawer(path) || this.showAdminDrawer(path)
          || this.showInboxDrawer(path) || this.showFilesDrawer(path)) {
          this.drawer = true;
        }
      }
    },
  },
  methods: {
    setCurrentNamespace(namespace: Namespace) {
      this.$store.commit(Mutation.SET_CURRENT_NAMESPACE, namespace);
      this.$router.push({ path: '/' });
    },
    goToNamespacePreferences(index: number, namespace: Namespace) {
      const route = index === 0 ? '/preferences' : `/groups/${namespace.path}/preferences`;
      this.$router.push({ path: route });
      this.namespaceMenu = false;
    },
    toggleDrawer() {
      this.$store.commit(Mutation.SET_DRAWER, !this.$store.state.drawer);
    },
    showGroupDrawer(path?: string): boolean {
      if (!path) {
        path = this.$route.path;
      }
      return path.startsWith('/groups') && path !== '/groups/new';
    },
    showUserPreferencesDrawer(path?: string): boolean {
      if (!path) {
        path = this.$route.path;
      }
      return path.startsWith('/preferences');
    },
    showToolsDrawer(path?: string): boolean {
      if (!path) {
        path = this.$route.path;
      }
      return path.startsWith('/tools');
    },
    showAdminDrawer(path?: string): boolean {
      if (!path) {
        path = this.$route.path;
      }
      return path.startsWith('/admin');
    },
    showInboxDrawer(path?: string): boolean {
      if (!path) {
        path = this.$route.path;
      }
      return path.startsWith('/inbox');
    },
    showFilesDrawer(path?: string): boolean {
      if (!path) {
        path = this.$route.path;
      }
      return path.startsWith('/files');
    },
  },
});
</script>


<style lang="scss" scoped>
@import '~vuetify/src/styles/styles.sass';

.b-app-bar-left-spacer {
  @media #{map-get($display-breakpoints, 'md-and-up')} {
    width: 72px;
  }
}

.v-main {
  @media #{map-get($display-breakpoints, 'md-and-up')} {
    margin-left: 72px;
  }
}

.b-navigation-drawer {
  @media #{map-get($display-breakpoints, 'md-and-up')} {
    margin-left: 72px;
  }
}

.v-app-bar {
  .headline {
    color: #fff;
  }
}

#b-app-bar-apps-card {
  width: 300px;
}

.v-main, .b-content {
  height: 100%;
}
</style>
