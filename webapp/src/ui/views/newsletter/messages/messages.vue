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

          <v-btn disabled color="success" depressed v-if="lists.length === 0">
             New message
            <v-icon left>mdi-plus</v-icon>
          </v-btn>

          <v-btn to="/newsletter/messages/new" color="success" depressed v-if="lists.length > 0">
            <v-icon left>mdi-plus</v-icon>
            New message
          </v-btn>
        </v-app-bar>

        <div class="text-right" v-if="lists.length === 0">
          Please create a list before a message
        </div>

      </v-col>

      <v-col cols="12" class="ma-0 py-0">
        <b-newsletter-messages-list :messages="messages" :loading="loading" />
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import BNewsletterMessagesList from '@/ui/components/newsletter/messages_list.vue';
import { Message, List } from '@/domain/newsletter/model';


export default VueApp.extend({
  name: 'BNewsletterMessagesView',
  components: {
    BNewsletterMessagesList,
  },
  data() {
    return {
      loading: false,
      error: '',
      messages: [] as Message[],
      lists: [] as List[],
    };
  },
  created() {
    this.fetchData();
  },
  methods: {
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        // TODO: merge into only one api request
        this.messages = await this.$newsletterService.fetchMessages();
        this.lists = await this.$newsletterService.fetchLists();
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
