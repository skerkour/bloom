<template>
  <v-container fluid>

    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center" class="text-center" v-if="loading">
      <v-col cols="12" sm="10" md="8" xl="6">
        <v-progress-circular
          :size="50"
          color="primary"
          indeterminate
        />
      </v-col>
    </v-row>

    <v-row justify="center" v-if="analytics">
      <v-col cols="12">
        <b-analytics-visits :visits="analytics.visits" />
      </v-col>

      <v-col cols="12" md="6">
        <b-analytics-pages :pages="analytics.pages" />
      </v-col>
      <v-col cols="12" md="6">
        <b-analytics-referrers :referrers="analytics.referrers" />
      </v-col>


      <v-col cols="12" md="6">
        <b-analytics-devices :devices="analytics.devices" />
      </v-col>
      <!-- <v-col cols="12" md="4">
        <b-analytics-browsers :browsers="browsers" />
      </v-col>
      <v-col cols="12" md="4">
        <b-analytics-oses :oses="oses" />
      </v-col> -->

      <!-- <v-col cols="12">
        <b-analytics-countries :countries="countries" />
      </v-col> -->

      <v-col cols="12" md="6">
        <b-analytics-events :events="analytics.events" />
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { Analytics } from '@/domain/analytics/model';
import BAnalyticsDevices from '@/ui/components/analytics/devices.vue';
import BAnalyticsEvents from '@/ui/components/analytics/events.vue';
import BAnalyticsPages from '@/ui/components/analytics/pages.vue';
import BAnalyticsReferrers from '@/ui/components/analytics/referrers.vue';
import BAnalyticsVisits from '@/ui/components/analytics/visits.vue';

export default VueApp.extend({
  name: 'BAnalyticsView',
  components: {
    BAnalyticsEvents,
    BAnalyticsDevices,
    BAnalyticsPages,
    BAnalyticsReferrers,
    BAnalyticsVisits,
  },
  data() {
    return {
      loading: false,
      error: '',
      analytics: null as Analytics | null,
    };
  },
  created() {
    this.fetchData();
  },
  methods: {
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.analytics = await this.$analyticsService.fetchAnalytics();
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
