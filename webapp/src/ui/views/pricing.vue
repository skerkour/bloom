<template>
  <v-container class="mt-3 text-center" fluid>

    <v-row class="justify-center">
      <v-col cols="12" >
        <h4 class="text-h4">
          Start automating your projects today
        </h4>
      </v-col>
    </v-row>

    <v-row class="justify-center text-center">
      <v-col cols="12" md="10" xl="8" class="px-5 pt-5 pb-0">
        <p class="text-body-1 blm-free-tier">
          We offer an <b>eternal free tier</b> for your personal projects and MVPs.
          <b><router-link to="/register">Try it free now!</router-link></b>
        </p>
      </v-col>
    </v-row>

    <v-row class="justify-center">
      <v-col cols="12" md="10"  xl="8" class="px-5">
        <b-pricing-table />
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12" class="text-center py-3">
        <v-btn outlined to="/faq" color="primary">
          Questions? Explore the FAQ
        </v-btn>
      </v-col>
    </v-row>


    <v-row class="justify-center">
      <v-col cols="12" md="10" xl="8" class="text-center py-3">
        <!-- TODO: bloom VS image -->
        <v-divider />
      </v-col>
    </v-row>


    <v-row>
      <v-col cols="12" class="text-center py-3">
        <v-btn outlined to="/features" color="primary">
          Explore features
        </v-btn>
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import BPricingTable from '@/ui/components/kernel/pricing_table.vue';

export default VueApp.extend({
  name: 'BPricingView',
  components: {
    BPricingTable,
  },
});
</script>


<style lang="scss" scoped>
.blm-free-tier {
  font-size: 18px !important;
}
</style>
