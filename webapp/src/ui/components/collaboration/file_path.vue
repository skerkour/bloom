<template>
  <v-breadcrumbs :items="items">
    <template v-slot:divider>
      <v-icon>mdi-chevron-right</v-icon>
    </template>
  </v-breadcrumbs>
</template>


<script lang="ts">
import { PropType } from 'vue';
import { VueApp } from '@/app/vue';
import { File, FilePath } from '@/api/graphql/model';

export default VueApp.extend({
  name: 'BFilePath',
  props: {
    file: {
      type: Object as PropType<File>,
      required: true,
    },
    projectFullPath: {
      type: String as PropType<string>,
      required: true,
    },
  },
  computed: {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    items(): any[] {
      return this.file.path.map((pathItem: FilePath) => {
        const name = pathItem.name === '__ROOT__' ? 'My files' : pathItem.name;
        return {
          text: name,
          disabled: false,
          to: `/${this.projectFullPath}/-/files/${pathItem.id}`,
        };
      });
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
