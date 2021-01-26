<template>
  <v-card flat>
    <v-card-text>
      <v-data-table
        :headers="headers"
        :items="devices"
        item-key="deviceType"
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
              {{ item.device_type }}
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
import { Device } from '@/domain/analytics/model';
import { PropType } from 'vue';


export default VueApp.extend({
  name: 'BAnalyticsDevices',
  props: {
    devices: {
      type: Array as PropType<Device[]>,
      required: true,
    },
  },
  data() {
    return {
      selected: [],
      headers: [
        {
          text: 'Device',
          align: 'start',
          sortable: true,
          value: 'deviceType',
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
