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
        <v-textarea v-model="input" outlined label="Input" rows="8" />
      </v-col>
    </v-row>

    <v-row justify="center" class="text-center">
      <v-col cols="12">
        <v-btn color="primary" @click="encode" class="mx-4" depressed>
          Encode
        </v-btn>

        <v-btn color="primary" @click="decode" depressed>
          Decode
        </v-btn>

        <v-btn outlined @click="clear" class="mx-4">
          Clear
        </v-btn>
      </v-col>
    </v-row>

    <v-row justify="center" class="text-center">
      <v-col cols="12" md="10" xl="6">
        <v-textarea :value="output" readonly  outlined placeholder="Output" rows="8" />
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BHexView',
  data() {
    return {
      input: '',
      output: '',
      error: '',
    };
  },
  methods: {
    encode() {
      try {
        this.output = this.$toolsService.encodeHex(this.input);
      } catch (err) {
        this.error = err.message;
      }
    },
    decode() {
      try {
        this.output = this.$toolsService.decodeHex(this.input);
      } catch (err) {
        this.error = err.message;
      }
    },
    clear() {
      this.input = '';
      this.output = '';
      this.error = '';
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
