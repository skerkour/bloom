<template>
  <v-card flat>
    <v-card-text>
      <v-data-table
        :headers="headers"
        :items="browsers"
        item-key="browser"
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
              {{ item.browser }}
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
import { AnalyticsBrowser } from '@/api/graphql/model';
import { PropType } from 'vue';


export default VueApp.extend({
  name: 'BAnalyticsBrowsers',
  props: {
    browsers: {
      type: Array as PropType<AnalyticsBrowser[]>,
      required: true,
    },
  },
  data() {
    return {
      selected: [],
      headers: [
        {
          text: 'Browser',
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
