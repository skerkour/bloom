<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center" v-if="loading" >
      <v-col cols="12" sm="10" md="8" xl="6">
        <v-progress-circular
          :size="50"
          color="primary"
          indeterminate
        />
      </v-col>
    </v-row>


    <v-row justify="center" v-if="loaded">
      <v-col cols="12" md="8" lg="6" xl="4" >
        <v-tabs v-model="tab">
          <v-tab>Projects</v-tab>
          <v-tab>Groups</v-tab>
        </v-tabs>
        <v-tabs-items v-model="tab">
          <v-tab-item>
            <v-col cols="12" md="8">
              <v-btn color="success" to="/projects/new" depressed :disabled="groups.length === 0">
                <v-icon left>mdi-plus</v-icon>
                New project
              </v-btn>
              <v-alert
                border="top"
                colored-border
                type="info"
                elevation="1"
                v-if="groups.length === 0"
              >
                You need to create a group before creating a project.
              </v-alert>
            </v-col>
            <v-col cols="12" md="8">
              <b-projects-list :projectsWithNamespace="projects" />
            </v-col>
          </v-tab-item>

          <v-tab-item>
            <p class="py-1">
              With Groups you can organize related projects together and grant many people access
              to several projects at once.
            </p>
            <div class="pb-4">
              <v-btn to="/groups/new" color="success" depressed>
                <v-icon left>mdi-plus</v-icon>
                New group
              </v-btn>
            </div>

            <b-groups-list :groups="groups" v-if="groups.length !== 0"/>
            <div v-else>
              <p>No group yet</p>
            </div>
          </v-tab-item>
        </v-tabs-items>
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
/* eslint-disable arrow-body-style */
import { VueApp } from '@/app/vue';
import { Group, Project } from '@/api/graphql/model';
import BGroupsList from '@/ui/components/groups/groups_list.vue';
// import BProjectsList from '@/ui/components/projects/projects_list.vue';
import { ProjectWithNamespace } from '@/domain/projects/service';


export default VueApp.extend({
  name: 'BHome',
  components: {
    BGroupsList,
    // BProjectsList,
  },
  data() {
    return {
      error: '',
      loading: false,
      loaded: false,
      tab: 0,
      groups: [] as Group[],
      projects: [] as ProjectWithNamespace[],
    };
  },
  created() {
    this.fetchGroups();
  },
  methods: {
    async fetchGroups(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.groups = await this.$usersService.fetchMyGroupsWithProjects();
        this.groups.forEach((group: Group) => {
          const projects = group.projects.map((project: Project) => {
            return {
              namespace: group.path,
              project,
            } as ProjectWithNamespace;
          });
          this.projects = this.projects.concat(projects);
        });
        this.loaded = true;
        if (this.groups.length === 0) {
          this.tab = 1;
        }
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
