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

    <v-row v-if="message">
      <b-newsletter-message :message="message.message"
        @updated="onMessageUpdated" :list="listId" />
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { MessageWithLists } from '@/domain/newsletter/model';
import BNewsletterMessage from '@/ui/components/newsletter/message.vue';

export default VueApp.extend({
  name: 'BNewsletterMessageView',
  components: {
    BNewsletterMessage,
  },
  data() {
    return {
      loading: false,
      error: '',
      message: null as MessageWithLists | null,
    };
  },
  computed: {
    listId(): string {
      return this.$route.params.listId;
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
        this.message = await this.$newsletterService.fetchMessage(this.$route.params.messageId);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    onMessageUpdated(message: MessageWithLists) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      this.message = message;
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
