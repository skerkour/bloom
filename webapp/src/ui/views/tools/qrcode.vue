<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center" class="text-center">
      <v-col cols="12" md="10" xl="6">
        <v-textarea
          v-model="input"
          label="Input"
          :loading="loading"
          outlined
        />
      </v-col>
    </v-row>

    <v-row justify="center" class="text-center">
      <v-col cols="12">
        <v-btn color="primary" @click="generate" class="mx-4" depressed :loading="loading">
          Generate
        </v-btn>

        <v-btn outlined color="primary" @click="clear" class="mx-4" :loading="loading">
          Clear
        </v-btn>
      </v-col>
    </v-row>

    <v-row justify="center" class="text-center">
      <v-col cols="12" md="10" xl="6">
        <img :src="`data:image/jpeg;base64, ${qrcode}`" alt="QR Code" v-if="qrcode" />
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BHexdumpView',
  data() {
    return {
      loading: false,
      qrcode: '',
      error: '',
      input: '',
    };
  },
  methods: {
    async generate() {
      this.error = '';
      this.loading = true;

      try {
        const qrcode = await this.$kernelService.generateQrCode(this.input);
        this.qrcode = qrcode.base64_jpeg_qr_code;
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    clear() {
      this.qrcode = '';
      this.error = '';
      this.input = '';
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
