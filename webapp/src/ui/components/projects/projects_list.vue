<template>
  <div>
    <v-list two-line nav v-if="projects.length !== 0">
      <v-list-item
        v-for="project in projects"
        :key="project.id"
        :to="`/${namespacePath}/${project.path}`"
      >
        <v-list-item-avatar>
          <v-img :src="project.avatarUrl" />
          <!-- <v-avatar size="36" color="teal lighten-4" v-else>
            <span>{{ project.name[0] }}</span>
          </v-avatar> -->
        </v-list-item-avatar>

        <v-list-item-content>
          <v-list-item-title v-text="project.name"></v-list-item-title>
          <v-list-item-subtitle> {{ `${namespacePath}/${project.path}` }} </v-list-item-subtitle>
        </v-list-item-content>
      </v-list-item>
    </v-list>


    <v-list two-line nav v-else-if="projectsWithNamespace.length !== 0">
      <v-list-item
        v-for="projectWithNamespace in projectsWithNamespace"
        :key="projectWithNamespace.project.id"
        :to="`/${projectWithNamespace.namespace}/${projectWithNamespace.project.path}`"
      >
        <v-list-item-avatar>
          <v-img :src="projectWithNamespace.project.avatarUrl" />
        </v-list-item-avatar>

        <v-list-item-content>
          <v-list-item-title v-text="projectWithNamespace.project.name"></v-list-item-title>
          <v-list-item-subtitle>
            {{ `${projectWithNamespace.namespace}/${projectWithNamespace.project.path}` }}
          </v-list-item-subtitle>
        </v-list-item-content>
      </v-list-item>
    </v-list>
  </div>
</template>


<script lang="ts">
import { PropType } from 'vue';
import { VueApp } from '@/app/vue';
import { Project } from '@/api/graphql/model';
import { ProjectWithNamespace } from '@/domain/projects/service';

export default VueApp.extend({
  name: 'BProjectsList',
  props: {
    projects: {
      type: Array as PropType<Project[]>,
      default: () => [],
      required: false,
    },
    namespacePath: {
      type: String,
      default: '',
      required: false,
    },
    projectsWithNamespace: {
      type: Array as PropType<ProjectWithNamespace[]>,
      default: () => [],
      required: false,
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
