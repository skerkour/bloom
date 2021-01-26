<template>
  <v-container fluid>
    <v-row>
      <b-contact />
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import BContact from '@/ui/components/inbox/contact.vue';

export default VueApp.extend({
  name: 'ProjectNewContactView',
  components: {
    BContact,
  },
});
</script>


<style lang="scss" scoped>
</style>
