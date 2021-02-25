<template>
  <v-list nav dense class="text-left">

    <v-list-item exact link to="/tools/base64">
      <v-list-item-content>
        <v-list-item-title>Base64 encode/decode</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <v-list-item exact link to="/tools/qrcode">
      <v-list-item-content>
        <v-list-item-title>QR code generator</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <v-list-item exact link to="/tools/hex">
      <v-list-item-content>
        <v-list-item-title>Hex encode/decode</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <v-list-item exact link to="/tools/json">
      <v-list-item-content>
        <v-list-item-title>JSON formatter/validator</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <v-list-item exact link to="/tools/url">
      <v-list-item-content>
        <v-list-item-title>URL encode/decode</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <v-list-item exact link to="/tools/timestamp">
      <v-list-item-content>
        <v-list-item-title>Timestamp converter</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <v-list-item exact link to="/tools/rot13">
      <v-list-item-content>
        <v-list-item-title>Rot 13</v-list-item-title>
      </v-list-item-content>
    </v-list-item>

    <!-- TODO -->
    <!-- <v-list-item exact link to="/tools/hexdump">
      <v-list-item-content>
        <v-list-item-title>Hexdump</v-list-item-title>
      </v-list-item-content>
    </v-list-item> -->

  </v-list>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BToolsDrawer',
});
</script>


<style lang="scss" scoped>
</style>
