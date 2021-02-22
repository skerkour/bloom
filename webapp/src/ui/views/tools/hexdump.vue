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
        <v-file-input
          v-model="input"
          show-size
          truncate-length="20"
          label="Upload file"
          hint="Max size: 2MB"
          :loading="loading"
        />
      </v-col>
    </v-row>

    <v-row justify="center" class="text-center">
      <v-col cols="12">
        <v-btn color="primary" @click="dump" class="mx-4" depressed :loading="loading">
          Dump
        </v-btn>

        <v-btn outlined color="primary" @click="clear" class="mx-4" :loading="loading">
          Clear
        </v-btn>
      </v-col>
    </v-row>

    <v-row justify="center" class="text-center">
      <v-col cols="12" md="10" xl="6">
        <pre>{{ output }}</pre>
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
      output: '',
      error: '',
      input: null as File | null,
    };
  },
  methods: {
    async dump() {
      if (!this.input) {
        this.error = 'Please select 1 file';
        return;
      }

      this.error = '';
      this.loading = true;

      try {
        // this.output = await this.$toolsService.hexdump(this.input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    clear() {
      this.output = '';
      this.error = '';
      this.input = null;
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
