<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
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

    <v-row v-if="project" class="text-body-1">
      <v-col cols="12">
        <b>ID</b>: {{ project.id }}
      </v-col>

      <v-col cols="12">
        <b>Name</b>: {{ project.name }}
      </v-col>

      <v-col cols="12">
        <b>Path</b>: {{ project.path }}
      </v-col>

      <v-col cols="12">
        <b>Created at</b>: {{ date(project.createdAt) }}
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { Project } from '@/api/graphql/model';
import { VueApp } from '@/app/vue';
import date from '@/app/filters/date';

export default VueApp.extend({
  name: 'BAdminProjectsView',
  data() {
    return {
      loading: false,
      error: '',
      project: null as Project | null,
    };
  },
  computed: {
    projectId(): string {
      return this.$route.params.projectId;
    },
  },
  mounted() {
    this.fetchData();
  },
  methods: {
    date,
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.project = await this.$projectsService.adminFetchProject(this.projectId);
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
