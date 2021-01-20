<template>
  <v-container>
    <v-row class="justify-center text-center">
      <v-col cols="12" sm="8">
        <h2>Check your email</h2>
        <p class="mt-3">
          We've sent you a 12 characters confirmation code by email.<br/>
          The code will only be valid for 30 minutes.
        </p>
      </v-col>

      <v-col cols="12" sm="8">
        <v-text-field
          v-model="code"
          label="Your confirmation code"
          :disabled="loading"
          outlined
          v-mask="{ tokens: codeTokens, mask: codeMask }"
          @keyup="checkCodeLength"
        />
      </v-col>

      <v-col cols="12" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error">
          {{ error }}
        </v-alert>
      </v-col>

      <v-col cols="12" class="mt-5">
        <v-btn color="success" @click="completeRegistration" :loading="loading" depressed>
          Complete Registration
        </v-btn>
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { mask } from 'vue-the-mask';
import { CompleteRegistration } from '@/domain/kernel/model';

const CODE_LENGTH = 14;

export default VueApp.extend({
  name: 'BCompleteRegistrationView',
  directives: {
    mask,
  },
  data() {
    return {
      loading: false,
      error: '',
      code: '',
      email: '',
      codeMask: 'XXXX-XXXX-XXXX',
      pendingUserId: '',
      codeTokens: {
        X: {
          pattern: /[0-9a-zA-Z#@!]/,
          transform: (v: string) => v.toLocaleLowerCase(),
        },
      },
    };
  },
  computed: {
    canCompleteRegistration(): boolean {
      return this.code.length === CODE_LENGTH;
    },
  },
  created() {
    if (this.$store.state.pendingUserId) {
      this.pendingUserId = this.$store.state.pendingUserId;
    } else {
      this.$router.push({ path: '/register' });
    }
  },
  methods: {
    async completeRegistration() {
      this.loading = true;
      this.error = '';
      const input: CompleteRegistration = {
        pending_user_id: this.pendingUserId,
        code: this.code,
      };

      try {
        await this.$kernelService.completeRegistration(input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    checkCodeLength() {
      if (this.canCompleteRegistration) {
        this.completeRegistration();
      }
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
