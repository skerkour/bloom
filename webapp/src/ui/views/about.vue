<template>
  <v-container class="mt-4 text-center text-body-1">
    <v-row class="justify-center">

      <v-col cols="12" class="text-center">
        <h1 class="text-h3">
          Bloom
        </h1>
      </v-col>

      <v-col cols="12" sm="10" md="8" class="text-center">
        <p class="font-weight-regular">
          We believe that everyone has a right to privacy, whether it be individuals or
          businesses. <br />
          By thinking privacy and sustainability first we are creating a path to exit the
            exploitative business practices of today's internet giants, based on attention
            monetization and behavior manipulaiton. <br /><br />

          We believe that sharing and collaboration is the only way to build a
          sustainable future in a finite world, and we are building this future today.
        </p>
      </v-col>


      <v-col cols="12" class="text-center">
        <h1 class="display-1 font-weight-medium">
          Mission
        </h1>
      </v-col>

      <v-flex xs12 sm10 md8 class="text-center">
        <p class="font-weight-regular text-body-1">
          Empowering people with open technologies.
        </p>
      </v-flex>


      <v-col cols="12" class="text-center">
        <h1 class="display-1 font-weight-medium">
          Values
        </h1>
      </v-col>

      <v-col cols="12" sm="10" md="8" class="text-center">
        <p class="font-weight-regular text-body-1">
          Cooperation, Acuity, Transparency, Sustainability (<b>CATS</b> 🐱).
        </p>
      </v-col>


      <v-col cols="12" class="text-center">
        <h1 class="display-1 font-weight-medium">
          The Crew
        </h1>
      </v-col>

      <v-col cols="12" sm="3" class="text-center">
        <a href="https://kerkour.com" target="_blank" rel="noopener">
          <v-avatar size="100px" color="grey lighten-4">
            <img src="/assets/imgs/sylvain.jpg" alt="Sylvain Kerkour picture">
          </v-avatar>
          <h3 class="mt-3">
            Sylvain Kerkour
          </h3>
          <p>France 🇫🇷</p>
        </a>
      </v-col>

      <v-col cols="12" class="text-center">
        <h1 class="display-1 font-weight-medium">
          Want to learn more?
        </h1>
      </v-col>

      <v-col cols="12" class="text-center">
        <p class="text-body-1">
          We share strong opinions and crazy ideas about Open Source, minimalism and sustainability
          on the blog
      <a href="https://kerkour.com" target="_blank" rel="noopener">kerkour.com</a>.
        </p>
      </v-col>

    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BAboutView',
});
</script>


<style lang="scss" scoped>
</style>
