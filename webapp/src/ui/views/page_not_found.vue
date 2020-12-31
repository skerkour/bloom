<template>
  <v-container class="mt-3 text-center">
    <v-row row wrap justify-center>

      <v-col cols="12" class="text-center mb-5">
        <v-img alt="404" src="/assets/imgs/404.svg" id="img-404" contain  height="400"/>
      </v-col>

      <v-col cols="12" class="text-center">
        <h1 class="display-2 font-weight-medium">
         Page not found
        </h1>
      </v-col>

      <v-col cols="12" class="text-center mt-4 mb-4">
        <v-btn to="/" color="primary" depressed>Back to Home</v-btn>
      </v-col>

    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BPageNotFoundView',
});
</script>


<style lang="scss" scoped>
</style>
