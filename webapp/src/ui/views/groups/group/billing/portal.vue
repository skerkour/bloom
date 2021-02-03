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
      <v-col cols="12">
        <h1 class="text-h4">
          Loading Billing Portal... Please do not quit.
        </h1>
      </v-col>
      <v-col cols="12" sm="10" md="8" xl="6">
        <v-progress-circular
          :size="50"
          color="primary"
          indeterminate
        />
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BGroupBillingPortalView',
  data() {
    return {
      loading: false,
      error: '',
    };
  },
  computed: {
    groupPath(): string {
      return this.$route.params.groupPath;
    },
  },
  created() {
    this.loadPortal();
  },
  methods: {
    async loadPortal(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        await this.$kernelService.gotoCustomerPortal(this.groupPath);
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
