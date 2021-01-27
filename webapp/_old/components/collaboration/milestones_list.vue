<template>
  <v-data-table
    :headers="headers"
    :items="milestones"
    item-key="id"
    hide-default-footer
    :loading="loading"
  >
    <template v-slot:no-data>
      <p class="text-center">
        No milestone.
      </p>
    </template>

    <template v-slot:item="{ item }" class="text-left">
      <tr @click="gotoMilestone(item)" class="bloom-pointer">
        <td>
          {{ item.title }}
        </td>
        <td>
          <span v-if="item.startDate">{{ calendar(item.startDate) }}</span>
          <span v-else>-</span>

        </td>
        <td>
          <span v-if="item.dueDate">{{ calendar(item.dueDate) }}</span>
          <span v-else>-</span>
        </td>
      </tr>
    </template>
  </v-data-table>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';
import { Milestone } from '@/api/graphql/model';
import { calendar } from '@/app/filters';


export default VueApp.extend({
  name: 'BMilestonesList',
  props: {
    milestones: {
      type: Array as PropType<Milestone[]>,
      required: true,
    },
    loading: {
      type: Boolean as PropType<boolean>,
      required: false,
      default: false,
    },
  },
  data() {
    return {
      headers: [
        {
          text: 'Title',
          align: 'start',
          sortable: true,
          value: 'title',
          width: '40%',
        },
        {
          text: 'Start date',
          align: 'start',
          sortable: true,
          value: 'startDate',
        },
        {
          text: 'Due date',
          align: 'start',
          sortable: true,
          value: 'dueDate',
        },
      ],
    };
  },
  computed: {
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
  },
  methods: {
    calendar,
    gotoMilestone(milestone: Milestone) {
      this.$router.push({ path: `/${this.projectFullPath}/-/milestones/${milestone.id}` });
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
