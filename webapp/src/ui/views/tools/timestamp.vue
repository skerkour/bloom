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
        <v-text-field v-model="input" outlined label="Timestamp or date" />

        <v-btn color="primary" @click="convert" class="mx-4" depressed>
          Convert
        </v-btn>

        <v-btn color="primary" @click="now" class="mx-4" depressed>
          Now
        </v-btn>

        <v-btn outlined @click="clear" class="mx-4">
          Clear
        </v-btn>
      </v-col>
    </v-row>


    <v-row justify="center" class="text-center mt-5" v-if="timestamp">
      <v-col cols="12" md="10" xl="6">
        <p>
          Unix: {{ timestamp.unix }}
        </p>
        <p>
          ISO 8601: {{ timestamp.iso }}
        </p>
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { Timestamp } from '@/domain/tools/service';

export default VueApp.extend({
  name: 'BJsonView',
  data() {
    return {
      input: '',
      error: '',
      timestamp: null as Timestamp | null,
    };
  },
  methods: {
    convert() {
      try {
        this.timestamp = this.$toolsService.timestamp(this.input);
      } catch (err) {
        this.error = err.message;
      }
    },
    now() {
      this.input = new Date().toISOString();
      this.convert();
    },
    clear() {
      this.input = '';
      this.error = '';
      this.timestamp = null;
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
