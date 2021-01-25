<template>
  <v-navigation-drawer app clipped v-model="drawer"
    :mobile-breakpoint="this.$vuetify.breakpoint.thresholds.sm">
    <v-list nav dense class="text-left">

      <v-list-item to="/inbox" color="primary">
        <v-list-item-icon>
          <v-icon>mdi-inbox</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Inbox</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-list-item to="/inbox/archive">
        <v-list-item-icon>
          <v-icon color="success">mdi-check</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Done</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-divider />

      <v-list-item to="/spam">
        <v-list-item-icon>
          <v-icon>mdi-alert-octagon</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Spam</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-list-item to="/trash">
        <v-list-item-icon>
          <v-icon>mdi-delete</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Trash</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-divider />

      <v-list-item exact link to="/inbox/chatbox">
        <v-list-item-icon>
          <v-icon>mdi-chat-processing-outline</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Chabtox</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

    </v-list>
  </v-navigation-drawer>
</template>


<script lang="ts">
import { Mutation } from '@/app/store';
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BInboxDrawer',
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
