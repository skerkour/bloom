<template>
  <v-bottom-navigation fixed v-if="show">
    <v-btn to="/inbox">
      <span>Inbox</span>
      <v-icon>mdi-inbox</v-icon>
    </v-btn>

    <v-btn to="/files">
      <span>Files</span>
      <v-icon>mdi-folder</v-icon>
    </v-btn>

    <v-btn to="/analytics">
      <span>Analytics</span>
      <v-icon>mdi-chart-line</v-icon>
    </v-btn>

    <v-btn to="/apps">
      <span>All</span>
      <v-icon>mdi-apps</v-icon>
    </v-btn>

  </v-bottom-navigation>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';


export default VueApp.extend({
  name: 'BBottomNavBar',
  computed: {
    authenticated(): boolean {
      return this.$store.state.session !== null;
    },
    show(): boolean {
      // TODO
      // return this.authenticated && this.$vuetify.breakpoint.smAndDown;
      return false;
    },
  },
});
</script>

<style lang="scss" scoped>
</style>
