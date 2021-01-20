<template>
  <b-register-form />
</template>


<script lang="ts">
import BRegisterForm from '@/ui/components/kernel/register_form.vue';
import { VueApp } from '@/app/vue';


export default VueApp.extend({
  name: 'BRegisterView',
  components: {
    BRegisterForm,
  },
});
</script>


<style lang="scss" scoped>
</style>
