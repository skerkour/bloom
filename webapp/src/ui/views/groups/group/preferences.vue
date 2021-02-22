<template>
  <v-container fluid>
    <v-row justify="start">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>


    <v-row class="mb-5 pb-5">
      <v-col cols="12">
        <h2 class="text-h4">Group profile</h2>
      </v-col>
    </v-row>

    <v-row v-if="group" class="mx-5">
      <v-col cols="10" md="6" lg="5" xl="4">
        <b-avatar-form :loading="loading" @update-avatar="updateAvatar"
          :avatarUrl="group.avatar_url" />
      </v-col>
    </v-row>

    <v-row class="d-flex align-start flex-column mx-5">
      <v-col cols="10" md="6" lg="5" xl="4">
        <v-text-field v-model="name" label="Name" :loading="loading" />
      </v-col>
      <v-col cols="10" md="6" lg="5" xl="4">
        <v-text-field v-model="path" label="Path" :loading="loading" />
      </v-col>
      <v-col cols="10" md="6" lg="5" xl="4">
        <v-textarea v-model="description" label="Description" :loading="loading" />
      </v-col>

      <v-col cols="10" md="6" lg="5" xl="4">
        <v-btn @click="resetFields" text class="mr-5" :loading="loading">
          Cancel
        </v-btn>

        <v-btn @click="updateProfile" color="success" depressed class="ml-5" :loading="loading">
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
        <v-btn color="error" depressed @click="deleteGroup" :loading="loading">
          <v-icon left>mdi-close-octagon</v-icon> Delete group
        </v-btn>
      </v-col>
    </v-row>


  </v-container>
</template>


<script lang="ts">
/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { VueApp } from '@/app/vue';
import { Group, UpdateGroupProfile } from '@/domain/kernel/model';
import BAvatarForm from '@/ui/components/kernel/avatar_form.vue';


export default VueApp.extend({
  name: 'BGroupPreferencesView',
  components: {
    BAvatarForm,
  },
  data() {
    return {
      group: null as Group | null,
      name: '',
      description: '',
      path: '',

      loading: false,
      error: '',
    };
  },
  computed: {
    groupPath(): string {
      return this.$route.params.groupPath;
    },
  },
  mounted() {
    this.fetchData();
  },
  methods: {
    resetFields() {
      if (this.group) {
        this.name = this.group.name;
        this.description = this.group.description;
        this.path = this.group.path;
      } else {
        this.name = '';
        this.description = '';
        this.path = '';
      }
    },
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.group = await this.$kernelService.fetchGroup(this.groupPath);
        this.resetFields();
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async updateProfile(): Promise<void> {
      this.loading = true;
      this.error = '';
      const oldPath = this.group!.path;
      const name = this.group!.name === this.name ? null : this.name;
      const description = this.group!.description === this.description ? null : this.description;
      const path = this.group!.path === this.path ? null : this.path;
      const input: UpdateGroupProfile = {
        group_id: this.group!.id!,
        name,
        description,
        path,
      };

      try {
        this.group = await this.$kernelService.updateGroupProfile(input);
        if (oldPath !== this.group.path) {
          this.$router.push({ path: `/groups/${this.group.path}/preferences` });
        } else {
          this.resetFields();
        }
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async deleteGroup() {
      // eslint-disable-next-line no-alert, no-restricted-globals
      if (!confirm(`Do you really want to delete the ${this.groupPath} group?`)) {
        return;
      }
      this.loading = true;
      this.error = '';

      try {
        await this.$kernelService.deleteGroup(this.group!.id!, this.group!.path);
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
        this.group = await this.$kernelService.updateGroupAvatar(this.group!.id!, file);
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
