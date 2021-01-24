<template>
  <v-container>
    <v-row class="justify-center text-center">

      <v-col cols="12" sm="8">
        <v-text-field
          label="Email or Username"
          v-model="emailOrUsername"
          :disabled="loading"
          @keyup="lowercaseEmailOrUsername"
          @keyup.enter.native="signIn"
        />
      </v-col>

      <v-col cols="12" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error">
          {{ error }}
        </v-alert>
      </v-col>

      <v-col cols="12" class="mt-5">
        <v-btn color="success"  @click="signIn" :loading="loading" depressed>
          Sign in
        </v-btn>
      </v-col>

    </v-row>
  </v-container>
</template>


<script lang="ts">
import { SignIn } from '@/domain/kernel/model';
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BLoginForm',
  data() {
    return {
      emailOrUsername: '',
      loading: false,
      error: '',
    };
  },
  methods: {
    async signIn() {
      this.loading = true;
      this.error = '';
      const input: SignIn = {
        email_or_username: this.emailOrUsername,
      };

      try {
        await this.$kernelService.signIn(input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    lowercaseEmailOrUsername() {
      this.emailOrUsername = this.emailOrUsername.toLowerCase();
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
