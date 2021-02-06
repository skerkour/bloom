<template>
  <v-dialog
    v-model="show"
    @keydown.esc="close"
    @click:outside="close"
    persistent
    scrollable
    width="50%"
    :fullscreen="$vuetify.breakpoint.xsOnly"
  >
    <v-card>
      <v-card-title class="headline" v-if="event === null">
        <h3 class="headline mb-0">Create new event</h3>
        <v-spacer />

        <v-btn text @click="cancel">
          Cancel
        </v-btn>
        <v-btn color="primary" @click="createEvent" depressed>
          Create
        </v-btn>
      </v-card-title>
      <v-card-title dark class="headline" v-else>
        <h3 class="headline mb-0">
          <h3 class="headline mb-0">{{ event.title }}</h3>
        </h3>
        <v-spacer />
        <v-btn text @click="cancel">
          Cancel
        </v-btn>
        <v-btn color="primary" @click="updateEvent" depressed>
          Save
        </v-btn>
        <v-menu bottom left>
          <template v-slot:activator="{ on }">
            <v-btn icon v-on="on">
              <v-icon>mdi-dots-vertical</v-icon>
            </v-btn>
          </template>

          <v-list>
            <v-list-item @click="deleteEvent">
              <v-list-item-icon>
                <v-icon>mdi-delete</v-icon>
              </v-list-item-icon>
              <v-list-item-title>Delete event</v-list-item-title>
            </v-list-item>
          </v-list>
        </v-menu>
      </v-card-title>


      <v-card-text>
        <v-container fluid grid-list-lg>
          <v-row>
            <v-col cols="12">
              <v-alert icon="mdi-alert-circle" :value="error !== ''" type="error" dismissible>
                {{ error }}
              </v-alert>
            </v-col>

            <v-col cols="12">
              <v-text-field label="Title" v-model="title" outlined/>
            </v-col>

            <v-col cols="6">
              <v-menu
                ref="startAtDateMenu"
                v-model="startAtDateMenu"
                :close-on-content-click="false"
                :nudge-right="40"
                transition="scale-transition"
                offset-y
                min-width="290px"
              >
                <template v-slot:activator="{ on }">
                  <v-text-field
                    :value="formattedStartAt"
                    label="Start at"
                    prepend-icon="mdi-calendar"
                    readonly
                    v-on="on"
                  />
                </template>
                <v-date-picker v-model="vuetifyStartAt" @input="startAtDateMenu = false" />
              </v-menu>
            </v-col>

            <v-col cols="6">
              <v-menu
                ref="startAtTimeMenu"
                v-model="startAtTimeMenu"
                :close-on-content-click="false"
                :nudge-right="40"
                transition="scale-transition"
                offset-y
                min-width="290px"
              >
                <template v-slot:activator="{ on }">
                  <v-text-field
                    :value="startAtTime"
                    label="Start at"
                    prepend-icon="mdi-clock-outline"
                    readonly
                    v-on="on"
                  />
                </template>
                <v-time-picker
                  v-model="startAtTime"
                  format="24hr"
                  @click:minute="startAtTimeMenu = false"
                ></v-time-picker>
              </v-menu>
            </v-col>

            <v-col cols="6">
              <v-menu
                ref="endAtDateMenu"
                v-model="endAtDateMenu"
                :close-on-content-click="false"
                :nudge-right="40"
                transition="scale-transition"
                offset-y
                min-width="290px"
              >
                <template v-slot:activator="{ on }">
                  <v-text-field
                    :value="formattedEndAt"
                    label="End at"
                    prepend-icon="mdi-calendar"
                    readonly
                    v-on="on"
                  />
                </template>
                <v-date-picker v-model="vuetifyEndAt" @input="endAtDateMenu = false" />
              </v-menu>
            </v-col>

            <v-col cols="6">
              <v-menu
                ref="endAtTimeMenu"
                v-model="endAtTimeMenu"
                :close-on-content-click="false"
                :nudge-right="40"
                transition="scale-transition"
                offset-y
                min-width="290px"
              >
                <template v-slot:activator="{ on }">
                  <v-text-field
                    v-model="endAtTime"
                    label="End at"
                    prepend-icon="mdi-clock-outline"
                    readonly
                    v-on="on"
                  />
                </template>
                <v-time-picker
                  v-model="endAtTime"
                  format="24hr"
                  @click:minute="endAtTimeMenu = false"
                ></v-time-picker>
              </v-menu>
            </v-col>

            <v-col cols="12">
              <v-text-field label="Location" v-model="location" outlined />
            </v-col>

            <v-col cols="12">
              <v-textarea
                label="Description"
                v-model="description"
                outlined
              />
            </v-col>

          </v-row>
        </v-container>
      </v-card-text>

    </v-card>
  </v-dialog>
</template>


<script lang="ts">
/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { VueApp } from '@/app/vue';
import {
  CalendarEvent, CreateEvent, DeleteEvent, UpdateEvent,
} from '@/domain/calendar/model';
import moment from 'moment';
import { PropType } from 'vue';

export default VueApp.extend({
  name: 'BCalendarEventDialog',
  props: {
    value: {
      type: Boolean as PropType<boolean>,
      required: true,
    },
    event: {
      type: Object as PropType<CalendarEvent | null>,
      required: false,
      default: null,
    },
  },
  data() {
    return {
      title: '',
      description: '',
      location: '',
      now: new Date(),
      startAt: new Date(),
      startAtTime: '08:00',
      startAtDateMenu: false,
      startAtTimeMenu: false,
      endAt: new Date(),
      endAtTime: '09:00',
      endAtDateMenu: false,
      endAtTimeMenu: false,
      error: '',
      loading: false,
    };
  },
  computed: {
    show: {
      get(): boolean {
        return this.value;
      },
      set(value: boolean) {
        this.$emit('input', value);
        this.$emit('closed');
      },
    },
    formattedStartAt: {
      get(): string {
        return this.startAt ? moment(this.startAt).format('dddd, MMMM Do YYYY') : '';
      },
    },
    formattedEndAt: {
      get(): string {
        return this.endAt ? moment(this.endAt).format('dddd, MMMM Do YYYY') : '';
      },
    },
    vuetifyStartAt: {
      get(): string {
        return this.startAt.toISOString().substr(0, 10);
      },
      set(value: string) {
        this.startAt = new Date(value);
      },
    },
    vuetifyEndAt: {
      get(): string {
        return this.endAt.toISOString().substr(0, 10);
      },
      set(value: string) {
        this.endAt = new Date(value);
      },
    },
  },
  watch: {
    event(event: CalendarEvent | null) {
      if (event) {
        this.title = event.title;
        this.description = event.description;
        this.location = event.location;
        this.startAt = moment(event.start_at).toDate();
        this.startAtTime = this.dateToTimeSring(this.startAt);
        this.endAt = moment(event.end_at).toDate();
        this.endAtTime = this.dateToTimeSring(this.endAt);
      } else {
        this.emptyFields();
      }
    },
  },
  methods: {
    cancel() {
      this.close();
      this.emptyFields();
    },
    emptyFields() {
      this.title = '';
      this.description = '';
      this.location = '';
      this.startAt = this.now;
      this.endAt = this.now;
    },
    close() {
      this.show = false;
      this.startAtDateMenu = false;
      this.startAtTimeMenu = false;
      this.endAtDateMenu = false;
      this.endAtTimeMenu = false;
      this.error = '';
      this.loading = false;
    },
    dateToTimeSring(date: Date): string {
      const hours = date.getHours().toString();
      const minutes = date.getMinutes().toString();
      return `${hours.padStart(2, '0')}:${minutes.padStart(2, '0')}`;
    },
    timeToDate(date: Date, time: string): Date {
      const ret = date;
      ret.setHours(parseInt(time[0] + time[1], 10));
      ret.setMinutes(parseInt(time[3] + time[4], 10));
      return ret;
    },
    async createEvent() {
      this.error = '';
      this.loading = true;

      const startAt = this.timeToDate(this.startAt, this.startAtTime);
      const endAt = this.timeToDate(this.endAt, this.endAtTime);
      const input: CreateEvent = {
        namespace_id: this.$store.state.currentNamespace!.id,
        location: this.location,
        title: this.title,
        description: this.description,
        start_at: startAt.toISOString(),
        end_at: endAt.toISOString(),
      };

      try {
        const event = await this.$calendarService.createEvent(input);
        this.$emit('created', event);
        this.close();
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async updateEvent() {
      this.error = '';
      this.loading = true;

      const startAt = this.timeToDate(this.startAt, this.startAtTime);
      const endAt = this.timeToDate(this.endAt, this.endAtTime);
      const input: UpdateEvent = {
        event_id: this.event!.id,
        title: this.title,
        description: this.description,
        location: this.location,
        start_at: startAt.toISOString(),
        end_at: endAt.toISOString(),
      };


      try {
        const event = await this.$calendarService.updateEvent(input);
        this.$emit('updated', event);
        this.close();
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async deleteEvent() {
      this.error = '';
      this.loading = true;
      const input: DeleteEvent = {
        event_id: this.event!.id,
      };

      try {
        await this.$calendarService.deleteEvent(input);
        this.$emit('deleted', this.event);
        this.close();
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
