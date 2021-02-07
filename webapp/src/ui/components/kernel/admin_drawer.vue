<template>
  <v-navigation-drawer app clipped v-model="drawer"
    :mobile-breakpoint="this.$vuetify.breakpoint.thresholds.sm">
    <v-list nav dense class="text-left">

      <v-list-item exact link to="/admin">
        <v-list-item-icon>
          <v-icon>mdi-view-dashboard</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Dashboard</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-list-item exact link to="/admin/users">
        <v-list-item-icon>
          <v-icon>mdi-account</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Users</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-list-item exact link to="/admin/groups">
        <v-list-item-icon>
          <v-icon>mdi-account-group</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Groups</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

    </v-list>
  </v-navigation-drawer>
</template>


<script lang="ts">
import { Mutation } from '@/app/store';
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BAdminDrawer',
  computed: {
    drawer: {
      get(): boolean {
        return this.$store.state.drawer;
      },
      set(value: boolean) {
        this.$store.commit(Mutation.SET_DRAWER, value);
      },
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
