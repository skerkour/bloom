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
window.$bloom = { project: '{{ projectId }}' }; <br/>
(function(){d=document;s=d.createElement("script");s.src="{{ baseUrl }}/libs/bloom.js";
s.async=1;d.getElementsByTagName("head")[0].appendChild(s);})(); <br/>
&lt;/script&gt;
      </code>
    </v-card-text>
  </v-card>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';

export default VueApp.extend({
  name: 'BInboxSetupCard',
  props: {
    baseUrl: {
      type: String as PropType<string>,
      required: true,
    },
    projectId: {
      type: String as PropType<string>,
      required: true,
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
