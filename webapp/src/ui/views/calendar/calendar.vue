<template>
  <v-container fluid class="pa-0 ma-0">

    <v-toolbar elevation="0">
      <v-btn text icon color="primary" @click="$refs.calendar.prev()">
        <v-icon dark>mdi-chevron-left</v-icon>
      </v-btn>
      <v-btn text icon color="primary" @click="$refs.calendar.next()">
        <v-icon dark>mdi-chevron-right</v-icon>
      </v-btn>

      <p class="ma-0 blm-pointer ml-3 d-none d-sm-flex" @click="centerToday">
        {{ today }}
      </p>

      <v-spacer />

       <v-btn right absolute color="primary" @click="openEventDialog" depressed>
        <v-icon left>mdi-plus</v-icon>
        Create Event
      </v-btn>

      </v-toolbar>

    <div class="fill-height b-calendar-wrapper">
      <v-calendar
        ref="calendar"
        v-model="focus"
        color="error"
        :type="calendarType"
        :start="start"
        :end="end"
        :now="now"
        @change="onCalendarChanged"
        @click:event="editEvent"
        :events="vuetifyEvents"
      />
    </div>

    <b-calendar-event-dialog
      v-model="showEventDialog"
      :event="currentEvent"
      @closed="closeEventDialog"
      @created="eventCreated"
      @updated="eventUpdated"
      @deleted="eventDeleted"
    />

  </v-container>
</template>


<script lang="ts">
/* eslint-disable @typescript-eslint/no-explicit-any, @typescript-eslint/no-non-null-assertion */
import { VueApp } from '@/app/vue';
import { CalendarEvent, GetEvents } from '@/domain/calendar/model';
import moment from 'moment';
import BCalendarEventDialog from '@/ui/components/calendar/event_dialog.vue';

export default VueApp.extend({
  name: 'BCalendarView',
  components: {
    BCalendarEventDialog,
  },
  data() {
    return {
      loading: false,
      error: '',
      calendarType: 'month',
      now: moment().format('YYYY-MM-DD'),
      focus: moment().format('YYYY-MM-DD'),
      today: moment().format('dddd ll'),
      showEventDialog: false,
      currentEvent: null as CalendarEvent | null,
      events: [] as CalendarEvent[],
      start: moment().startOf('month').format('YYYY-MM-DD'),
      end: moment().endOf('month').format('YYYY-MM-DD'),
    };
  },
  computed: {
    vuetifyEvents: {
      get(): any[] {
        return this.events.map((ev) => {
          const event = {} as any;
          event.event = ev;
          event.start = this.formatDateForVuetify(moment(ev.start_at).toDate());
          event.end = this.formatDateForVuetify(moment(ev.end_at).toDate());
          event.name = ev.title || '(No title)';
          return event;
        });
      },
    },
  },
  created() {
    this.fetchEvents();
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
    async fetchEvents(start?: string, end?: string) {
      this.loading = true;
      this.error = '';

      if (!start) {
        start = moment(this.start).toISOString();
      }
      if (!end) {
        end = moment(this.end).toISOString();
      }

      const input: GetEvents = {
        namespace_id: this.$store.state.currentNamespace!.id,
        start_at: start,
        end_at: end,
      };

      try {
        this.events = await this.$calendarService.fetchEvents(input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    onCalendarChanged(to: any) {
      let start = to.start.date;
      if (to.start.time === '') {
        start += ' 00:00:00';
      } else {
        start += ` ${to.start.time}`;
      }

      let end = to.end.date;
      if (to.end.time === '') {
        end += ' 23:59:59';
      } else {
        end += ` ${to.end.time}`;
      }

      this.fetchEvents(moment(start).toISOString(), moment(end).toISOString());
    },
    editEvent(event: { event: { event: CalendarEvent }}) {
      this.currentEvent = event.event.event;
      this.openEventDialog();
    },
    eventCreated(event: CalendarEvent) {
      this.events.push(event);
    },
    eventDeleted(event: CalendarEvent) {
      this.events = this.events.filter((ev) => ev.id !== event.id);
    },
    eventUpdated(updatedEvent: CalendarEvent) {
      this.events = this.events.map((event) => {
        if (event.id === updatedEvent.id) {
          return updatedEvent;
        }
        return event;
      });
    },
  },
});
</script>


<style lang="scss" scoped>
@import '~vuetify/src/styles/styles.sass';

.b-calendar-wrapper {
  @media #{map-get($display-breakpoints, 'sm-and-down')} {
    height: calc(100vh - 160px);
  }
  @media #{map-get($display-breakpoints, 'md-and-up')} {
    height: calc(100vh - 113px);
  }
}
</style>
