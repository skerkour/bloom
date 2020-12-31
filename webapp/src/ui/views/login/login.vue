<template>
  <b-login-form />
</template>


<script lang="ts">
import BLoginForm from '@/ui/components/users/login_form.vue';
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'MLoginView',
  components: {
    BLoginForm,
  },
});
</script>


<style lang="scss" scoped>
</style>
