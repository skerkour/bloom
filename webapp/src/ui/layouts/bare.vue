<template>
  <v-app :dark="darkMode">
    <v-main>
      <v-container fluid>
        <router-view />
      </v-container>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BBareLayout',
  computed: {
    darkMode(): boolean {
      return this.$store.state.darkMode;
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
