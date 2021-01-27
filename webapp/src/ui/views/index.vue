<template>
  <b-redirect v-if="$store.state.session" :to="'/inbox'"/>
  <b-landing v-else />
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import BLanding from '@/ui/components/kernel/landing.vue';
import BRedirect from '@/ui/components/kernel/redirect.vue';


export default VueApp.extend({
  name: 'BIndexView',
  components: {
    BLanding,
    BRedirect,
  },
});
</script>


<style lang="scss" scoped>
</style>
