<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center" class="text-center" v-if="loading">
      <v-col cols="12" sm="10" md="8" xl="6">
        <v-progress-circular
          :size="50"
          color="primary"
          indeterminate
        />
      </v-col>
    </v-row>

    <v-row v-if="monitor">
      <b-monitor
        :monitor="monitor"
        @updated="onMonitorUpdated"
      />
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { Monitor } from '@/api/graphql/model';
import BMonitor from '@/ui/components/operations/monitor.vue';


export default VueApp.extend({
  name: 'BMonitorView',
  components: {
    BMonitor,
  },
  data() {
    return {
      loading: false,
      error: '',

      monitor: null as Monitor | null,
    };
  },
  computed: {
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
  },
  created() {
    this.fetchData();
  },
  methods: {
    onMonitorUpdated(updatedMonitor: Monitor) {
      this.monitor = updatedMonitor;
    },
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.monitor = await this.$operationsService.fetchMonitor(this.$route.params.monitorId);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
