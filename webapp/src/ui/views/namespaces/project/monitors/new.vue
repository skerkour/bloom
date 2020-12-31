<template>
  <v-container fluid>
    <v-row>
      <b-monitor />
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import BMonitor from '@/ui/components/operations/monitor.vue';

export default VueApp.extend({
  name: 'BNewMonitorView',
  components: {
    BMonitor,
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
