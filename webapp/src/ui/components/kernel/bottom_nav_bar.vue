<template>
  <v-bottom-navigation fixed v-if="show" grow color="primary" app>
    <v-btn to="/inbox" text fab>
      <span>Inbox</span>
      <v-icon>mdi-inbox</v-icon>
    </v-btn>

    <v-btn to="/calendar" text fab>
      <span>Calendar</span>
      <v-icon>mdi-calendar</v-icon>
    </v-btn>

    <v-btn to="/files" text fab>
      <span>Files</span>
      <v-icon>mdi-folder</v-icon>
    </v-btn>

    <v-btn to="/apps" text fab>
      <span>More</span>
      <v-icon>mdi-apps</v-icon>
    </v-btn>

  </v-bottom-navigation>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';


export default VueApp.extend({
  name: 'BBottomNavBar',
  props: {
    show: {
      type: Boolean as PropType<boolean>,
      required: true,
    },
  },
});
</script>

<style lang="scss" scoped>
</style>
