<template>
  <v-container fluid>

    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
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

    <v-row v-if="user" class="text-body-1">
      <v-col cols="12">
        <b>ID</b>: {{ user.id }}
      </v-col>

      <v-col cols="12">
        <b>Username</b>: <router-link :to="`/${user.username}`">{{ user.username }}</router-link>
      </v-col>

      <v-col cols="12">
        <b>Name</b>: {{ user.name }}
      </v-col>

      <v-col cols="12">
        <b>Created at</b>: {{ date(user.created_at) }}
      </v-col>

      <v-col cols="12">
        <b>Bio</b>: {{ user.description }}
      </v-col>

      <v-col cols="12">
        <b>Admin</b>:
        <v-chip v-if="user.is_admin" color="success">
          Admin
        </v-chip>
        <v-chip v-else>
          User
        </v-chip>
      </v-col>

      <v-col cols="12">
        <v-btn color="primary" @click="unblockUser" depressed v-if="user.blocked_at">
          Unblock
        </v-btn>
        <v-btn color="error" @click="blockUser" depressed v-else>
          Block
        </v-btn>
      </v-col>

      <!-- <v-col cols="12">
        <b-admin-groups-list :groups="groups" :loading="loading" />
      </v-col> -->
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { User } from '@/domain/kernel/model';
import { VueApp } from '@/app/vue';
import date from '@/app/filters/date';
import BAdminGroupsList from '@/ui/components/kernel/admin_groups_list.vue';

export default VueApp.extend({
  name: 'BAdminUserView',
  components: {
    BAdminGroupsList,
  },
  data() {
    return {
      loading: false,
      error: '',
      user: null as User | null,
    };
  },
  computed: {
    userId(): string {
      return this.$route.params.userId;
    },
    // groups(): Group[] {
    //   return this.user?.groups ?? [];
    // },
  },
  mounted() {
    this.fetchData();
  },
  methods: {
    date,
    async unblockUser(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.user = await this.$kernelService.adminUnblockUser(this.userId);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async blockUser(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.user = await this.$kernelService.adminBlockUser(this.userId);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.user = await this.$kernelService.adminFetchUser(this.userId);
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
