<template>
  <v-navigation-drawer app clipped v-model="drawer"
    :mobile-breakpoint="this.$vuetify.breakpoint.thresholds.sm">
    <v-list nav dense class="text-left">

      <v-list-item exact link :to="`/${groupPath}`">
        <v-list-item-icon>
          <v-icon>mdi-home-outline</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Group overview</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-list-item exact link :to="`/groups/${groupPath}/-/members`">
        <v-list-item-icon>
          <v-icon>mdi-account-group</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Members</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-list-item exact link :to="`/groups/${groupPath}/-/billing`">
        <v-list-item-icon>
          <v-icon>mdi-credit-card</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Billing</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-list-item exact link :to="`/groups/${groupPath}/-/preferences`">
        <v-list-item-icon>
          <v-icon>mdi-cog</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Preferences</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

    </v-list>
  </v-navigation-drawer>
</template>


<script lang="ts">
import { Mutation } from '@/app/store';
import { VueApp } from '@/app/vue';


export default VueApp.extend({
  name: 'BGroupDrawer',
  computed: {
    groupPath(): string {
      const groupPath = this.$route.params.groupPath ?? this.$route.params.namespacePath;
      return groupPath;
    },
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
