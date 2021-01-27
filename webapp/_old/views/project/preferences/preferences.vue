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
        <h2 class="text-h4">
          Project
          <span class="text-h5 text--secondary">({{ project.id }})</span>
        </h2>
      </v-col>
    </v-row>

    <v-row v-if="project">
      <v-col cols="12">
        <h2 class="text-h5">Public profile</h2>
      </v-col>
    </v-row>

    <v-row v-if="project" class="mx-5">
      <v-col cols="10" md="6" lg="5" xl="4">
        <b-avatar-form :loading="loading" @update-avatar="updateAvatar"
          :avatarUrl="project.avatarUrl" />
      </v-col>
    </v-row>

    <v-row class="mx-5">
      <v-col cols="10" md="6" lg="5" xl="4">
        <v-text-field v-model="name" label="Name" :loading="loading" />
      </v-col>
      <v-col cols="10" md="6" lg="5" xl="4">
        <v-text-field v-model="path" label="Path" :loading="loading" />
      </v-col>
      <v-col cols="10" md="6" lg="5" xl="4" >
        <v-textarea v-model="description" label="Description" :loading="loading" />
      </v-col>
      <v-col cols="10" md="6" class="d-none d-md-flex">
      </v-col>
      <v-col cols="10" md="6" lg="5" xl="4" class="flex-wrap">
        <v-text-field v-model="homepageUrl" label="Homepage URL" :loading="loading" />
      </v-col>
      <v-col cols="10" md="6" lg="5" xl="4">
        <v-text-field v-model="publicEmail" label="Public email" :loading="loading" />
      </v-col>
      <v-col cols="10" md="6" lg="5" xl="4">
        <v-text-field v-model="twitterUrl" label="Twitter page" :loading="loading" />
      </v-col>
      <v-col cols="10" md="6" lg="5" xl="4">
        <v-text-field v-model="facebookUrl" label="Facebook page" :loading="loading" />
      </v-col>
      <v-col cols="10" md="6" lg="5" xl="4">
        <v-text-field v-model="instagramUrl" label="Instagram page" :loading="loading" />
      </v-col>
      <v-col cols="10" md="6" lg="5" xl="4">
        <v-text-field v-model="whatsappNumber" label="WhatsApp number" :loading="loading" />
      </v-col>
      <v-col cols="10" md="6" lg="5" xl="4">
        <v-text-field v-model="mastodonUrl" label="Mastodon page" :loading="loading" />
      </v-col>
      <v-col cols="10" md="6" class="d-none d-md-flex">
      </v-col>


      <v-col cols="10" md="6" lg="5" xl="4">
        <v-btn @click="resetFields" text class="mr-5" :loading="loading">
          Cancel
        </v-btn>

        <v-btn @click="updateProject" color="success" depressed class="ml-5" :loading="loading">
          Save
        </v-btn>
      </v-col>
    </v-row>

    <v-row class="mt-5 pt-5">
      <v-col cols="12">
        <h2 class="text-h4 red--text">Danger zone</h2>
      </v-col>
    </v-row>
    <v-row>
      <v-col cols="12">
        <v-btn color="error" depressed @click="deleteProject" :loading="loading">
          <v-icon left>mdi-close-octagon</v-icon> Delete project
        </v-btn>
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
/* eslint-disable @typescript-eslint/no-non-null-assertion, max-len */
import { VueApp } from '@/app/vue';
import { Project, UpdateProjectInput } from '@/api/graphql/model';
import BAvatarForm from '@/ui/components/kernel/avatar_form.vue';


export default VueApp.extend({
  name: 'ProjectPreferencesView',
  components: {
    BAvatarForm,
  },
  data() {
    return {
      loading: false,
      error: '',
      project: null as Project | null,

      name: '',
      description: '',
      path: '',
      twitterUrl: '',
      facebookUrl: '',
      publicEmail: '',
      instagramUrl: '',
      whatsappNumber: '',
      mastodonUrl: '',
      homepageUrl: '',
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
  },
  created() {
    this.fetchData();
  },
  methods: {
    resetFields() {
      if (this.project) {
        this.name = this.project.name;
        this.description = this.project.description;
        this.path = this.project.path;
        this.twitterUrl = this.project.twitterUrl;
        this.facebookUrl = this.project.facebookUrl;
        this.publicEmail = this.project.publicEmail;
        this.instagramUrl = this.project.instagramUrl;
        this.whatsappNumber = this.project.whatsappNumber;
        this.mastodonUrl = this.project.mastodonUrl;
        this.homepageUrl = this.project.homepageUrl;
      } else {
        this.name = '';
        this.description = '';
        this.path = '';
        this.twitterUrl = '';
        this.facebookUrl = '';
        this.publicEmail = '';
        this.instagramUrl = '';
        this.whatsappNumber = '';
        this.mastodonUrl = '';
        this.homepageUrl = '';
      }
    },
    async fetchData() {
      this.loading = true;
      this.error = '';

      try {
        this.project = await this.$projectsService.fetchProjectPreferences(this.projectFullPath);
        this.resetFields();
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async updateProject(): Promise<void> {
      this.loading = true;
      this.error = '';
      const oldPath = this.project!.path;
      const name = this.project!.name === this.name ? null : this.name;
      const description = this.project!.description === this.description ? null : this.description;
      const path = this.project!.path === this.path ? null : this.path;
      const twitterUrl = this.project!.twitterUrl === this.twitterUrl ? null : this.twitterUrl;
      const facebookUrl = this.project!.facebookUrl === this.facebookUrl ? null : this.facebookUrl;
      const publicEmail = this.project!.publicEmail === this.publicEmail ? null : this.publicEmail;
      const instagramUrl = this.project!.instagramUrl === this.instagramUrl ? null : this.instagramUrl;
      const whatsappNumber = this.project!.whatsappNumber === this.whatsappNumber ? null : this.whatsappNumber;
      const mastodonUrl = this.project!.mastodonUrl === this.mastodonUrl ? null : this.mastodonUrl;
      const homepageUrl = this.project!.homepageUrl === this.homepageUrl ? null : this.homepageUrl;
      const input: UpdateProjectInput = {
        projectId: this.project!.id!,
        name,
        description,
        path,
        twitterUrl,
        facebookUrl,
        publicEmail,
        instagramUrl,
        whatsappNumber,
        mastodonUrl,
        homepageUrl,
      };

      try {
        this.project = await this.$projectsService.updateProject(input);
        if (oldPath !== this.project!.path) {
          this.$router.push({ path: `/${this.namespacePath}/${this.project!.path}/-/preferences` });
        } else {
          this.resetFields();
        }
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async updateAvatar(file: File): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        // eslint-disable-next-line max-len
        this.project!.avatarUrl = await this.$projectsService.updateProjectAvatar(this.project!.id!, file);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async deleteProject() {
      // eslint-disable-next-line no-alert, no-restricted-globals
      if (!confirm(`Do you really want to delete the ${this.name} project?`)) {
        return;
      }
      this.loading = true;
      this.error = '';

      try {
        await this.$projectsService.deleteProject(this.namespacePath, this.projectPath);
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
