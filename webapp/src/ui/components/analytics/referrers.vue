<template>
  <v-card flat>
    <v-card-text>
      <v-data-table
        :headers="headers"
        :items="referrers"
        item-key="referrer"
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
              {{ truncate(item.referrer) }}
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
import { Referrer } from '@/domain/analytics/model';
import { PropType } from 'vue';
import truncate from '@/app/filters/truncate';

export default VueApp.extend({
  name: 'BAnalyticsReferrers',
  props: {
    referrers: {
      type: Array as PropType<Referrer[]>,
      required: true,
    },
  },
  data() {
    return {
      selected: [],
      headers: [
        {
          text: 'Referrer',
          align: 'start',
          sortable: true,
          value: 'referrer',
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
  methods: {
    truncate,
  },
});
</script>


<style lang="scss" scoped>
</style>
