<template>
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
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BFilesDrawer',
});
</script>


<style lang="scss" scoped>
</style>
