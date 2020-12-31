<template>
  <v-card flat>
    <v-card-text>
      <v-data-table
        :headers="headers"
        :items="countries"
        item-key="country"
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
              {{ item.country }}
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
import { AnalyticsCountry } from '@/api/graphql/model';
import { PropType } from 'vue';


export default VueApp.extend({
  name: 'BAnalyticsCountries',
  props: {
    countries: {
      type: Array as PropType<AnalyticsCountry[]>,
      required: true,
    },
  },
  data() {
    return {
      selected: [],
      headers: [
        {
          text: 'Country',
          align: 'start',
          sortable: true,
          value: 'country',
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
