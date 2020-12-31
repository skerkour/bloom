<template>
  <v-data-table
    :headers="headers"
    :items="projects"
    item-key="id"
    :items-per-page="50"
    :loading="loading"
  >
    <template v-slot:no-data>
      <p class="text-center">
        No project.
      </p>
    </template>

    <template v-slot:item="{ item }" class="text-left">
      <tr @click="gotoProject(item)" class="bloom-pointer">
        <td>
          {{ item.id }}
        </td>
        <td>
          {{ date(item.createdAt) }}
        </td>
        <td>
          {{ item.name }}
        </td>
        <td>
          {{ item.path }}
        </td>
      </tr>
    </template>
  </v-data-table>
</template>


<script lang="ts">
import { PropType } from 'vue';
import { VueApp } from '@/app/vue';
import { Project } from '@/api/graphql/model';
import date from '@/app/filters/date';

export default VueApp.extend({
  name: 'BAdminProjectsList',
  props: {
    projects: {
      type: Array as PropType<Project[]>,
      default: [],
    },
    loading: {
      type: Boolean as PropType<boolean>,
      default: false,
      required: true,
    },
  },
  data() {
    return {
      headers: [
        {
          text: 'ID',
          align: 'start',
          sortable: false,
          value: 'id',
        },
        {
          text: 'Created At',
          align: 'start',
          sortable: true,
          value: 'createdAt',
        },
        {
          text: 'Name',
          align: 'start',
          sortable: true,
          value: 'name',
        },
        {
          text: 'Path',
          align: 'start',
          sortable: true,
          value: 'path',
        },
      ],
    };
  },
  methods: {
    date,
    gotoProject(project: Project) {
      this.$router.push({ path: `/admin/projects/${project.id}` });
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
