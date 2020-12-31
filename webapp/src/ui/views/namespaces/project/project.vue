<template>
  <v-container fluid>

    <v-row>
      <v-col cols="12" class="text-center" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
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

    <v-row v-if="project">
      <v-col cols="12">
        <h2 class="text-h3">
          {{ project.name }}
        </h2>
        <h3 class="text-h6">
          {{ projectFullPath }}
        </h3>
      </v-col>
    </v-row>

    <v-row v-if="project" justify="center">
      <v-col cols="12" md="4" v-for="card in cards" :key="card.title">
        <router-link :to="card.link">
          <v-card outlined class="mx-auto blm-project-card d-flex flex-column align-stretch">
            <v-card-title>
              <p class="display-1 text--primary">
                <v-icon>{{ card.icon }}</v-icon>
                {{ card.title }}
              </p>
            </v-card-title>
            <v-card-text class="text-body-1">
              {{ card.description }}
            </v-card-text>
          </v-card>
        </router-link>
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Project } from '@/api/graphql/model';
import { VueApp } from '@/app/vue';


export default VueApp.extend({
  name: 'MProjectView',
  data() {
    return {
      loading: false,
      error: '',
      project: null as Project | null,
    };
  },
  computed: {
    projectPath(): string {
      return this.$route.params.projectPath;
    },
    namespacePath(): string {
      return this.$route.params.namespacePath;
    },
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    cards(): any[] {
      return [
        {
          title: 'Bots',
          icon: 'mdi-robot',
          description: 'Create automated workflows and let machines work for you 24 / 7 with bots.',
          link: `/${this.projectFullPath}/-/bots`,
        },
        {
          title: 'Files',
          icon: 'mdi-folder-outline',
          description: 'Share files and collaborate with a private cloud Drive.',
          link: `/${this.projectFullPath}/-/files`,
        },
        {
          title: 'Tickets',
          icon: 'mdi-cards-outline',
          description: 'Tickets allow you to collaboratively develop ideas, solve problems, and plan work.',
          link: `/${this.projectFullPath}/-/tickets`,
        },
        {
          title: 'Milestones',
          icon: 'mdi-sign-direction',
          description: `Milestones are containers for tickets. They allow you to organize
            work into a cohesive group, with an optional period of time.`,
          link: `/${this.projectFullPath}/-/milestones`,
        },
        {
          title: 'Inbox',
          icon: 'mdi-inbox-arrow-down',
          description: 'A livechat widget to support your users and visitors faster than the light.',
          link: `/${this.projectFullPath}/-/inbox`,
        },
        {
          title: 'Contacts',
          icon: 'mdi-contacts-outline',
          description: `Contacts let you organize your users, subscribers and all the people you interact with
          in the context of your project.`,
          link: `/${this.projectFullPath}/-/contacts`,
        },
        {
          title: 'Analytics',
          icon: 'mdi-chart-line',
          description: 'Analytics lets you get insights and better understand your customers and visitors',
          link: `/${this.projectFullPath}/-/analytics`,
        },
        {
          title: 'Outbound messages',
          icon: 'mdi-send-check-outline',
          description: `Outbound messages (also known as campaigns) are messages that you can send and automate to
            reach and engage with your contacts.`,
          link: `/${this.projectFullPath}/-/outbound`,
        },
        {
          title: 'Lists',
          icon: 'mdi-format-list-bulleted',
          description: `Lists allow you to segment your outbound messages and give better control to your contacts
          to choose which messages they want to receive from you.`,
          link: `/${this.projectFullPath}/-/lists`,
        },
        {
          title: 'Monitors',
          icon: 'mdi-heart-pulse',
          description: 'Monitor your Websites and APIs and get notified when something go wrong.',
          link: `/${this.projectFullPath}/-/monitors`,
        },
        {
          title: 'Labels',
          icon: 'mdi-label-multiple-outline',
          description: 'Labels help you organize, filter, search and manage your different resources.',
          link: `/${this.projectFullPath}/-/labels`,
        },
        {
          title: 'Preferences',
          icon: 'mdi-cog',
          description: 'Update or delete your project.',
          link: `/${this.projectFullPath}/-/preferences`,
        },
      ];
    },
  },
  created() {
    this.fetchData();
  },
  methods: {
    async fetchData() {
      this.loading = true;
      this.error = '';

      try {
        this.project = await this.$projectsService.fetchProjectOverview(this.projectFullPath);
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
.blm-project-card {
  height: 100%;
}
</style>
