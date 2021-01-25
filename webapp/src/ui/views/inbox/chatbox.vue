<template>
  <v-container fluid>
    <v-row>
      <v-col cols="12" class="text-center" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
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

    <v-row v-if="chatboxPreferences">
      <v-col cols="12" class="text-left">
        <h2 class="headline">Chatbox</h2>

        <b-chatbox-setup-card class="mt-5 mb-5" :preferences="chatboxPreferences" />

        <b-chatbox-preferences
          :preferences="chatboxPreferences"
          @updated="onChatboxPreferencesUpdated"
        />
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import BChatboxPreferences from '@/ui/components/inbox/chatbox_preferences.vue';
import { ChatboxPreferences } from '@/domain/inbox/model';
import BChatboxSetupCard from '@/ui/components/inbox/chatbox_setup_card.vue';

export default VueApp.extend({
  name: 'BInboxChatboxView',
  components: {
    BChatboxPreferences,
    BChatboxSetupCard,
  },
  data() {
    return {
      loading: false,
      error: '',
      chatboxPreferences: null as ChatboxPreferences | null,
    };
  },
  created() {
    this.fetchData();
  },
  methods: {
    async fetchData() {
      this.loading = true;
      this.error = '';

      try {
        this.chatboxPreferences = await this.$inboxService.fetchChatboxPreferences();
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    onChatboxPreferencesUpdated(preferences: ChatboxPreferences) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      this.chatboxPreferences = preferences;
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
