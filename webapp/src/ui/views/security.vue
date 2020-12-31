<template>
  <v-container class="mt-4 text-center">
    <v-row class="justify-center">
      <v-col cols="12" sm="10" md="8" class="text-center text-body-1">

        <h1 class="mt-5 mb-3 text-h3">Security</h1>
        <p class="body-1 mb-5 font-weight-regular mb-5 text-left">
          Keeping our customers data safe and secure is our top priority. We take threats very
          seriously and work hard to protect our customers and their data. Your input and feedback
          on our security is greatly appreciated.
        </p>

        <h2 class="mt-5 mb-3 display-1 font-weight-medium">Reporting Issues</h2>

        <p class="body-1 font-weight-regular text-left">
          Send reports directly to <a href="mailto:hello@bloom.sh">hello@bloom.sh</a>
          and we will get back to you faster than the light.
        </p>

      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BSecurityView',
});
</script>


<style lang="scss" scoped>
</style>
