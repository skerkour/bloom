<template>
  <v-container fluid>
    <b-milestone />
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import BMilestone from '@/ui/components/collaboration/milestone.vue';


export default VueApp.extend({
  name: 'BNewMilestoneView',
  components: {
    BMilestone,
  },
});
</script>


<style lang="scss" scoped>
</style>
