<template>
  <v-data-table
    :headers="headers"
    :items="connections"
    item-key="id"
    hide-default-footer
    :loading="loading"
  >
    <template v-slot:no-data>
      <p class="text-center">
        No connection.
      </p>
    </template>

    <template v-slot:item="{ item }" class="text-left">
      <tr @click="gotoConnection(item)" class="bloom-pointer">
        <td>
          {{ item.name }}
        </td>
         <td>
          <span>{{ item.app.id }}</span>
        </td>
        <td>
          <span>{{ calendar(item.createdAt) }}</span>
        </td>
      </tr>
    </template>
  </v-data-table>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';
import { BotConnection } from '@/api/graphql/model';
import { calendar } from '@/app/filters';


export default VueApp.extend({
  name: 'BBotsList',
  props: {
    connections: {
      type: Array as PropType<BotConnection[]>,
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
        {
          text: 'App',
          align: 'start',
          sortable: true,
          value: 'app.id',
        },
        {
          text: 'Created at',
          align: 'start',
          sortable: true,
          value: 'createdAt',
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
    calendar,
    gotoConnection(connection: BotConnection) {
      this.$router.push({ path: `/${this.projectFullPath}/-/bots/connections/${connection.id}` });
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
