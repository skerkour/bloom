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

    <v-row justify="center" v-if="statusPage">
      <v-col cols="12" md="8" xl="6" class="d-flex">
        <v-avatar>
          <v-img
            :src="statusPage.avatarUrl"
            alt="Avatar"
          />
        </v-avatar>
        &nbsp;
        <h2 class="text-h4 d-inline pl-2 align-self-center">{{ statusPage.name }}</h2>

        <v-tooltip bottom v-if="statusPage.homepageUrl">
          <template v-slot:activator="{ on, attrs }">
            <a target="_blank" rel="noopener" :href="statusPage.homepageUrl"
              v-bind="attrs" v-on="on" class="align-self-center pl-3">
              <v-icon>mdi-web</v-icon>
            </a>
          </template>
          <span>Website</span>
        </v-tooltip>

        <v-tooltip bottom v-if="statusPage.publicEmail">
          <template v-slot:activator="{ on, attrs }">
            <a target="_blank" rel="noopener" :href="`mailto:${statusPage.publicEmail}`"
              v-bind="attrs" v-on="on" class="align-self-center pl-3">
              <v-icon>mdi-at</v-icon>
            </a>
          </template>
          <span>Email</span>
        </v-tooltip>

        <v-tooltip bottom v-if="statusPage.twitterUrl">
          <template v-slot:activator="{ on, attrs }">
            <a target="_blank" rel="noopener" :href="statusPage.twitterUrl"
              v-bind="attrs" v-on="on" class="align-self-center pl-3">
              <v-icon>mdi-twitter</v-icon>
            </a>
          </template>
          <span>Twitter</span>
        </v-tooltip>

        <v-tooltip bottom v-if="statusPage.whatsappNumber">
          <template v-slot:activator="{ on, attrs }">
            <a target="_blank" rel="noopener" :href="whatsappUrl"
              v-bind="attrs" v-on="on" class="align-self-center pl-3">
              <v-icon>mdi-whatsapp</v-icon>
            </a>
          </template>
          <span>WhatsApp</span>
        </v-tooltip>

      </v-col>

    </v-row>

    <v-row justify="center" v-if="statusPage" class="mt-5 d-flex flex-colum">
      <v-col cols="12" md="8">
        <h2 class="text-h5">Services</h2>
      </v-col>

      <v-col cols="12" md="8" class="pt-0">
        <v-row justify="left">
          <v-col cols="12" md="8" xl="6">
            <v-list>
              <v-list-item v-for="monitor in statusPage.monitors" :key="monitor.name">
                <v-list-item-content>
                  <v-list-item-title v-html="monitor.name"></v-list-item-title>
                </v-list-item-content>

                  <v-list-item-action>
                    <v-tooltip bottom>
                      <template v-slot:activator="{ on, attrs }">
              <v-icon color="success" v-if="isOperational(monitor.status)" v-bind="attrs" v-on="on">
                          mdi-check-circle
                          </v-icon>
                          <v-icon v-else-if="isUnknown(monitor.status)" v-bind="attrs" v-on="on">
                            mdi-help-circle
                          </v-icon>
                          <v-icon color="error" v-else v-bind="attrs" v-on="on">
                            mdi-alert-circle
                          </v-icon>
                      </template>
                      <span>{{ monitor.status }}</span>
                    </v-tooltip>
                  </v-list-item-action>
              </v-list-item>
            </v-list>
          </v-col>
        </v-row>

      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { MonitorStatus, StatusPage } from '@/api/graphql/model';
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BStatusPageView',
  data() {
    return {
      loading: false,
      error: '',
      statusPage: null as StatusPage | null,
    };
  },
  computed: {
    projectFullPath(): string {
      return this.$route.params.projectFullPath;
    },
    whatsappUrl(): string {
      return `https://api.whatsapp.com/send?phone=${this.statusPage?.whatsappNumber}`;
    },
  },
  mounted() {
    this.fetchData();
  },
  methods: {
    isOperational(status: string): boolean {
      return status === MonitorStatus.Operational;
    },
    isUnknown(status: string): boolean {
      return status === MonitorStatus.Unknown;
    },
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.statusPage = await this.$kernelService.fetchStatusPage(this.projectFullPath);
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
