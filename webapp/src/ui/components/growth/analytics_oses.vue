<template>
  <v-card flat>
    <v-card-text>
      <v-data-table
        :headers="headers"
        :items="oses"
        item-key="os"
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
              {{ item.os }}
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
import { AnalyticsOs } from '@/api/graphql/model';
import { PropType } from 'vue';


export default VueApp.extend({
  name: 'BAnalyticsOses',
  props: {
    oses: {
      type: Array as PropType<AnalyticsOs[]>,
      required: true,
    },
  },
  data() {
    return {
      selected: [],
      headers: [
        {
          text: 'Operating System',
          align: 'start',
          sortable: true,
          value: 'os',
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
