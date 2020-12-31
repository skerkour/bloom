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
import { VerifyEmailInput } from '@/api/graphql/model';
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BVerifyEmailView',
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
      this.loading = true;
      this.error = '';
      const input: VerifyEmailInput = {
        token: this.$route.params.token,
      };

      try {
        await this.$usersService.verifyEmail(input);
        this.success = 'Your email is now verified. You will be redirected soon.';
        setTimeout(() => {
          this.$router.push({ path: '/' });
        }, 2500);
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
