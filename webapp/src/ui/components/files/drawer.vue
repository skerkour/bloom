<template>
  <v-navigation-drawer app clipped v-model="drawer"
    :mobile-breakpoint="this.$vuetify.breakpoint.thresholds.sm">
    <v-list nav dense class="text-left">

      <v-list-item exact link to="/files">
        <v-list-item-icon>
          <v-icon>mdi-folder</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Files</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-list-item exact link to="/files/trash">
        <v-list-item-icon>
          <v-icon>mdi-delete</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Trash</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

    </v-list>
  </v-navigation-drawer>
</template>


<script lang="ts">
import { Mutation } from '@/app/store';
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BFilesDrawer',
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
