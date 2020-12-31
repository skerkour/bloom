<template>
  <v-container fluid>

    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center" class="text-center" v-if="loading">
      <v-col cols="12" sm="10" md="8" xl="6">
        <v-progress-circular
          :size="50"
          color="primary"
          indeterminate
        />
      </v-col>
    </v-row>

    <v-row justify="center" v-else>
      <v-col cols="12">
        <h2>Tickets</h2>
        <p>
          Tickets allow you to collaboratively develop ideas, solve problems, and plan work.
        </p>
      </v-col>

      <v-col cols="12" class="ma-0 py-0">
        <v-app-bar dense color="white" flat>
          <v-spacer />
          <v-btn :to="`/${projectFullPath}/-/tickets/new`" color="success" depressed>
          <v-icon left>mdi-plus</v-icon>
            New ticket
          </v-btn>
        </v-app-bar>
      </v-col>

      <v-col cols="12" class="ma-0 py-0">
        <v-tabs v-model="tab">
          <v-tab>Open</v-tab>
          <v-tab>Closed</v-tab>
        </v-tabs>
        <v-tabs-items v-model="tab">
          <v-tab-item>
            <tickets-list :projectFullPath="projectFullPath" :tickets="open" />
          </v-tab-item>
          <v-tab-item>
            <tickets-list :projectFullPath="projectFullPath" :tickets="closed" />
          </v-tab-item>
        </v-tabs-items>
      </v-col>
    </v-row>

  </v-container>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';
import { Project, Ticket } from '@/api/graphql/model';
import TicketsList from '@/ui/components/collaboration/tickets_list.vue';

export default VueApp.extend({
  name: 'ProjectTicketsView',
  components: {
    TicketsList,
  },
  data() {
    return {
      loading: false,
      error: '',
      tab: null,
      project: null as Project | null,
    };
  },
  computed: {
    tickets(): Ticket[] {
      return this.project ? this.project.tickets : [];
    },
    open(): Ticket[] {
      return this.project
        ? this.project.tickets.filter((ticket: Ticket) => !ticket.closedAt)
        : [];
    },
    closed(): Ticket[] {
      return this.project
        ? this.project.tickets.filter((ticket: Ticket) => ticket.closedAt)
        : [];
    },
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
  },
  created() {
    this.fetchData();
  },
  methods: {
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.project = await this.$collaborationService.fetchTickets(this.projectFullPath);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
  },
});
</script>

<style lang="scss" scoped>
</style>
