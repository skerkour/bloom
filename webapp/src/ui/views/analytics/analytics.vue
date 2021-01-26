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

    <v-row justify="center" v-if="project">
      <v-col cols="12">
        <b-analytics-visits :visits="visits" />
      </v-col>

      <v-col cols="12" md="6">
        <b-analytics-pages :pages="pages" />
      </v-col>
      <v-col cols="12" md="6">
        <b-analytics-referrers :referrers="referrers" />
      </v-col>


      <v-col cols="12" md="4">
        <b-analytics-devices :devices="devices" />
      </v-col>
      <v-col cols="12" md="4">
        <b-analytics-browsers :browsers="browsers" />
      </v-col>
      <v-col cols="12" md="4">
        <b-analytics-oses :oses="oses" />
      </v-col>

      <v-col cols="12">
        <b-analytics-countries :countries="countries" />
      </v-col>

      <v-col cols="12">
        <b-analytics-events :events="events" />
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import BAnalyticsBrowsers from '@/ui/components/growth/analytics_browsers.vue';
import BAnalyticsCountries from '@/ui/components/growth/analytics_countries.vue';
import BAnalyticsDevices from '@/ui/components/growth/analytics_devices.vue';
import BAnalyticsEvents from '@/ui/components/growth/analytics_events.vue';
import BAnalyticsOses from '@/ui/components/growth/analytics_oses.vue';
import BAnalyticsPages from '@/ui/components/growth/analytics_pages.vue';
import BAnalyticsReferrers from '@/ui/components/growth/analytics_referrers.vue';
import BAnalyticsVisits from '@/ui/components/growth/analytics_visits.vue';
import {
  Project, AnalyticsVisit, AnalyticsEvent, AnalyticsPage, AnalyticsReferrer,
  AnalyticsDevice, AnalyticsBrowser, AnalyticsOs, AnalyticsCountry,
} from '@/api/graphql/model';

export default VueApp.extend({
  name: 'BAnalyticsView',
  components: {
    BAnalyticsEvents,
    BAnalyticsBrowsers,
    BAnalyticsCountries,
    BAnalyticsDevices,
    BAnalyticsOses,
    BAnalyticsPages,
    BAnalyticsReferrers,
    BAnalyticsVisits,
  },
  data() {
    return {
      loading: false,
      error: '',
      project: null as Project | null,
    };
  },
  computed: {
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
    visits(): AnalyticsVisit[] {
      return this.project ? this.project.analytics.visits : [];
    },
    events(): AnalyticsEvent[] {
      return this.project ? this.project.analytics.events : [];
    },
    pages(): AnalyticsPage[] {
      return this.project ? this.project.analytics.pages : [];
    },
    referrers(): AnalyticsReferrer[] {
      return this.project ? this.project.analytics.referrers : [];
    },
    devices(): AnalyticsDevice[] {
      return this.project ? this.project.analytics.devices : [];
    },
    browsers(): AnalyticsBrowser[] {
      return this.project ? this.project.analytics.browsers : [];
    },
    oses(): AnalyticsOs[] {
      return this.project ? this.project.analytics.oses : [];
    },
    countries(): AnalyticsCountry[] {
      return this.project ? this.project.analytics.countries : [];
    },
  },
  created() {
    this.fetchData();
  },
  methods: {
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.project = await this.$growthService.fetchAnalytics(this.projectFullPath);
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
