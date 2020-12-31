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

    <v-row v-if="project">
      <ticket-details
        :projectFullPath="projectFullPath"
        :projectLabels="labels"
        :projectMilestones="milestones"
      />
    </v-row>
  </v-container>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';
import TicketDetails from '@/ui/components/collaboration/ticket_details.vue';
import { Project, Label, Milestone } from '@/api/graphql/model';

export default VueApp.extend({
  name: 'ProjectNewTicketView',
  components: {
    TicketDetails,
  },
  data() {
    return {
      error: '',
      loading: false,
      project: null as Project | null,
    };
  },
  computed: {
    labels(): Label[] {
      return this.project ? this.project.labels : [];
    },
    milestones(): Milestone[] {
      return this.project?.milestones.filter((milestone: Milestone) => !milestone.closedAt) ?? [];
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
        this.project = await this.$collaborationService
          .fetchLabelsAndMilestones(this.projectFullPath);
        console.log(this.project);
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
