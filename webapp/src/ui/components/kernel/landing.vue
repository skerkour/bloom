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
        <h2 class="text-h3 mb-5">
          Take back control of your data!
        </h2>
        <h3>
          Bloom is <a href="https://about.gitlab.com/handbook/product/single-application/" target="_blank" rel="noopener">single application</a>
          to automate your tasks, grow your projects and improve collaboration
          in your organization.
        </h3>
        <h3>
          It's like a mix between Intercom, Basecamp and Zapier.
        </h3>
        <h3>
          This is a private Beta. Please help us improve by reporting bugs and sharing feedback
          & ideas using the chatbox
          or by <router-link to="/contact">email</router-link> 😀
        </h3>
        <h3>
          All the code will be open sourced at the end of the private beta, in a few weeks.
        </h3>

        <v-row class="text-center text--darken-1 grey--text pt-4" align="center" justify="center" >
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
        <b-register-form />
      </v-col>
    </v-row>

    <b-footer />
  </v-container>
</template>

<script lang="ts">
import BRegisterForm from '@/ui/components/users/register_form.vue';
import { VueApp } from '@/app/vue';
import BFooter from '@/ui/components/kernel/footer.vue';


export default VueApp.extend({
  name: 'BLanding',
  components: {
    BRegisterForm,
    BFooter,
  },
});
</script>


<style lang="scss" scoped>
</style>
