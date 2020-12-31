<template>
  <component :is="layout">
    <router-view/>
  </component>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';
import BLayoutDefault from './layouts/default.vue';
import BLayoutAuth from './layouts/auth.vue';
import BLayoutBare from './layouts/bare.vue';


export default VueApp.extend({
  name: 'App',
  components: {
    BLayoutDefault,
    BLayoutAuth,
    BLayoutBare,
  },
  computed: {
    layout(): string {
      const layout = this.$route.meta.layout
        ? `b-layout-${this.$route.meta.layout}` : 'b-layout-default';
      return layout;
    },
  },
  created() {
    this.$vuetify.theme.dark = this.$store.state.darkMode;
  },
});
</script>


<style lang="scss">
.bloom-pointer, .blm-pointer {
  cursor: pointer;
}

a {
  text-decoration: none;
}

// .theme--light.v-navigation-drawer {
//   background-color: #fafafa;
// }
</style>
