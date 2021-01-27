<template>
  <v-container fluid>
    <v-row justify="center" class="text-center">
      <v-col cols="10">
        <v-avatar size="84">
          <v-img :src="user.avatarUrl" />
        </v-avatar>
      </v-col>
      <v-col cols="10">
        <h2 class="text-h5">{{ user.name }}</h2>
        <h3 class="text-subtitle-1">@{{ user.username }}</h3>
      </v-col>
      <v-col cols="6" xl="4">
        <p class="text-body-1">Bio: <br />
          {{ user.description }}
        </p>
      </v-col>

    </v-row>
  </v-container>
</template>


<script lang="ts">
import { User } from '@/api/graphql/model';
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';

export default VueApp.extend({
  name: 'BUserHomepage',
  props: {
    user: {
      type: Object as PropType<User>,
      required: true,
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
