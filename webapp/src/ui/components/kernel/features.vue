<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" md="10" xl="8">


        <v-row justify="center" class="mb-5">
          <v-col cols="12" sm="6" class="text-center">
            <img alt="secure" src="/assets/imgs/landing/inbox.png" height="200" />
          </v-col>

          <v-col cols="12" sm="6" class="text-center">
            <h4 class="text-h4">
              Universal Inbox
            </h4>
            <p class="mt-5">
              Our unified inbox let you be always present when your customers or your relatives
              need you. <br />
              Spam free.
            </p>
          </v-col>
        </v-row>


        <v-row justify="center" class="mt-5 mb-5">
          <v-col cols="12" sm="6" class="text-center" order="2" order-sm="0">
            <h4 class="text-h4">
              Shared files
            </h4>
            <p class="mt-5">
              Keep photos, stories, designs, drawings, recordings, videos, and more. <br />
              In Files, your data can be reached from any smartphone, tablet, or computer.
                So wherever you go, your files follow.
            </p>
          </v-col>

          <v-col cols="12" sm="6" class="text-center">
            <img alt="secure" src="/assets/imgs/landing/files.png" height="200" />
          </v-col>
        </v-row>


        <v-row justify="center" class="mt-5 mb-5">
          <v-col cols="12" sm="6" class="text-center">
            <img alt="secure" src="/assets/imgs/landing/newsletter.png" height="200" />
          </v-col>

          <v-col cols="12" sm="6" class="text-center">
            <h4 class="text-h4">
              Simplified Newsletter
            </h4>
            <p class="mt-5">
              Bloom makes it easy for content creators to send newsletters
              and build an engaged audience.
            </p>
          </v-col>
        </v-row>


        <v-row justify="center" class="mt-5 mb-5">
          <v-col cols="12" sm="6" class="text-center" order="2" order-sm="0">
            <h4 class="text-h4">
              Privacy-focused Analytics
            </h4>
            <p class="mt-5">
              Bloom provides a simple, light-weight and focused alternative to Google Analytics.
            </p>
          </v-col>

          <v-col cols="12" sm="6" class="text-center">
            <img alt="secure" src="/assets/imgs/landing/analytics.png" height="200" />
          </v-col>
        </v-row>


      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BFeatures',
});
</script>


<style lang="scss" scoped>
img {
  border: 1px solid #efefef;
  border-radius: 4px;
}
</style>
