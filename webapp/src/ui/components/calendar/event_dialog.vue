<template>
  <v-dialog
    v-model="show"
    max-width="800px"
    scrollable
  >
    <v-card>

    </v-card>
  </v-dialog>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { CalendarEvent } from '@/domain/calendar/model';
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
  computed: {
    show: {
      get(): boolean {
        return this.value;
      },
      set(value: boolean) {
        this.$emit('input', value);
      },
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
