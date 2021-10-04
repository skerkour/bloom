<template>
  <v-footer height="auto" class="text-left" color="white">
    <v-container>
      <v-row class="text-center text-sm-left">
        <v-col cols="12" sm="4" class="pa-4">
          <h5 class="text-h5 pb-3">Organization</h5>
          <ul>
            <li><router-link class="nav-link" to="/about">About</router-link></li>
            <li><router-link class="nav-link" to="/contact">Contact</router-link></li>
            <li><router-link class="nav-link" to="/security">Security</router-link></li>
            <li><router-link class="nav-link" to="/terms">Terms</router-link></li>
            <li><router-link class="nav-link" to="/privacy">Privacy</router-link></li>
          </ul>
        </v-col>

        <v-col cols="12" sm="4" class="pa-4">
          <h5 class="text-h5 pb-3">Product</h5>
          <ul>
            <li><router-link class="nav-link" to="/pricing">Pricing</router-link></li>
            <li><router-link class="nav-link" to="/features">Features</router-link></li>
            <li><a class="nav-link" href="https://github.com/skerkour/bloom/projects" target="_blank" rel="noopener">Roadmap</a></li>
            <li><router-link class="nav-link" to="/faq">F.A.Q</router-link></li>
            <li><router-link class="nav-link" to="/licensing">Licensing</router-link></li>
            <!-- <li><a class="nav-link" href="https://bloom.sh/status/bloom/bloom" target="_blank" rel="noopener">Status</a></li> -->
          </ul>
        </v-col>

        <v-col cols="12" sm="4" class="pa-4">
          <h4 class="text-h5 pb-3">Community</h4>
          <ul>
            <li><a class="nav-link" href="https://kerkour.com" target="_blank" rel="noopener">Blog</a></li>
            <li><a class="nav-link" href="https://kerkour.com/subscribe" target="_blank" rel="noopener">Newsletter</a></li>
            <li><a class="nav-link" href="https://twitter.com/SylvainKerkour" target="_blank" rel="noopener">Twitter</a></li>
            <li><a class="nav-link" href="https://github.com/skerkour/bloom" target="_blank" rel="noopener">GitHub</a></li>
            <li><a class="nav-link" href="https://social.kerkour.com/@sylvain" target="_blank" rel="me noopener">Mastodon</a></li>
          </ul>
        </v-col>

        <v-col cols="12" class="blm-footer-year-col">
          <p class="text-center blm-footer-year">
            <span>&copy; 2018 - {{ new Date().getFullYear() }} Bloom</span>
          </p>
        </v-col>

      </v-row>
    </v-container>
  </v-footer>

</template>


<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BFooter',
});
</script>


<style lang="scss" scoped>
.theme--dark.v-footer {
  a {
    color: white;
  }
}

.theme--light.v-footer {
  a {
    color: black;
  }
}

ul li {
  margin-bottom: 10px;
}

ul {
  list-style-type: none;
}

ol, .v-application ul {
  padding-left: 0px;
}

.blm-footer-year {
  margin-bottom: 0px;
}
.blm-footer-year-col {
  padding-bottom: 0px;
}

</style>
