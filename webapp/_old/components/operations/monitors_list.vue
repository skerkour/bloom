<template>
  <v-data-table
    :headers="headers"
    :items="monitors"
    item-key="id"
    hide-default-footer
    :loading="loading"
  >
    <template v-slot:no-data>
      <p class="text-center">
        No monitor.
      </p>
    </template>

    <template v-slot:item="{ item }" class="text-left">
      <tr @click="gotoMonitor(item)" class="bloom-pointer">
        <td>
          {{ item.name }}
        </td>
        <td>
          {{ item.status }}
        </td>
        <td>
          {{ item.showOnStatusPage }}
        </td>
      </tr>
    </template>
  </v-data-table>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';
import { Monitor } from '@/api/graphql/model';

export default VueApp.extend({
  name: 'BMonitorsList',
  props: {
    monitors: {
      type: Array as PropType<Monitor[]>,
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
      selected: [],
      headers: [
        {
          text: 'Name',
          align: 'start',
          sortable: true,
          value: 'name',
          width: '50%',
        },
        {
          text: 'Status',
          align: 'start',
          sortable: true,
          value: 'status',
        },
        {
          text: 'Show on status page',
          align: 'start',
          sortable: true,
          value: 'showOnStatusPage',
        },
      ],
    };
  },
  computed: {
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
  },
  methods: {
    gotoMonitor(monitor: Monitor) {
      this.$router.push({ path: `/${this.projectFullPath}/-/monitors/${monitor.id}` });
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
