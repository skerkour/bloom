<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>


    <v-row>
      <v-col cols="12" md="10" xl="8">
        <b-admin-projects-list :projects="projects" :loading="loading" />
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Project } from '@/api/graphql/model';
import { VueApp } from '@/app/vue';
import BAdminProjectsList from '@/ui/components/projects/admin_projects_list.vue';

export default VueApp.extend({
  name: 'BAdminProjectsView',
  components: {
    BAdminProjectsList,
  },
  data() {
    return {
      loading: false,
      error: '',
      projects: [] as Project[],
    };
  },
  mounted() {
    this.fetchData();
  },
  methods: {
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.projects = await this.$projectsService.adminFetchAllProjects();
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
