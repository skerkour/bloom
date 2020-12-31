<template>
  <b-home v-if="$store.state.me" />
  <b-landing v-else />
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import BLanding from '@/ui/components/kernel/landing.vue';
import BHome from '@/ui/components/kernel/home.vue';


export default VueApp.extend({
  name: 'BIndexView',
  components: {
    BLanding,
    BHome,
  },
});
</script>


<style lang="scss" scoped>
</style>
