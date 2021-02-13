<template>
  <v-navigation-drawer app clipped v-model="drawer"
    :mobile-breakpoint="this.$vuetify.breakpoint.thresholds.sm">
    <v-list nav dense class="text-left">

      <v-list-item exact link to="/inbox" color="primary">
        <v-list-item-icon>
          <v-icon color="primary">mdi-inbox</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Inbox</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-list-item exact link to="/inbox/done" color="success">
        <v-list-item-icon>
          <v-icon color="success">mdi-check</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Done</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-divider />

      <v-list-item exact link to="/inbox/trash">
        <v-list-item-icon>
          <v-icon>mdi-delete</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Trash</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-list-item exact link to="/inbox/spam">
        <v-list-item-icon>
          <v-icon>mdi-alert-octagon</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Spam</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-list-item exact link to="/inbox/contacts">
        <v-list-item-icon>
          <v-icon>mdi-account-box</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Contacts</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-divider />

      <v-list-item exact link to="/inbox/chatbox">
        <v-list-item-icon>
          <v-icon>mdi-chat-processing-outline</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Chatbox</v-list-item-title>
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
