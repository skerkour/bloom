<template>
  <div>
  <v-list two-line nav>
    <v-list-item
      v-for="group in groups"
      :key="group.id"
      :to="`/${group.path}`"
    >
      <v-list-item-avatar>
        <v-img :src="group.avatarUrl" v-if="group.avatarUrl" />
        <v-avatar size="36" color="teal lighten-4" v-else>
          <span>{{ group.name[0] }}</span>
        </v-avatar>
      </v-list-item-avatar>

      <v-list-item-content>
        <v-list-item-title v-text="group.name"></v-list-item-title>
        <v-list-item-subtitle v-text="group.path"></v-list-item-subtitle>
      </v-list-item-content>
    </v-list-item>
  </v-list>

    <!-- <v-list-group no-action v-for="group in groups" :key="group.id" :value="true"
      :append-icon="null">
      <v-list-item slot='activator' :to="`/${group.path}`">
        <v-list-item-avatar>
          <v-img :src="group.avatarUrl" />
        </v-list-item-avatar>
        <v-list-item-title class="text-h5">{{ group.name }}</v-list-item-title>
      </v-list-item>
      <v-row justify="center">
        <v-col cols="10">
          <v-subheader>Projects</v-subheader>
          <b-projects-list :namespacePath="group.path" :projects="group.projects" />
        </v-col>
      </v-row>
    </v-list-group> -->
  </div>
</template>


<script lang="ts">
import { PropType } from 'vue';
import { VueApp } from '@/app/vue';
import { Group } from '@/api/graphql/model';
// import BProjectsList from '@/ui/components/projects/projects_list.vue';


export default VueApp.extend({
  name: 'BGroupsList',
  components: {
    // BProjectsList,
  },
  props: {
    groups: {
      type: Array as PropType<Group[]>,
      default: [],
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
