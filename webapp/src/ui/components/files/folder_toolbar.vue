<template>
  <v-app-bar flat color="white">
      <v-toolbar-title>
        <b-file-path :file="folder" />
      </v-toolbar-title>

      <v-spacer />

      <v-menu offset-y>
        <template v-slot:activator="{ on }">
        <v-btn color="success" depressed v-on="on">
          <v-icon left>mdi-plus</v-icon>
          New
        </v-btn>
      </template>

      <v-list>
        <v-list-item @click="openNewFolderDialog">
          <v-list-item-action>
            <v-icon>mdi-folder-plus</v-icon>
          </v-list-item-action>
          <v-list-item-content>
            <v-list-item-title>Create folder</v-list-item-title>
          </v-list-item-content>
        </v-list-item>

        <v-divider />

        <v-list-item @click="uploadFiles">
          <v-list-item-action>
            <v-icon>mdi-file-upload</v-icon>
          </v-list-item-action>
          <v-list-item-content>
            <v-list-item-title>Upload Files</v-list-item-title>
          </v-list-item-content>
        </v-list-item>

      </v-list>


      </v-menu>

  </v-app-bar>
</template>


<script lang="ts">
import { PropType } from 'vue';
import { VueApp } from '@/app/vue';
import { File } from '@/domain/files/model';
import BFilePath from './file_path.vue';

export default VueApp.extend({
  name: 'BFolderToolbar',
  components: {
    BFilePath,
  },
  props: {
    folder: {
      type: Object as PropType<File>,
      required: true,
    },
  },
  methods: {
    openNewFolderDialog() {
      this.$emit('new-folder-clicked');
    },
    uploadFiles() {
      this.$emit('upload-files-clicked');
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
