<template>
  <v-dialog
    v-model="show"
    max-width="400px"
   persistent
  >
    <v-card>
      <v-card-title class="headline">
        {{ title }}
      </v-card-title>

      <v-card-text>
        <v-alert icon="mdi-alert-circle" :value="error !== ''" type="error" dismissible>
          {{ error }}
        </v-alert>

        <img :src="`data:image/jpeg;base64, ${qrcode}`" alt="QR Code" v-if="qrcode" />
        <v-text-field
          label="Code"
          outline
          v-model="code"
          :loading="loading"
          outlined
          counter="6"
          @keyup.enter.native="enableDisable"
        />
      </v-card-text>

      <v-card-actions>
        <v-btn text @click="cancel" :loading="loading">Cancel</v-btn>

        <v-spacer />

        <v-btn @click="enableDisable" :color="btnColor" :loading="loading" depressed>
          {{ title }}
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
    qrcode: {
      type: String as PropType<string>,
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
    title(): string {
      return this.qrcode ? 'Enable 2FA' : 'Disable 2FA';
    },
    btnColor(): string {
      return this.qrcode ? 'success' : 'error';
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
    enableDisable() {
      if (this.qrcode) {
        this.$emit('enable', this.code);
      } else {
        this.$emit('disable', this.code);
      }
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
