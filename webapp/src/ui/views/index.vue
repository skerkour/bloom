<template>
  <b-inbox v-if="$store.state.me" />
  <b-landing v-else />
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import BLanding from '@/ui/components/kernel/landing.vue';
import BInbox from './inbox/inbox.vue';


export default VueApp.extend({
  name: 'BIndexView',
  components: {
    BLanding,
    BInbox,
  },
});
</script>


<style lang="scss" scoped>
</style>
