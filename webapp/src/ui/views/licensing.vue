<template>
  <v-container class="mt-4 text-center">
    <v-row class="justify-center">
      <v-col cols="12" class="mb-2">
        <h1 class="text-h3">
          Licensing
        </h1>
      </v-col>

      <v-col cols="12" sm="10" md="8" class="text-left">
        <p class="font-weight-regular text-body-1">
          Our philosophy is simple: either you contribute back to the commons by opening your work
          too, either you contribute by financing our work. <br /><br />

          We use contaminating licenses to
          avoid behaviors which will damage our vision of Open Source/Free Software and ensure
          that those who benefits from our work contribute back to the community
          by releasing the modified version and the tooling around, or by funding the project.
          <br /><br />

          Our time is as valuable as yours and we don't work for free.<br /><br />

          You can find the license for each project in the <code>LICENSE.txt</code> file
          at the root of the project, and sometimes in subfolders if components have a different
          license.<br /><br />
        </p>
      </v-col>


      <v-col cols="12" sm="10" md="8" class="text-left">
        <h2 class="headline">GNU Affero General Public License v3.0 (AGPL-3.0)</h2>
        <p class="text-body-1">
          Used for server side software.
        </p>
      </v-col>


      <v-col cols="12" sm="10" md="8" class="text-left">
        <h2 class="headline">GNU General Public License v3.0 (GPL-3.0)</h2>
        <p class="text-body-1">
          Used for client side applications or libraries.
        </p>
      </v-col>

    </v-row>

    <v-row class="justify-center mt-3">
      <v-btn color="primary" to="/register">
        Get started for free
      </v-btn>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BLicensingView',
});
</script>


<style lang="scss" scoped>
</style>
