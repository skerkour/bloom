<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center" v-if="selectedApp === ''">
      <v-col cols="12" sm="10" md="8" xl="6">
        <v-text-field
          v-model="query" outlined label="Search app"
          prepend-inner-icon="mdi-magnify"/>
      </v-col>
    </v-row>

    <v-row justify="center" align="center" align-content="center" v-if="selectedApp === ''">
      <v-col cols="12" sm="10" md="8" xl="6" v-if="loading">
        <v-progress-circular
          :size="50"
          color="primary"
          indeterminate
        />
      </v-col>

      <v-col cols="3" v-for="(app, index) in filteredApps" :key="index" align-self="center"
        class="text-center" >
        <v-avatar class="blm-pointer" size="52" @click="selectApp(app.id)">
          <img :src="app.avatarUrl" :alt="app.name" />
        </v-avatar>
        <p class="text-subtitle-1 blm-pointer">{{ app.name }}</p>
      </v-col>
    </v-row>

    <v-row v-else>
      <v-col cols="12" sm="10" md="8" xl="6">
        <b-ssh-connection v-if="selectedApp === 'ssh'" />
        <b-telegram-connection v-if="selectedApp === 'telegram'" />
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { BotApp } from '@/api/graphql/model';
import { VueApp } from '@/app/vue';
import BSshConnection from '@/ui/components/bots/apps/ssh.vue';
import BTelegramConnection from '@/ui/components/bots/apps/telegram.vue';


export default VueApp.extend({
  name: 'BNewConnectionView',
  components: {
    BSshConnection,
    BTelegramConnection,
  },
  data() {
    return {
      error: '',
      loading: false,
      query: '',
      apps: [] as BotApp[],
      selectedApp: '',
    };
  },
  computed: {
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
    filteredApps(): BotApp[] {
      if (!this.query) {
        return this.apps;
      }
      return this.apps.filter((app: BotApp) => app.name.toLowerCase().includes(this.query));
    },
  },
  created() {
    this.fetchData();
  },
  methods: {
    selectApp(appId: string) {
      this.selectedApp = appId;
    },
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        const apps = await this.$botsService.fetchAllApps();
        this.apps = apps.filter((app: BotApp) => app.id !== 'bloom');
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
