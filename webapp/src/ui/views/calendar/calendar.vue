<template>
  <v-container fluid class="pa-0 ma-0">

    <v-toolbar elevation="0">
      <v-btn text icon color="primary" @click="$refs.calendar.prev()">
        <v-icon dark>mdi-chevron-left</v-icon>
      </v-btn>
      <v-btn text icon color="primary" @click="$refs.calendar.next()">
        <v-icon dark>mdi-chevron-right</v-icon>
      </v-btn>

        <!-- <v-spacer /> -->

      <p class="ma-0 blm-pointer ml-3" @click="centerToday">
        {{ today }}
      </p>

      </v-toolbar>

    <div class="fill-height" style="height: calc(100vh - 56px)">
      <v-calendar
        ref="calendar"
        v-model="focus"
        color="error"
      />
    </div>

  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import moment from 'moment';

export default VueApp.extend({
  name: 'BCalendarView',
  data() {
    return {
      now: moment().format('YYYY-MM-DD'),
      focus: moment().format('YYYY-MM-DD'),
      today: moment().format('dddd ll'),
    };
  },
  methods: {
    centerToday() {
      this.focus = this.now;
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
