<template>
  <v-container>
    <v-row class="justify-center text-center">

      <v-col cols="12" sm="8">
        <v-text-field
          label="Email"
          type="email"
          v-model="email"
          :rules="emailRules"
          :disabled="loading"
          @keyup="lowercaseEmail"
        />
      </v-col>

      <v-col cols="12" sm="8">
        <v-text-field
          label="Username"
          type="text"
          v-model="username"
          :rules="usernameRules"
          :disabled="loading"
          @keyup="lowercaseUsername"
          @keyup.enter.native="register"
        />
      </v-col>

      <v-col cols="12" class="text-center py-2">
        By creating an account, I agree to the
        <a href="/terms" target="_blank" rel="noopener">Terms of Service</a>
        and <a href="privacy" target="_blank" rel="noopener">Privacy policy</a>.
      </v-col>

      <v-col cols="12" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error">
          {{ error }}
        </v-alert>
      </v-col>

      <v-col cols="12" class="mt-3">
        <v-btn color="success"  @click="register" :loading="loading" depressed>
          Sign up for Bloom
        </v-btn>
      </v-col>

    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Register } from '@/domain/kernel/model';
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BRegisterForm',
  data() {
    return {
      username: '',
      usernameRules: [
        (v: string) => !!v || 'Username is required',
        (v: string) => v.length >= 4 || 'Username is too short',
        (v: string) => v.length <= 20 || 'Username is too long',
        (v: string) => /^[a-z0-9]*$/.test(v) || 'Username is not valid',
      ],
      email: '',
      emailRules: [
        (v: string) => !!v || 'Email is required',
        (v: string) => v.indexOf('@') !== -1 || 'Email is not valid',
      ],
      loading: false,
      isValid: false,
      error: '',
    };
  },
  methods: {
    async register(): Promise<void> {
      this.loading = true;
      this.error = '';
      const input: Register = {
        username: this.username,
        email: this.email,
      };

      try {
        await this.$kernelService.register(input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    lowercaseEmail(): void {
      this.email = this.email.toLowerCase();
    },
    lowercaseUsername(): void {
      this.username = this.username.toLowerCase();
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
