<template>
  <v-container fluid>
    <b-billing />
  </v-container>
</template>


<script lang="ts">
/* eslint-disable max-len, @typescript-eslint/no-non-null-assertion */
import { VueApp } from '@/app/vue';
import BBilling from '@/ui/components/kernel/billing.vue';

export default VueApp.extend({
  name: 'BUserBilling',
  components: {
    BBilling,
  },
});
</script>


<style lang="scss" scoped>

</style>
