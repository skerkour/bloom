<template>
  <v-container fluid>
    <b-bot />
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import BBot from '@/ui/components/bots/bot.vue';

export default VueApp.extend({
  name: 'BNewBotView',
  components: {
    BBot,
  },
});
</script>


<style lang="scss" scoped>
</style>
