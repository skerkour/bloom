<template>
  <v-app :dark="darkMode">
    <b-project-drawer v-if="projectDrawer" />
    <b-group-drawer v-else-if="groupDrawer" />
    <b-user-preferences-drawer v-if="userPreferencesDrawer" />
    <b-tools-drawer v-if="toolsDrawer" />
    <b-admin-drawer v-if="adminDrawer" />

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

      <v-menu left bottom v-if="authenticated">
        <template v-slot:activator="{ on }">
          <v-btn icon v-on="on">
            <v-icon>mdi-plus</v-icon>
          </v-btn>
        </template>

        <v-list>
          <v-list-item to="/projects/new">
            <v-list-item-title>New project</v-list-item-title>
          </v-list-item>
          <v-list-item to="/groups/new">
            <v-list-item-title>New group</v-list-item-title>
          </v-list-item>
        </v-list>
      </v-menu>

      <v-btn to="/login" text v-if="!authenticated">
        Sign In
      </v-btn>

      <v-btn to="/register" depressed color="success" v-if="!authenticated">
        Register for free
      </v-btn>

      <v-menu left bottom v-if="authenticated">
        <template v-slot:activator="{ on }">
          <v-btn icon v-on="on">
          <v-avatar size="35">
            <v-img :src="avatarUrl" />
          </v-avatar>
          </v-btn>
        </template>

        <v-card>
          <v-list>
            <v-list-item :to="`/${$store.state.me.username}`" class="pt-3 pb-3">
              <v-list-item-avatar>
                <img :src="avatarUrl">
              </v-list-item-avatar>

              <v-list-item-content>
                <v-list-item-title>{{ $store.state.me.name }}</v-list-item-title>
                <v-list-item-subtitle>@{{ $store.state.me.username }}</v-list-item-subtitle>
              </v-list-item-content>
            </v-list-item>

            <v-divider />

            <v-list-item to="/tools">
              <v-list-item-icon>
                <v-icon>mdi-hammer-wrench</v-icon>
              </v-list-item-icon>
              <v-list-item-content>
                <v-list-item-title>Tools</v-list-item-title>
              </v-list-item-content>
            </v-list-item>
            <v-list-item to="/preferences">
              <v-list-item-icon>
                <v-icon>mdi-cog</v-icon>
              </v-list-item-icon>
              <v-list-item-content>
                <v-list-item-title>Preferences</v-list-item-title>
              </v-list-item-content>
            </v-list-item>
            <v-list-item to="/admin" v-if="$store.state.me.isAdmin">
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
  </v-app>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';
import BProjectDrawer from '@/ui/components/projects/drawer.vue';
import BGroupDrawer from '@/ui/components/groups/drawer.vue';
import BUserPreferencesDrawer from '@/ui/components/kernel/preferences_drawer.vue';
import BFooter from '@/ui/components/kernel/footer.vue';
import BToolsDrawer from '@/ui/components/tools/drawer.vue';
import BAdminDrawer from '@/ui/components/admin/drawer.vue';
import { Mutation } from '@/app/store';

export default VueApp.extend({
  name: 'BDefaultLayout',
  components: {
    BProjectDrawer,
    BGroupDrawer,
    BUserPreferencesDrawer,
    BFooter,
    BToolsDrawer,
    BAdminDrawer,
  },
  computed: {
    darkMode(): boolean {
      return this.$store.state.darkMode;
    },
    authenticated(): boolean {
      return this.$store.state.session !== null;
    },
    avatarUrl(): string {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      return this.$store.state.me!.avatarUrl;
    },
    projectDrawer(): boolean {
      if (this.$route.meta.projectDrawer) {
        return true;
      }
      return false;
    },
    groupDrawer(): boolean {
      if (this.$route.meta.groupDrawer || this.$store.state.namespaceIsGroup) {
        return true;
      }
      return false;
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
    showFooter(): boolean {
      return this.$route.meta.auth === false && this.$route.path !== '/';
    },
    showDrawerButton(): boolean {
      // eslint-disable-next-line no-unneeded-ternary
      return (this.$store.state.me ? true : false)
        // eslint-disable-next-line max-len
        && (this.projectDrawer || this.groupDrawer || this.userPreferencesDrawer || this.toolsDrawer || this.adminDrawer);
    },
  },
  methods: {
    toggleDrawer() {
      this.$store.commit(Mutation.SET_DRAWER, !this.$store.state.drawer);
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
</style>
