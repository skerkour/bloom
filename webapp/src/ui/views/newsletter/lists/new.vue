<template>
  <b-list />
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import BList from '@/ui/components/newsletter/list.vue';


export default VueApp.extend({
  name: 'BNewListView',
  components: {
    BList,
  },
});
</script>


<style lang="scss" scoped>
</style>
