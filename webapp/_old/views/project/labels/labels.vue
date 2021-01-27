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
        <h2>Labels</h2>
        <p>
          Labels help you organize, filter, search and manage your different resources.
        </p>
      </v-col>

      <v-col cols="12" class="ma-0 py-0">
        <v-app-bar dense color="white" flat>
          <v-spacer />
          <v-btn :to="`/${projectFullPath}/-/labels/new`" color="success" depressed>
            <v-icon left>mdi-plus</v-icon>
            New label
          </v-btn>
        </v-app-bar>
      </v-col>

      <v-col cols="12" class="ma-0 py-0">
        <labels-list
          :projectFullPath="projectFullPath" :labels="labels" @delete="deleteLabel"
          :loading="loading" />
      </v-col>
    </v-row>

  </v-container>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';
import { Project, Label, DeleteLabelInput } from '@/api/graphql/model';
import LabelsList from '@/ui/components/collaboration/labels_list.vue';

export default VueApp.extend({
  name: 'ProjectLabelsView',
  components: {
    LabelsList,
  },
  data() {
    return {
      loading: false,
      error: '',
      project: null as Project | null,
    };
  },
  computed: {
    labels(): Label[] {
      return this.project?.labels ?? [];
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
        this.project = await this.$collaborationService.fetchLabels(this.projectFullPath);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async deleteLabel(label: Label): Promise<void> {
      // eslint-disable-next-line no-alert, no-restricted-globals
      if (!confirm(`Do you really want to delete label ${label.name}?`)) {
        return;
      }

      this.loading = true;
      this.error = '';
      const input: DeleteLabelInput = {
        labelId: label.id,
      };

      try {
        await this.$collaborationService.deleteLabel(input);
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        this.project!.labels = this.project!.labels.filter((projectLabel: Label) => {
          if (label.id === projectLabel.id) {
            return false;
          }
          return true;
        });
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
