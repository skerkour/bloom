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


    <v-row justify="center" v-if="user">
      <b-user-homepage :user="user" />
    </v-row>

    <v-row justify="center" v-if="group">
      <v-col cols="12" md="8">
        <h1>Group: {{ group.name }} </h1>
        <v-btn color="success" @click="goToCreateProject" depressed>
          <v-icon left>mdi-plus</v-icon>
          New project
        </v-btn>
      </v-col>
      <v-col cols="12" md="8">
        <b-projects-list :projects="projects" :namespace-path="this.namespacePath" />
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { User, Group, Project } from '@/api/graphql/model';
// import BProjectsList from '@/ui/components/projects/projects_list.vue';
import BUserHomepage from '@/ui/components/kernel/user_homepage.vue';


export default VueApp.extend({
  name: 'BNamespaceView',
  components: {
    // BProjectsList,
    BUserHomepage,
  },
  data() {
    return {
      user: null as User | null,
      group: null as Group | null,
      projects: [] as Project[],
      error: '',
      loading: false,
    };
  },
  computed: {
    namespacePath(): string {
      return this.$route.params.namespacePath;
    },
  },
  watch: {
    namespacePath(to: string) {
      // this.$router.push({ path: `/${to}`, f });
      this.fetchData(to);
    },
  },
  mounted() {
    this.fetchData(this.$route.params.namespacePath);
  },
  destroyed() {
    this.$namespacesService.leaveNamespaceView();
  },
  methods: {
    goToCreateProject(): void {
      this.$router.push({ path: '/projects/new', query: { namespace: this.$route.params.namespacePath } });
    },
    async fetchData(path: string): Promise<void> {
      this.loading = true;
      this.error = '';
      this.user = null;
      this.group = null;

      try {
        const namespace = await this.$namespacesService.fetchNamespace(path);
        // eslint-disable-next-line no-underscore-dangle
        if (namespace.__typename === 'Group') {
          this.group = namespace;
        } else {
          this.user = namespace;
        }
        this.projects = namespace.projects;
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
