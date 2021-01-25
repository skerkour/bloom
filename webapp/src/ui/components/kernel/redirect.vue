<template>
  <div />
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';

export default VueApp.extend({
  name: 'BRedirect',
  props: {
    to: {
      type: String as PropType<string>,
      required: false,
      default: '/',
    },
  },
  created() {
    this.$router.push({ path: this.to });
  },
});
</script>


<style lang="scss">
</style>
