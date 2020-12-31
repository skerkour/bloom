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
        <h2>Milestones</h2>
        <p>
          Milestones are containers for tickets. They allow you to organize work
          into a cohesive group, with an optional period of time.
        </p>
      </v-col>

      <v-col cols="12" class="ma-0 py-0">
        <v-app-bar dense color="white" flat>
          <v-spacer />
          <v-btn :to="`/${projectFullPath}/-/milestones/new`" color="success" depressed>
            <v-icon left>mdi-plus</v-icon>
            New milestone
          </v-btn>
        </v-app-bar>
      </v-col>

      <v-col cols="12" class="ma-0 py-0">
        <v-tabs v-model="tab">
          <v-tab>Open</v-tab>
          <v-tab>Closed</v-tab>
        </v-tabs>
        <v-tabs-items v-model="tab">
          <v-tab-item>
            <b-milestones-list :milestones="open" :loading="loading"/>
          </v-tab-item>
          <v-tab-item>
            <b-milestones-list :milestones="closed" :loading="loading"/>
          </v-tab-item>
        </v-tabs-items>
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { Milestone, Project } from '@/api/graphql/model';
import { VueApp } from '@/app/vue';
import BMilestonesList from '@/ui/components/collaboration/milestones_list.vue';


export default VueApp.extend({
  name: 'BMilestonesView',
  components: {
    BMilestonesList,
  },
  data() {
    return {
      tab: null,
      loading: false,
      error: '',
      project: null as Project | null,
    };
  },
  computed: {
    milestones(): Milestone[] {
      return this.project?.milestones ?? [];
    },
    open(): Milestone[] {
      return this.project?.milestones.filter((milestone: Milestone) => !milestone.closedAt) ?? [];
    },
    closed(): Milestone[] {
      return this.project?.milestones.filter((milestone: Milestone) => milestone.closedAt) ?? [];
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
        this.project = await this.$collaborationService.fetchMilestones(this.projectFullPath);
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
