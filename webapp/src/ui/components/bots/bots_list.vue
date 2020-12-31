<template>
  <v-data-table
    :headers="headers"
    :items="bots"
    item-key="id"
    hide-default-footer
    :loading="loading"
  >
    <template v-slot:no-data>
      <p class="text-center">
        No bot.
      </p>
    </template>

    <template v-slot:item="{ item }" class="text-left">
      <tr @click="gotoBot(item)" class="bloom-pointer">
        <td>
          {{ item.name }}
        </td>
        <td>
          <span>{{ calendar(item.createdAt) }}</span>
        </td>
        <td>
          <span v-if="item.lastExecutedAt">{{ calendar(item.lastExecutedAt) }}</span>
          <span v-else>-</span>
        </td>
      </tr>
    </template>
  </v-data-table>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';
import { Bot } from '@/api/graphql/model';
import { calendar } from '@/app/filters';


export default VueApp.extend({
  name: 'BBotsList',
  props: {
    bots: {
      type: Array as PropType<Bot[]>,
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
          width: '40%',
        },
        {
          text: 'Created at',
          align: 'start',
          sortable: true,
          value: 'createdAt',
        },
        {
          text: 'Last execution',
          align: 'start',
          sortable: true,
          value: 'lastExecutedAt',
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
    gotoBot(bot: Bot) {
      this.$router.push({ path: `/${this.projectFullPath}/-/bots/${bot.id}` });
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
