<template>
  <v-container class="mt-3 text-center" fluid>

    <v-row class="justify-center">
      <v-col cols="12" class="text-center">
        <h1 class="text-h3">
          Features
        </h1>
      </v-col>

      <!-- <v-col cols="12">
        <p class="text-body-1">
          Bloom is an open source automation
          <a href="https://about.gitlab.com/handbook/product/single-application/" target="_blank" rel="noopener">single application</a>
          that work for you 24 / 7 so you can enjoy life and sleep at nights 🤖
        </p>
      </v-col> -->
    </v-row>


    <v-row class="justify-center">
      <v-col cols="10" sm="8" md="12" class="text-center">
        <h1 class="text-h4">
          Bots
        </h1>
      </v-col>
    </v-row>

    <v-row class="justify-center">
      <v-col cols="10" sm="8" md="12" class="text-center">
        <h1 class="text-h4">
          Growth
        </h1>
      </v-col>
    </v-row>

    <v-row class="justify-center">
      <v-col cols="10" sm="8" md="12" class="text-center">
        <h1 class="text-h4">
          Support
        </h1>
      </v-col>
    </v-row>

    <v-row class="justify-center">
      <v-col cols="10" sm="8" md="12" class="text-center">
        <h1 class="text-h4">
          Collaboration
        </h1>
      </v-col>
    </v-row>

    <v-row class="justify-center">
      <v-col cols="10" sm="8" md="12" class="text-center">
        <h1 class="text-h4">
          Monitoring
        </h1>
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BFeaturesView',
});
</script>


<style lang="scss" scoped>
</style>
