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

      <p class="ma-0 blm-pointer ml-3 d-none d-sm-flex" @click="centerToday">
        {{ today }}
      </p>

      <v-spacer />

       <v-btn right absolute color="primary" @click="openEventDialog" depressed>
        <v-icon left>mdi-plus</v-icon>
        Create Event
      </v-btn>

      </v-toolbar>

    <div class="fill-height" style="height: calc(100vh - 113px)">
      <v-calendar
        ref="calendar"
        v-model="focus"
        color="error"
      />
    </div>

    <b-calendar-event-dialog
      v-model="showEventDialog"
      :event="currentEvent"
    />

  </v-container>
</template>


<script lang="ts">
/* eslint-disable @typescript-eslint/no-explicit-any */
import { VueApp } from '@/app/vue';
import { CalendarEvent } from '@/domain/calendar/model';
import moment from 'moment';
import BCalendarEventDialog from '@/ui/components/calendar/event_dialog.vue';

export default VueApp.extend({
  name: 'BCalendarView',
  components: {
    BCalendarEventDialog,
  },
  data() {
    return {
      now: moment().format('YYYY-MM-DD'),
      focus: moment().format('YYYY-MM-DD'),
      today: moment().format('dddd ll'),
      showEventDialog: false,
      currentEvent: null as CalendarEvent | null,
      events: [] as CalendarEvent[],
    };
  },
  computed: {
    vuetifyEvents: {
      get(): any[] {
        return this.events.map((ev) => {
          const event = ev as any;
          event.start_at_date = moment(ev.start_at).toDate();
          event.end_at_date = moment(ev.end_at).toDate();
          event.start = this.formatDateForVuetify(event.start_at_date);
          event.end = this.formatDateForVuetify(event.end_at_date);
          event.name = ev.title || '(No title)';
          return event;
        });
      },
    },
  },
  methods: {
    centerToday() {
      this.focus = this.now;
    },
    openEventDialog() {
      this.showEventDialog = true;
    },
    closeEventDialog() {
      this.showEventDialog = false;
      this.currentEvent = null;
    },
    formatDateForVuetify(date: Date) {
      return `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate()} ${date.getHours()}:${date.getMinutes()}`;
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
