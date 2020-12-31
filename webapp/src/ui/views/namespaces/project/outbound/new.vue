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

    <v-row v-if="project">
      <b-outbound-message :projectLists="lists" />
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { List, Project } from '@/api/graphql/model';
import BOutboundMessage from '@/ui/components/growth/outbound_message.vue';

export default VueApp.extend({
  name: 'BNewOutboundMessageView',
  components: {
    BOutboundMessage,
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
      return this.project ? this.project.lists : [];
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
        const res = await this.$growthService.fetchLists(this.projectFullPath);
        this.project = res;
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
