<template>
  <span>
    <v-icon v-if="isFolder(file)">mdi-folder</v-icon>
    <v-icon v-else>mdi-file</v-icon>
  </span>
</template>


<script lang="ts">
import { PropType } from 'vue';
import { VueApp } from '@/app/vue';
import { File } from '@/domain/files/model';

export default VueApp.extend({
  name: 'BFileIcon',
  props: {
    file: {
      type: Object as PropType<File>,
      required: true,
    },
  },
  methods: {
    isFolder(file: File): boolean {
      return file.type === this.$filesService.fileTypeFolder;
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
