<template>
  <v-container fluid class="pa-5">

    <v-row>
      <h2 class="headline">
        New Label
      </h2>
    </v-row>

    <v-row class="pa-5">
      <label-details :projectFullPath="projectFullPath" />
    </v-row>

  </v-container>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';
import LabelDetails from '@/ui/components/collaboration/label_details.vue';

export default VueApp.extend({
  name: 'ProjectNewLabelView',
  components: {
    LabelDetails,
  },
  computed: {
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
  },
});
</script>

<style lang="scss" scoped>
</style>
