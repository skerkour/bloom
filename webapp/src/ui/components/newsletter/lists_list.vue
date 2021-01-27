<template>
  <v-data-table
    :headers="headers"
    :items="lists"
    item-key="id"
    hide-default-footer
    :loading="loading"
  >
    <template v-slot:no-data>
      <p class="text-center">
        No list.
      </p>
    </template>

    <template v-slot:item="{ item }" class="text-left">
      <tr @click="gotoList(item)" class="bloom-pointer">
        <td>
          {{ item.name }}
        </td>
      </tr>
    </template>
  </v-data-table>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';
import { List } from '@/api/graphql/model';
import { calendar } from '@/app/filters';


export default VueApp.extend({
  name: 'BListsList',
  props: {
    lists: {
      type: Array as PropType<List[]>,
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
          text: 'Name',
          align: 'start',
          sortable: true,
          value: 'name',
        },
      ],
    };
  },
  methods: {
    calendar,
    gotoList(list: List) {
      this.$router.push({ path: `/newsletter/lists/${list.id}` });
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
