<template>
  <v-container fluid>

    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center">
      <v-col cols="12">
        <h2>Monitors</h2>
        <p>
          Monitor your Websites and APIs and get notified when something go wrong.
        </p>
      </v-col>

      <v-col cols="12" class="ma-0 py-0">
        <v-app-bar dense color="white" flat>
          <v-spacer />
          <v-btn :to="`/status/${projectFullPath}`" target="_blank"
            rel="noopener" color="primary" outlined class="mr-3">
            <v-icon left>mdi-list-status</v-icon>
            Go to status page
          </v-btn>
          <v-btn :to="`/${projectFullPath}/-/monitors/new`" color="success" depressed>
            <v-icon left>mdi-plus</v-icon>
            New monitor
          </v-btn>
        </v-app-bar>
      </v-col>

      <v-col cols="12" class="ma-0 py-0">
        <b-monitors-list :monitors="monitors" :loading="loading" />
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import BMonitorsList from '@/ui/components/operations/monitors_list.vue';
import { Project, Monitor } from '@/api/graphql/model';

export default VueApp.extend({
  name: 'BMonitorsView',
  components: {
    BMonitorsList,
  },
  data() {
    return {
      loading: false,
      error: '',
      project: null as Project | null,
    };
  },
  computed: {
    monitors(): Monitor[] {
      return this.project?.monitors ?? [];
    },
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
  },
  created() {
    this.fetchData();
  },
  methods: {
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.project = await this.$operationsService.fetchMonitors(this.projectFullPath);
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
