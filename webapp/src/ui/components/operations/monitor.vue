<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>


    <v-row>
      <v-col v-if="editing">
        <v-btn @click="cancel" depressed :loading="loading" >
          Cancel
        </v-btn>
      </v-col>

      <v-spacer />

      <v-col class="text-right">
        <v-btn class="mr-3"
          @click="deleteMonitor"
          depressed
          color="error"
          :loading="loading"
          v-if="monitor && editing">
          <v-icon left>mdi-delete</v-icon>
          Delete
        </v-btn>
        <v-btn @click="updateClicked" depressed color="primary" :loading="loading"
          v-if="monitor && !editing">
          <v-icon left>mdi-pencil</v-icon>
          Edit
        </v-btn>
        <v-btn @click="update" depressed color="success" :loading="loading" v-if="updating">
          <v-icon left>mdi-content-save</v-icon>
          Save changes
        </v-btn>
        <v-btn @click="create" depressed color="success" :loading="loading" v-if="!monitor">
          Create monitor
        </v-btn>
      </v-col>
    </v-row>

    <v-row v-if="!loading && editing">
      <v-container fluid>
        <v-row v-if="editing">
          <v-col cols="12">
            <div class="headline" v-if="!monitor">
              New monitor
            </div>
            <div class="headline" v-else>
              Update monitor
            </div>
          </v-col>
        </v-row>


        <v-row>
          <v-col cols="12" sm="6" lg="4">
            <v-text-field
              v-model="name"
              label="Name"
            />
          </v-col>
        </v-row>

        <v-row v-if="monitor">
          <v-col cols="12" sm="6" xl="4">
            <v-switch
              v-model="isActive"
              :label="activeLabel"
            />
          </v-col>
        </v-row>

        <v-row>
          <v-col cols="12" sm="6" lg="4">
            <v-text-field
              v-model="endpoint"
              label="Endpoint"
              placeholder="https://example.com"
            />
          </v-col>
        </v-row>

        <v-row>
          <v-col cols="12" sm="6" lg="4">
            <v-select
              :items="httpMethods"
              label="HTTP Method"
              v-model="httpMethod"
            />
          </v-col>
        </v-row>

        <v-row>
          <v-col cols="12" sm="6" xl="4">
            <v-text-field
              v-model="minHTTPStatusCode"
              hide-details
              type="number"
              label="Minimum expected Status Code"
              hint="The monitor will be considered down if the response status code is lower
                thna this value"
              persistent-hint
            />
          </v-col>
        </v-row>

        <v-row>
          <v-col cols="12" sm="6" xl="4">
            <v-text-field
              v-model="maxHTTPStatusCode"
              hide-details
              type="number"
              label="Maximum expected Status Code"
              hint="The monitor will be considered down if the response status code is greater
                thna this value"
              persistent-hint
            />
          </v-col>
        </v-row>

        <v-row>
          <v-col cols="12" sm="6" xl="4">
            <v-text-field
              label="Text body assertion (optional)"
              v-model="bodyTextToMatch"
              hint="The monitor will be considered down if the response body does not
                contain the expected string of characters"
              persistent-hint
            />
          </v-col>
        </v-row>

        <v-row>
          <v-col cols="12" sm="6" xl="4">
            <v-switch
              v-model="followHTTPRedirects"
              label="Follow redirects"
            />
          </v-col>
        </v-row>

        <v-row>
          <v-col cols="12" sm="6" xl="4">
            <v-switch
              v-model="showOnStatusPage"
              label="Show on status page"
            />
          </v-col>
        </v-row>


      </v-container>
    </v-row>

    <v-row v-else>
      <v-col cols="12">
        <h2>{{ monitor.name }}</h2>
        <h4 class="subtitle-1">{{ monitor.endpoint }} - {{ monitor.status }}</h4>

        <!--
          Uptime
          response time
          TLS certificate metrics

          status changes (error)
          pings

         -->
      </v-col>

      <!-- <v-col cols="12">
        <v-card flat>
          <v-card-text>
            <b-monitor-uptime-chart
              :uptime="monitor.uptime"
            />
          </v-card-text>
        </v-card>
      </v-col> -->

      <v-col cols="12">
        <v-card flat>
          <v-card-text>
            <b-monitor-response-time-chart
              :pings="monitor.pings"
            />
          </v-card-text>
        </v-card>
      </v-col>

      <v-col cols="12">
        <v-tabs v-model="tab">
          <v-tab>Uptime changes</v-tab>
          <v-tab>Pings</v-tab>
        </v-tabs>
        <v-tabs-items v-model="tab">
          <v-tab-item>
            <b-monitor-status-changes-table
              :changes="monitor.statusChanges"
            />
          </v-tab-item>
          <v-tab-item>
            <b-pings-table
              :pings="monitor.pings"
            />
          </v-tab-item>
        </v-tabs-items>
      </v-col>

    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';
import {
  Monitor, MonitorHttpMethod, MonitorType, CreateMonitorInput, DeleteMonitorInput,
  UpdateMonitorInput,
} from '@/api/graphql/model';
import BMonitorResponseTimeChart from './monitor_response_time_chart.vue';
import BMonitorUptimeChart from './monitor_uptime_chart.vue';
import BMonitorStatusChangesTable from './monitor_status_changes_table.vue';
import BPingsTable from './pings_table.vue';

export default VueApp.extend({
  name: 'BMonitors',
  components: {
    BMonitorResponseTimeChart,
    BMonitorUptimeChart,
    BPingsTable,
    BMonitorStatusChangesTable,
  },
  props: {
    monitor: {
      type: Object as PropType<Monitor | null>,
      required: false,
      default: null,
    },
  },
  data() {
    return {
      loading: false,
      error: '',
      updating: false,
      tab: 0,

      httpMethods: Object.values(MonitorHttpMethod),

      name: '',
      endpoint: '',
      httpMethod: MonitorHttpMethod.Get,
      bodyTextToMatch: '',
      minHTTPStatusCode: 0,
      maxHTTPStatusCode: 0,
      followHTTPRedirects: true,
      isActive: true,
      showOnStatusPage: false,
    };
  },
  computed: {
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
    editing(): boolean {
      return this.monitor === null || this.updating;
    },
    activeLabel(): string {
      return this.isActive ? 'Active' : 'Disabled';
    },
  },
  watch: {
    monitor() {
      this.clearFields();
    },
  },
  mounted() {
    this.clearFields();
  },
  methods: {
    cancel() {
      if (this.monitor) {
        this.updating = false;
        this.clearFields();
      } else {
        this.$router.push({ path: `/${this.projectFullPath}/-/monitors` });
      }
    },
    updateClicked() {
      this.updating = true;
    },
    clearFields() {
      if (this.monitor) {
        this.name = this.monitor.name;
        this.endpoint = this.monitor.endpoint;
        this.httpMethod = this.monitor.httpMethod;
        this.bodyTextToMatch = this.monitor.bodyTextToMatch;
        this.minHTTPStatusCode = this.monitor.minHTTPStatusCode;
        this.maxHTTPStatusCode = this.monitor.maxHTTPStatusCode;
        this.followHTTPRedirects = this.monitor.followHTTPRedirects;
        this.isActive = this.monitor.isActive;
        this.showOnStatusPage = this.monitor.showOnStatusPage;
        this.updating = false;
      } else {
        this.name = '';
        this.endpoint = '';
        this.httpMethod = MonitorHttpMethod.Get;
        this.bodyTextToMatch = '';
        this.minHTTPStatusCode = 200;
        this.maxHTTPStatusCode = 299;
        this.followHTTPRedirects = true;
        this.isActive = true;
        this.updating = false;
        this.showOnStatusPage = false;
      }
    },
    async create() {
      this.loading = true;
      this.error = '';
      const input: CreateMonitorInput = {
        projectFullPath: this.projectFullPath,
        name: this.name,
        endpoint: this.endpoint,
        type: MonitorType.Https,
        httpMethod: this.httpMethod,
        bodyTextToMatch: this.bodyTextToMatch,
        minHTTPStatusCode: this.minHTTPStatusCode,
        maxHTTPStatusCode: this.maxHTTPStatusCode,
        followHTTPRedirects: this.followHTTPRedirects,
        showOnStatusPage: this.showOnStatusPage,
      };

      try {
        await this.$operationsService.createMonitor(input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async update() {
      this.loading = true;
      this.error = '';
      const input: UpdateMonitorInput = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        monitorId: this.monitor!.id,
        name: this.name,
        endpoint: this.endpoint,
        httpMethod: this.httpMethod,
        bodyTextToMatch: this.bodyTextToMatch,
        minHTTPStatusCode: this.minHTTPStatusCode,
        maxHTTPStatusCode: this.maxHTTPStatusCode,
        followHTTPRedirects: this.followHTTPRedirects,
        isActive: this.isActive,
        showOnStatusPage: this.showOnStatusPage,
      };

      try {
        const updatedMonitor = await this.$operationsService.updateMonitor(input);
        this.$emit('updated', updatedMonitor);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async deleteMonitor() {
      // eslint-disable-next-line no-alert, no-restricted-globals
      if (!confirm('Do you really want to delete monitor?')) {
        return;
      }

      this.loading = true;
      this.error = '';
      const input: DeleteMonitorInput = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        monitorId: this.monitor!.id,
      };

      try {
        await this.$operationsService.deleteMonitor(this.projectFullPath, input);
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
