<template>
  <v-data-table
    :headers="headers"
    :items="messages"
    item-key="id"
    :loading="loading"
  >
    <template v-slot:no-data>
      <p class="text-center">
        No outbound message.
      </p>
    </template>

    <template v-slot:item="{ item }" class="text-left">
      <tr @click="gotoMessage(item)" class="bloom-pointer">
        <td>
          {{ item.subject }}
        </td>
        <td>
          <span v-if="item.lastSentAt"> {{ calendar(item.lastSentAt) }}</span>
          <span v-else>-</span>
        </td>
        <td>
          {{ item.sentCount }}
        </td>
        <td>
          {{ item.errorCount }}
        </td>
      </tr>
    </template>
  </v-data-table>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';
import { Message } from '@/domain/newsletter/model';
import { calendar } from '@/app/filters';


export default VueApp.extend({
  name: 'BNewsletterMessagesList',
  props: {
    messages: {
      type: Array as PropType<Message[]>,
      required: true,
    },
    loading: {
      type: Boolean as PropType<boolean>,
      required: false,
      default: false,
    },
    list: {
      type: String as PropType<string>,
      required: true,
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
          width: '30%',
        },
        // {
        //   text: 'Type',
        //   align: 'start',
        //   sortable: true,
        //   value: 'type',
        // },
        {
          text: 'Last sent at',
          align: 'start',
          sortable: true,
          value: 'lastSentAt',
        },
        {
          text: 'Sent',
          align: 'start',
          sortable: true,
          value: 'sentCount',
        },
        {
          text: 'Errors',
          align: 'start',
          sortable: true,
          value: 'errorCount',
        },
      ],
    };
  },
  methods: {
    calendar,
    gotoMessage(message: Message) {
      this.$router.push({ path: `/newsletter/lists/${this.list}/messages/${message.id}` });
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
