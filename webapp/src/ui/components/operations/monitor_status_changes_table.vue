<template>
  <v-data-table
    :headers="headers"
    :items="changes"
    item-key="id"
    hide-default-footer
  >
    <template v-slot:no-data>
      <p class="text-center">
        No change.
      </p>
    </template>

    <template v-slot:item="{ item }" class="text-left">
      <tr>
        <td>
          {{ calendar(item.createdAt) }}
        </td>
        <td>
          {{ item.from }}
        </td>
        <td>
          {{ item.to }}
        </td>
      </tr>
    </template>
  </v-data-table>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { MonitorStatusChange } from '@/api/graphql/model';
import { PropType } from 'vue';
import { calendar } from '@/app/filters';

export default VueApp.extend({
  name: 'BMonitorStatusChangesTable',
  props: {
    changes: {
      type: Array as PropType<MonitorStatusChange[]>,
      required: true,
    },
  },
  data() {
    return {
      headers: [
        {
          text: 'Date',
          align: 'start',
          sortable: false,
          value: 'createdAt',
        },
        {
          text: 'From',
          align: 'start',
          sortable: false,
          value: 'from',
        },
        {
          text: 'To',
          align: 'start',
          sortable: false,
          value: 'to',
        },
      ],
    };
  },
  methods: {
    calendar,
  },
});
</script>


<style lang="scss" scoped>
</style>
