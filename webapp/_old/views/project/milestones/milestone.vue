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

    <v-row v-if="milestone">
      <b-milestone :milestone="milestone" @updated="onMilestoneUpdated" />
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { Milestone } from '@/api/graphql/model';
import { VueApp } from '@/app/vue';
import BMilestone from '@/ui/components/collaboration/milestone.vue';


export default VueApp.extend({
  name: 'BMilestoneView',
  components: {
    BMilestone,
  },
  data() {
    return {
      loading: false,
      error: '',
      milestone: null as Milestone | null,
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
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        const res = await this.$collaborationService.fetchMilestone(this.$route.params.milestoneId);
        this.milestone = res;
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    onMilestoneUpdated(milestone: Milestone) {
      this.milestone = milestone;
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
