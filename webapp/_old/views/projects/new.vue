<template>
  <v-container fluid>

    <v-row justify="center">
       <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>


    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4">
        <v-card elevation="0">
          <v-card-title>
            <p class="display-1 text--primary">
              New Project
            </p>
          </v-card-title>

          <v-card-text>
            <v-row v-if="showNewProjectInfoAlert">
              <v-alert
                border="top"
                colored-border
                type="info"
                elevation="1"
              >
                You need to create a group before creating a project.
              </v-alert>
            </v-row>

            <v-row>
              <v-col cols="12">
                <v-text-field
                  label="Project name" v-model="name" :disabled="loading"
                  @input="nameToNamespace"
                />
              </v-col>
              <v-col cols="6" sm="4">
                <v-text-field
                  prefix="https://bloom.sh"
                  disabled
                />
              </v-col>
              <v-col cols="6" sm="4">
                <v-select
                  :items="namespaces"
                  label="Namespace"
                  v-model="namespacePath"
                />
              </v-col>
              <v-col cols="12" sm="4">
                <v-text-field
                  label="Project url"
                  v-model="path"
                  @keyup="lowercasePath"
                  :disabled="loading"
                />
              </v-col>
              <v-col cols="12">
                <v-textarea
                  label="Description"
                  v-model="description"
                  :disabled="loading"
                ></v-textarea>
              </v-col>
            </v-row>
          </v-card-text>

          <v-card-actions>
            <v-btn text @click="cancel">
              Cancel
            </v-btn>

            <v-spacer />

            <v-btn color="success" @click="createProject" :loading="loading" depressed>
              Create project
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { Group, CreateProjectInput } from '@/api/graphql/model';

export default VueApp.extend({
  name: 'MNewProjectView',
  data() {
    return {
      loading: false,
      loaded: false,
      name: '',
      error: '',
      path: '',
      namespacePath: '',
      description: '',
      namespaces: [] as string[],
    };
  },
  computed: {
    showNewProjectInfoAlert(): boolean {
      return this.loaded && this.namespaces.length === 0;
    },
  },
  created() {
    if (this.$route.query.namespace) {
      this.namespacePath = this.$route.query.namespace as string;
    }
    this.fetchData();
  },
  methods: {
    cancel(): void {
      this.$router.back();
    },
    lowercasePath(): void {
      this.path = this.path.toLowerCase();
    },
    nameToNamespace() {
      this.path = this.name.toLowerCase()
        .replaceAll(' ', '-')
        .replaceAll('.', '-')
        .replaceAll('_', '-');
    },
    async fetchData() {
      this.loading = true;
      this.error = '';

      try {
        const groups = await this.$usersService.fetchMyGroups();
        groups.forEach((group: Group) => {
          this.namespaces.push(group.path);
        });
        if (this.namespaces.length !== 0 && this.namespacePath === '') {
          [this.namespacePath] = this.namespaces;
        }
        this.loaded = true;
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async createProject() {
      this.loading = true;
      this.error = '';
      const input: CreateProjectInput = {
        namespacePath: this.namespacePath,
        name: this.name,
        path: this.path,
        description: this.description,
      };

      try {
        await this.$projectsService.createProject(input);
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
