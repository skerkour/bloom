<template>
  <v-app :dark="darkMode">
    <b-group-drawer v-if="groupDrawer" />
    <b-user-preferences-drawer v-if="userPreferencesDrawer" />
    <b-tools-drawer v-if="toolsDrawer" />
    <b-admin-drawer v-if="adminDrawer" />
    <b-inbox-drawer v-if="inboxDrawer" />
    <b-files-drawer v-if="filesDrawer" />
    <b-newsletter-drawer v-if="newsletterDrawer" />

    <v-app-bar app color="#24292e" dark elevation="0" dense fixed clipped-left>
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

      <v-menu offset-y close-on-content-click>
        <template v-slot:activator="{ on, attrs }">
          <v-btn icon v-if="showNavBarAppsButton" v-bind="attrs" v-on="on">
            <v-icon>mdi-apps</v-icon>
          </v-btn>
        </template>

        <v-card id="b-app-bar-apps-card">
          <v-card-text>
            <v-row justify="start">
              <v-col cols="4" v-for="(app, index) in apps" :key="index" class="text-center pa-0">
                <router-link :to="app.url">
                  <v-avatar class="blm-pointer"  size="52">
                    <v-icon :color="app.color">{{ app.icon }}</v-icon>
                  </v-avatar>
                  <p class="subtitle-1 font-weight-medium">
                    {{ app.name }}
                  </p>
                </router-link>
              </v-col>
            </v-row>
          </v-card-text>
        </v-card>
      </v-menu>

      <v-btn to="/login" text v-if="!authenticated">
        Sign In
      </v-btn>

      <v-btn to="/register" depressed color="success" v-if="!authenticated">
        Register for free
      </v-btn>

      <v-menu left bottom v-if="authenticated" v-model="namespaceMenu">
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

    <b-bottom-nav-bar />
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
import BNewsletterDrawer from '@/ui/components/newsletter/drawer.vue';
import { Mutation } from '@/app/store';
import { Namespace } from '@/domain/kernel/model';

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
    BNewsletterDrawer,
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
      return this.$route.path.startsWith('/groups') && this.$route.path !== '/groups/new';
    },
    userPreferencesDrawer(): boolean {
      return this.$route.path.startsWith('/preferences');
    },
    toolsDrawer(): boolean {
      return this.$route.path.startsWith('/tools');
    },
    adminDrawer(): boolean {
      return this.$route.path.startsWith('/admin');
    },
    inboxDrawer(): boolean {
      return this.$route.path.startsWith('/inbox');
    },
    filesDrawer(): boolean {
      return this.$route.path.startsWith('/files');
    },
    newsletterDrawer(): boolean {
      return this.$route.path.startsWith('/newsletter');
    },
    showFooter(): boolean {
      return this.$route.meta.auth === false && this.$route.path !== '/';
    },
    showNavBarAppsButton(): boolean {
      // TODO
      return this.authenticated; //  && this.$vuetify.breakpoint.mdAndUp;
    },
    showDrawerButton(): boolean {
      // eslint-disable-next-line no-unneeded-ternary
      return (this.$store.state.session ? true : false)
        // eslint-disable-next-line max-len
        && (this.groupDrawer || this.userPreferencesDrawer || this.toolsDrawer || this.adminDrawer
          || this.inboxDrawer || this.filesDrawer || this.newsletterDrawer);
    },
  },
  data() {
    return {
      namespaceMenu: false,
      apps: [
        {
          name: 'Inbox',
          icon: 'mdi-inbox',
          url: '/inbox',
          color: 'primary',
        },
        {
          name: 'Calendar',
          icon: 'mdi-calendar',
          url: '/calendar',
          color: 'purple lighten-1',
        },
        {
          name: 'Files',
          icon: 'mdi-folder',
          url: '/files',
          color: 'yellow',
        },
        {
          name: 'Analytics',
          icon: 'mdi-chart-line',
          url: '/analytics',
          color: 'success',
        },
        {
          name: 'Newsletter',
          icon: 'mdi-feather',
          url: '/newsletter',
          color: 'black',
        },
        {
          name: 'Tools',
          icon: 'mdi-hammer-wrench',
          url: '/tools',
          color: '',
        },
      ],
    };
  },
  methods: {
    setCurrentNamespace(namespace: Namespace) {
      this.$store.commit(Mutation.SET_CURRENT_NAMESPACE, namespace);
      this.$router.push({ path: '/' });
    },
    toggleDrawer() {
      this.$store.commit(Mutation.SET_DRAWER, !this.$store.state.drawer);
    },
    goToNamespacePreferences(index: number, namespace: Namespace) {
      const route = index === 0 ? '/preferences' : `/groups/${namespace.path}/preferences`;
      this.$router.push({ path: route });
      this.namespaceMenu = false;
    },
  },
});
</script>


<style lang="scss" scoped>
.v-app-bar {
  .headline {
    color: #fff;
  }
}

#b-app-bar-apps-card {
  width: 300px;
}
</style>
