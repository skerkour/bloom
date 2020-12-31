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
        <b-history :loading="loading" :history="history" />
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { BotExecution, Project } from '@/api/graphql/model';
import { VueApp } from '@/app/vue';
import BHistory from '@/ui/components/bots/history.vue';

export default VueApp.extend({
  name: 'BBotsHistoryView',
  components: {
    BHistory,
  },
  data() {
    return {
      loading: false,
      error: '',
      project: null as Project | null,
    };
  },
  computed: {
    history(): BotExecution[] {
      return this.project?.botsHistory ?? [];
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
