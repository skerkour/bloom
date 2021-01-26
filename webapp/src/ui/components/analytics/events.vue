<template>
  <v-card flat>
    <v-card-text>
      <v-data-table
        :headers="headers"
        :items="events"
        item-key="eventName"
        hide-default-footer
      >
        <template v-slot:no-data>
          <p class="text-center">
            No data.
          </p>
        </template>
        <template v-slot:item="{ item }" class="text-left">
          <tr>
            <td>
              {{ item.event_name }}
            </td>
            <td>
              {{ item.views }}
            </td>
            <td>
              {{ item.visitors }}
            </td>
          </tr>
        </template>

      </v-data-table>
    </v-card-text>
  </v-card>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { AnalyticsEvent } from '@/domain/analytics/model';
import { PropType } from 'vue';


export default VueApp.extend({
  name: 'BAnalyticsEvents',
  props: {
    events: {
      type: Array as PropType<AnalyticsEvent[]>,
      required: true,
    },
  },
  data() {
    return {
      selected: [],
      headers: [
        {
          text: 'Event',
          align: 'start',
          sortable: true,
          value: 'event_name',
        },
        {
          text: 'Views',
          align: 'start',
          sortable: true,
          value: 'views',
        },
        {
          text: 'Uniques',
          align: 'start',
          sortable: true,
          value: 'visitors',
        },
      ],
    };
  },
});
</script>


<style lang="scss" scoped>
</style>
