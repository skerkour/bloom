<template>
  <v-container fluid>

    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center">
      <v-col cols="12">
        <h2>Newsletter messages</h2>
        <p>
          Newsletter messages (also known as campaigns or issues) are messages that you can
          send and automate to reach and engage with your contacts.
        </p>
      </v-col>

      <v-col cols="12" class="ma-0 py-0">
        <v-app-bar dense color="white" flat>
          <v-spacer />
          <v-btn to="/inbox/newsletter/messages/new" color="success" depressed>
            <v-icon left>mdi-plus</v-icon>
            New message
          </v-btn>
        </v-app-bar>
      </v-col>

      <v-col cols="12" class="ma-0 py-0">
        <b-outbound-messages-list :messages="messages" :loading="loading" />
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import {
  Project,
  OutboundMessage,
} from '@/api/graphql/model';
import BOutboundMessagesList from '@/ui/components/growth/outbound_messages_list.vue';


export default VueApp.extend({
  name: 'BOutboundMessagesView',
  components: {
    BOutboundMessagesList,
  },
  data() {
    return {
      loading: false,
      error: '',
      project: null as Project | null,
    };
  },
  computed: {
    messages(): OutboundMessage[] {
      return this.project?.outboundMessages ?? [];
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
        this.project = await this.$growthService.fetchOutboundMessages(this.projectFullPath);
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
