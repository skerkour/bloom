<template>
  <v-data-table
    :headers="headers"
    :items="groups"
    item-key="id"
    :loading="loading"
    :items-per-page="50"
  >
    <template v-slot:no-data>
      <p class="text-center">
        No group.
      </p>
    </template>

    <template v-slot:item="{ item }" class="text-left">
      <tr @click="gotoGroup(item)" class="bloom-pointer">
        <td>
          {{ item.id }}
        </td>
        <td>
          {{ date(item.created_at) }}
        </td>
        <td>
          {{ item.name }}
        </td>
        <td>
          {{ item.path }}
        </td>
      </tr>
    </template>
  </v-data-table>
</template>


<script lang="ts">
import { PropType } from 'vue';
import { VueApp } from '@/app/vue';
import { Group } from '@/domain/kernel/model';
import date from '@/app/filters/date';

export default VueApp.extend({
  name: 'BAdminGroupsList',
  props: {
    groups: {
      type: Array as PropType<Group[]>,
      default: [],
    },
    loading: {
      type: Boolean as PropType<boolean>,
      default: false,
      required: true,
    },
  },
  data() {
    return {
      headers: [
        {
          text: 'ID',
          align: 'start',
          sortable: false,
          value: 'id',
        },
        {
          text: 'Created At',
          align: 'start',
          sortable: true,
          value: 'created_at',
        },
        {
          text: 'Name',
          align: 'start',
          sortable: true,
          value: 'name',
        },
        {
          text: 'Path',
          align: 'start',
          sortable: true,
          value: 'path',
        },
      ],
    };
  },
  methods: {
    date,
    gotoGroup(group: Group) {
      this.$router.push({ path: `/admin/groups/${group.id}` });
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
