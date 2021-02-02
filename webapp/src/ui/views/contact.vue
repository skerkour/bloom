<template>
  <v-container class="mt-4 text-center">
    <v-row class="justify-center">
       <v-col cols="12" class="mb-2">
        <h1 class="text-h3">
          Contact
        </h1>
      </v-col>

       <v-col cols="12" sm="10" md="8" class="mb-5">
        <p class="font-weight-regular text-left text-body-1">
          You can contact the Bloom team using one of the following way:
          <ul>
            <li>
              <span>Using the chatbox at the bottom right of this website</span>
            </li>
            <li>
              <span>Using email:</span>&nbsp;
              <a href="mailto:hello@bloom.sh">hello@bloom.sh</a>
            </li>
            <li>
              <span>By saying <i>Hi</i> on Twitter:</span>&nbsp;
              <a href="https://twitter.com/sylvain_kerkour" target="_blank" rel="noopener">@sylvain_kerkour</a>
            </li>
            <li>
              <span>Opening an issue on</span>&nbsp;
              <a href="https://github.com/skerkour/bloom/issues" target="_blank" rel="noopener">GitHub</a>
            </li>
            <li>
              <span>On the fediverse:</span>&nbsp;
              <a href="https://mastodon.social/@42bloom" target="_blank" rel="noopener">@42bloom@mastodon.social</a>
            </li>
          </ul>
        </p>
      </v-col>

    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BContactView',
});
</script>


<style lang="scss" scoped>
</style>
