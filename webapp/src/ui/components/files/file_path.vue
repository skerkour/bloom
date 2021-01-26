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
import { File, FilePath } from '@/domain/files/model';

export default VueApp.extend({
  name: 'BFilePath',
  props: {
    file: {
      type: Object as PropType<File>,
      required: true,
    },
  },
  computed: {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    items(): any[] {
      return this.file.path.map((pathItem: FilePath) => {
        const name = pathItem.name === this.$filesService.rootFileName ? 'My files' : pathItem.name;
        return {
          text: name,
          disabled: false,
          to: `/files/${pathItem.id}`,
        };
      });
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
