<template>
  <div>
    <v-list two-line nav v-if="tickets.length !== 0">
      <v-list-item
        v-for="ticket in tickets"
        :key="ticket.id"
        :to="`/${projectFullPath}/-/tickets/${ticket.id}`"
      >
        <v-list-item-content>
          <v-list-item-title v-text="ticket.title"></v-list-item-title>
          <v-list-item-subtitle >
            opened {{ date(ticket.createdAt) }}
          by <router-link :to="`/${ticket.author.username}`">{{ ticket.author.name }}</router-link>
          </v-list-item-subtitle>
        </v-list-item-content>

        <v-list-item-action>
          <v-list-item-action-text v-text="ticket.updatedAt" />
          <v-tooltip bottom>
            <template v-slot:activator="{ on, attrs }">
              <v-icon  color="error" v-bind="attrs" v-on="on" v-if="ticket.closedAt">
                mdi-alert-circle-check-outline
              </v-icon>
            </template>
            <span>Closed</span>
          </v-tooltip>

          <v-tooltip bottom>
            <template v-slot:activator="{ on, attrs }">
              <v-icon  color="success" v-bind="attrs" v-on="on" v-if="!ticket.closedAt">
                mdi-alert-circle-outline
              </v-icon>
            </template>
            <span>Open</span>
          </v-tooltip>
        </v-list-item-action>
      </v-list-item>
    </v-list>

    <div v-else class="text-center">
      <h2 class="headline">No ticket yet</h2>
    </div>
  </div>
</template>


<script lang="ts">
import { PropType } from 'vue';
import { VueApp } from '@/app/vue';
import { Ticket } from '@/api/graphql/model';
import date from '@/app/filters/date';

export default VueApp.extend({
  name: 'BTicketsList',
  props: {
    tickets: {
      type: Array as PropType<Ticket[]>,
      default: [],
    },
  },
  computed: {
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
  },
  methods: {
    date,
  },
});
</script>


<style lang="scss" scoped>
</style>
