<template>
  <v-data-table
    :headers="headers"
    :items="sessions"
    item-key="id"
    hide-default-footer
    :loading="loading"
  >
    <template v-slot:no-data>
      <p class="text-center">
        No session.
      </p>
    </template>

    <template v-slot:item="{ item }" class="text-left">
      <tr>
        <td>
          {{ calendar(item.created_at) }}
        </td>
        <td>
          <v-btn depressed color="error" @click="revoke(item)" :disabled="loading"
            v-if="item.id === currentSession.id">
            Sign Out
          </v-btn>
          <v-btn outlined color="error" @click="revoke(item)" :disabled="loading" v-else>
            Revoke
          </v-btn>
        </td>
      </tr>
    </template>
  </v-data-table>
</template>


<script lang="ts">
import { Session } from '@/domain/kernel/model';
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';
import calendar from '@/app/filters/calendar';

export default VueApp.extend({
  name: 'BSessionsLists',
  props: {
    sessions: {
      type: Array as PropType<Session[]>,
      required: true,
    },
    currentSession: {
      type: Object as PropType<Session>,
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
          text: 'Signed in',
          align: 'start',
          sortable: true,
          value: 'created_at',
        },
        {
          text: 'Actions',
          align: 'start',
          sortable: false,
        },
      ],
    };
  },
  methods: {
    calendar,
    revoke(session: Session) {
      this.$emit('revoke', session);
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
