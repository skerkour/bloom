<template>
  <v-dialog
    v-model="show"
    max-width="400px"
   persistent
  >
    <v-card>
      <v-card-title class="headline">
        Enter you 2FA code
      </v-card-title>

      <v-card-text>
        <v-alert icon="mdi-alert-circle" :value="error !== ''" type="error" dismissible>
          {{ error }}
        </v-alert>

        <v-text-field
          label="Code"
          outline
          v-model="code"
          :loading="loading"
          outlined
          counter="6"
          @keyup.enter.native="deleteMyAccount"
        />
      </v-card-text>

      <v-card-actions>
        <v-btn text @click="cancel" :loading="loading">Cancel</v-btn>

        <v-spacer />

        <v-btn @click="deleteMyAccount" color="error" :loading="loading" depressed>
          <v-icon left>mdi-close-octagon</v-icon>
          Delete my account
        </v-btn>
      </v-card-actions>

    </v-card>

  </v-dialog>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';

export default VueApp.extend({
  name: 'BTwoFASetupDialog',
  props: {
    value: {
      type: Boolean as PropType<boolean>,
      required: true,
    },
    error: {
      type: String as PropType<string>,
      required: true,
    },
    loading: {
      type: Boolean as PropType<boolean>,
      required: true,
    },
  },
  data() {
    return {
      code: '',
    };
  },
  computed: {
    show: {
      get(): boolean {
        return this.value;
      },
      set(value: boolean) {
        this.$emit('input', value);
      },
    },
  },
  watch: {
    value(value: boolean) {
      if (!value) {
        this.code = '';
      }
    },
  },
  methods: {
    cancel() {
      this.code = '';
      this.show = false;
    },
    deleteMyAccount() {
      this.$emit('delete-account', this.code);
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
