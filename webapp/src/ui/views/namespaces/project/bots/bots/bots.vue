<template>
  <v-container fluid>

    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center">
      <v-col cols="12">
        <h2>Bots</h2>
        <p>
          Bots are automations that connect apps and work for you 24 / 7.
        </p>
      </v-col>

      <v-col cols="12" class="ma-0 py-0">
        <v-app-bar dense color="white" flat>
          <v-spacer />
          <v-btn :to="`/${projectFullPath}/-/bots/new`" color="success" depressed>
            <v-icon left>mdi-plus</v-icon>
            New Bot
          </v-btn>
        </v-app-bar>
      </v-col>

      <v-col cols="12" class="ma-0 py-0">
        <b-bots-list :loading="loading" :bots="bots" />
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { Bot, Project } from '@/api/graphql/model';
import { VueApp } from '@/app/vue';
import BBotsList from '@/ui/components/bots/bots_list.vue';

export default VueApp.extend({
  name: 'BBotsView',
  components: {
    BBotsList,
  },
  data() {
    return {
      loading: false,
      error: '',
      project: null as Project | null,
    };
  },
  computed: {
    bots(): Bot[] {
      return this.project?.bots ?? [];
    },
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
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
        this.project = await this.$botsService.fetchBotsForProject(this.projectFullPath);
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
