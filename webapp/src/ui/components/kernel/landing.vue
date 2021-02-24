<template>
  <!-- <v-container fluid>
    <v-row class="text-center" justify="center">
      <v-col cols="12" md="6">
        <h1 class="display-2 mb-4">Bloom work for you 24 / 7</h1>
        <v-img
          src="/assets/imgs/minions.svg"
          class="my-3"
          contain
          height="400"
        />
      </v-col>
    </v-row>
  </v-container> -->
  <v-container fluid>
    <v-row class="text-center" justify="center" align="center">
      <v-col cols="12" md="6">
        <h2 class="text-h3 mb-5 mt-5 pt-4">
          The all-in-one workspace
        </h2>
        <!-- <h3 class="text-h4">
          Take back control of your productivity!
        </h3>
        <h4 class="text-h5">
          Bloom is the easiest way to de-Google your life and business.
        </h4>
        -->
        <h4 class="text-h5">
          Open Source Inbox, Calendar, Files, Contacts & much more
        </h4>

        <v-row class="text-center text--darken-1 grey--text pt-4 mt-5"
          align="center" justify="center" >
          <v-col cols="12" sm="6" md="4" align="center" justify="center" class="pb-3">
            <h5 class="title mb-2">#1 Product of the Day</h5>
            <a href="https://www.producthunt.com/posts/bloom-05383d81-983e-4d4d-b22f-78dd91066cda" target="_blank" rel="noopener" class="blm-pointer">
              <v-img alt="Product Hunt" src="/assets/imgs/landing/ph.svg" width="200px" contain>
                <template v-slot:placeholder>
                  <v-row
                    class="fill-height ma-0"
                    align="center"
                    justify="center"
                  >
                    <v-progress-circular indeterminate color="grey lighten-2" />
                  </v-row>
                </template>
              </v-img>
            </a>
          </v-col>
          <v-col cols="12" sm="6" md="4" align="center" justify="center" class="pb-3">
            <h5 class="title mb-2">#1 on</h5>
            <a href="https://news.ycombinator.com/item?id=20105567" target="_blank" rel="noopener" class="blm-pointer">
              <v-img alt="Hacker news" src="/assets/imgs/landing/hn.svg" width="200px" >
                <template v-slot:placeholder>
                  <v-row
                    class="fill-height ma-0"
                    align="center"
                    justify="center"
                  >
                    <v-progress-circular indeterminate color="grey lighten-2" />
                  </v-row>
                </template>
              </v-img>
            </a>
          </v-col>
        </v-row>

      </v-col>

      <v-col cols="12" md="6">
        <b-register-form class="pt-5"/>
      </v-col>
    </v-row>


    <v-row class="my-5 py-5" justify="center">
      <v-col cols="12" md="8" class="text-center">
        <h2 class="text-h3 font-weight-medium">
          I want to
        </h2>
      </v-col>

      <v-col cols="12" md="10" lg="8" xl="6">
        <v-row>
          <v-col cols="12" md="6" class="text-center my-5">
            <v-row justify="center">
              <v-col cols="12">
                <v-img alt="Super Hero"
                  src="/assets/imgs/landing/superhero.svg" height="200px" contain />
              </v-col>
              <v-col cols="12">
                <h3 class="text-h4">
                  De-Google my life
                </h3>
              </v-col>

              <v-col cols="12">
                <v-btn depressed color="success" to="/register" class="b-btn-lower">
                  Bloom for Individuals
                </v-btn>
              </v-col>
            </v-row>
          </v-col>


          <v-col cols="12" md="6" class="text-center my-5">
            <v-row justify="center">
              <v-col cols="12">
                <v-img alt="Productivity"
                  src="/assets/imgs/landing/productivity.svg" height="200px" contain />
              </v-col>
              <v-col cols="12">
                <h3 class="text-h4">
                  Grow my business
                </h3>
              </v-col>

              <v-col cols="12">
                <v-btn depressed color="primary" to="/register" class="b-btn-lower">
                  Bloom for Businesses
                </v-btn>
              </v-col>
            </v-row>
          </v-col>
        </v-row>
      </v-col>

    </v-row>

    <b-features class="b-features" />


    <b-footer />
  </v-container>
</template>

<script lang="ts">
import BRegisterForm from '@/ui/components/kernel/register_form.vue';
import { VueApp } from '@/app/vue';
import BFooter from '@/ui/components/kernel/footer.vue';
import BFeatures from './features.vue';


export default VueApp.extend({
  name: 'BLanding',
  components: {
    BRegisterForm,
    BFooter,
    BFeatures,
  },
});
</script>


<style lang="scss" scoped>
.b-features {
  margin-top: 80px;
}

.b-btn-lower {
  text-transform: none !important;
}
</style>
