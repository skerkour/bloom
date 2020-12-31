<template>
  <v-data-table
    :headers="headers"
    :items="history"
    item-key="id"
    hide-default-footer
    :loading="loading"
  >
    <template v-slot:no-data>
      <p class="text-center">
        No history.
      </p>
    </template>

    <template v-slot:item="{ item }" class="text-left">
      <tr>
        <td>
          {{ item.bot.name }}
        </td>
        <td>
          <span>{{ calendar(item.createdAt) }}</span>
        </td>
        <td>
          <span v-if="item.completedAt">{{ calendar(item.completedAt) }}</span>
          <span v-else>-</span>
        </td>
        <td>
          {{ item.status }}
        </td>
      </tr>
    </template>
  </v-data-table>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';
import { BotExecution } from '@/api/graphql/model';
import { calendar } from '@/app/filters';


export default VueApp.extend({
  name: 'BBotsHistory',
  props: {
    history: {
      type: Array as PropType<BotExecution[]>,
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
          text: 'Bot',
          align: 'start',
          sortable: true,
          value: 'name',
          width: '40%',
        },
        {
          text: 'Started at',
          align: 'start',
          sortable: true,
          value: 'createdAt',
        },
        {
          text: 'Completed at',
          align: 'start',
          sortable: true,
          value: 'completedAt',
        },
        {
          text: 'Status',
          align: 'start',
          sortable: true,
          value: 'status',
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
