<template>
  <v-card elevation="0" outlined>
    <v-card-title>
      <h3 class="text-h5">
        Setup the chatbox widget on your website
      </h3>
    </v-card-title>

    <v-card-text>
      <code>
&lt;script type="text/javascript"&gt; <br/>
window.$bloom = { project: '{{ namespaceId }}' }; <br/>
(function(){d=document;s=d.createElement("script");s.src="{{ baseUrl }}/libs/bloom.js";
s.async=1;d.getElementsByTagName("head")[0].appendChild(s);})(); <br/>
&lt;/script&gt;
      </code>
    </v-card-text>
  </v-card>
</template>


<script lang="ts">
/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { VueApp } from '@/app/vue';
import { ChatboxPreferences } from '@/domain/inbox/model';
import { PropType } from 'vue';

export default VueApp.extend({
  name: 'BChatboxSetupCard',
  props: {
    preferences: {
      required: true,
      type: Object as PropType<ChatboxPreferences>,
    },
  },
  computed: {
    namespaceId(): string {
      return this.$store.state.currentNamespace!.id!;
    },
    baseUrl(): string {
      return this.preferences.base_url;
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
