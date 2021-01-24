
<template>
<v-container>
    <v-row class="justify-center text-center">
      <v-col cols="12" sm="8">
        <h2>Please enter your 2FA code from your authenticator app</h2>
      </v-col>

      <v-col cols="12" sm="8">
        <v-text-field
          v-model="code"
          label="Your 2fa code"
          :disabled="loading"
          outlined
          @keyup="checkCodeLength"
          counter="6"
        />
      </v-col>

      <v-col cols="12" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error">
          {{ error }}
        </v-alert>
      </v-col>

      <v-col cols="12" class="mt-5">
        <v-btn color="success" @click="completeTwoFA" :loading="loading">
          Sign in
        </v-btn>
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { CompleteTwoFaChallenge } from '@/domain/kernel/model';

const CODE_LENGTH = 6;

export default VueApp.extend({
  name: 'B2FAView',
  data() {
    return {
      loading: false,
      error: '',
      code: '',
      pendingSessionId: '',
    };
  },
  computed: {
    canSignIn(): boolean {
      return this.code.length === CODE_LENGTH;
    },
  },
  created() {
    if (this.$store.state.pendingSessionId) {
      this.pendingSessionId = this.$store.state.pendingSessionId;
    } else {
      this.$router.push({ path: '/login' });
    }
  },
  methods: {
    async completeTwoFA() {
      this.loading = true;
      this.error = '';
      const input: CompleteTwoFaChallenge = {
        pending_session_id: this.pendingSessionId,
        code: this.code,
      };

      try {
        await this.$kernelService.completeTwoFaChallenge(input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    checkCodeLength() {
      if (this.canSignIn) {
        this.completeTwoFA();
      }
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
