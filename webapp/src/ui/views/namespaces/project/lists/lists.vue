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
        <h2>Lists</h2>
        <p>
          Lists allow you to segment your outbound messages and give better control to your contacts
          to choose which messages they want to receive from you.
        </p>
      </v-col>

      <v-col cols="12" class="ma-0 py-0">
        <v-app-bar dense color="white" flat>
          <v-spacer />
          <v-btn :to="`/${projectFullPath}/-/lists/new`" color="success" depressed>
            <v-icon left>mdi-plus</v-icon>
            New List
          </v-btn>
        </v-app-bar>
      </v-col>

      <v-col cols="12" class="ma-0 py-0">
        <b-lists-list :lists="lists" :loading="loading" />
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { List, Project } from '@/api/graphql/model';
import { VueApp } from '@/app/vue';
import BListsList from '@/ui/components/growth/lists_list.vue';

export default VueApp.extend({
  name: 'BListsView',
  components: {
    BListsList,
  },
  data() {
    return {
      loading: false,
      error: '',
      project: null as Project | null,
    };
  },
  computed: {
    lists(): List[] {
      return this.project?.lists ?? [];
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
        this.project = await this.$growthService.fetchLists(this.projectFullPath);
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
