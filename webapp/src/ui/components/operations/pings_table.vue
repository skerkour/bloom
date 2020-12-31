<template>
  <v-data-table
    :headers="headers"
    :items="pings"
    item-key="id"
    hide-default-footer
  >
    <template v-slot:no-data>
      <p class="text-center">
        No ping.
      </p>
    </template>

    <template v-slot:item="{ item }" class="text-left">
      <tr>
        <td>
          {{ calendar(item.createdAt) }}
        </td>
        <td>
          {{item.timeToFirstByte }} ms
        </td>
        <td>
          {{item.error }}
        </td>
      </tr>
    </template>
  </v-data-table>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { Ping } from '@/api/graphql/model';
import { PropType } from 'vue';
import { calendar } from '@/app/filters';


export default VueApp.extend({
  name: 'BPingsTable',
  props: {
    pings: {
      type: Array as PropType<Ping[]>,
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
          text: 'TTFB (Time To First Byte)',
          align: 'start',
          sortable: false,
          value: 'timeToFirstByte',
        },
        {
          text: 'Error',
          align: 'start',
          sortable: false,
          value: 'error',
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
