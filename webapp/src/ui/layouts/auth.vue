<template>
  <v-app :dark="darkMode">
    <v-app-bar app color="white" elevation="0" dense>
      <router-link to="/">
        <v-toolbar-title class="headline">
          Bloom
        </v-toolbar-title>
      </router-link>

      <!-- <v-spacer />

      <v-tooltip>
        <template v-slot:activator="{ on }">
          <v-btn icon to="/help" v-on="on">
            <v-icon>mdi-help-circle-outline</v-icon>
          </v-btn>
        </template>
        <span>Help</span>
      </v-tooltip> -->
    </v-app-bar>

    <v-main>
      <v-container fluid>
        <v-row align="start" justify="center" class="mt-3">
          <v-tabs fixed-tabs class="mb-5" v-if="showTabs">
            <v-tab to="/register">Create account</v-tab>
            <v-tab to="/login">Sign in</v-tab>
          </v-tabs>
          <router-view />
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BAuthLayout',
  computed: {
    darkMode(): boolean {
      return this.$store.state.darkMode;
    },
    showTabs(): boolean {
      if (this.$route.path === '/login' || this.$route.path === '/register') {
        return true;
      }
      return false;
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
