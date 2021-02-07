<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" md="4" v-for="card in cards" :key="card.title">
        <router-link :to="card.link">
          <v-card outlined class="mx-auto blm-project-card d-flex flex-column align-stretch">
            <v-card-title>
              <p class="display-1 text--primary">
                <v-icon>{{ card.icon }}</v-icon>
                {{ card.title }}
              </p>
            </v-card-title>
            <v-card-text class="text-body-1">
              {{ card.description }}
            </v-card-text>
          </v-card>
        </router-link>
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BAdminDashboardView',
  data() {
    return {
      cards: [
        {
          title: 'Users',
          icon: 'mdi-account',
          description: '',
          link: '/admin/users',
        },
        {
          title: 'Groups',
          icon: 'mdi-account-group',
          description: '',
          link: '/admin/groups',
        },
      ],
    };
  },
});
</script>


<style lang="scss" scoped>
</style>
