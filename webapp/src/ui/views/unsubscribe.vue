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

    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="success !== ''">
        <v-alert type="success" :value="success !== ''">
          {{ success }}
        </v-alert>
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { VueApp } from '@/app/vue';
import { UnsubscribeFromList } from '@/domain/inbox/model';

export default VueApp.extend({
  name: 'BConfirmListSubscriptionView',
  data() {
    return {
      loading: false,
      error: '',
      success: '',
    };
  },
  created() {
    this.fetchData();
  },
  methods: {
    async fetchData(): Promise<void> {
      const { subscription } = this.$route.query;
      if (!subscription) {
        // || !this.$route.params.listId) {
        this.error = 'Link is broken';
        return;
      }
      const input: UnsubscribeFromList = {
        subscription_id: subscription! as string,
      };

      this.loading = true;
      this.error = '';

      try {
        await this.$inboxService.unsubscribeFromList(input);
        this.success = 'Successfully unsubscribed';
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
