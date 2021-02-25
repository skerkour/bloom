<template>
  <v-list nav dense class="text-left">

    <v-list-item exact link to="/preferences/profile">
      <v-list-item-icon>
        <v-icon>mdi-account-outline</v-icon>
      </v-list-item-icon>
      <v-list-item-content>
        <v-list-item-title>Profile</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <v-list-item exact link to="/preferences/billing">
      <v-list-item-icon>
        <v-icon>mdi-credit-card</v-icon>
      </v-list-item-icon>
      <v-list-item-content>
        <v-list-item-title>Billing</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <v-list-item exact link to="/preferences/invitations">
      <v-list-item-icon>
        <v-icon>mdi-email-outline</v-icon>
      </v-list-item-icon>
      <v-list-item-content>
        <v-list-item-title>Group invitations</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <v-list-item exact link to="/preferences/sessions">
      <v-list-item-icon>
        <v-icon>mdi-cellphone-link</v-icon>
      </v-list-item-icon>
      <v-list-item-content>
        <v-list-item-title>Devices</v-list-item-title>
      </v-list-item-content>
    </v-list-item>
  </v-list>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BPreferencesDrawer',
});
</script>


<style lang="scss" scoped>
</style>
