<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>


    <v-row>
      <v-col cols="12" md="10" xl="8">
        <b-admin-users-list :users="users" :loading="loading" />
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { User } from '@/domain/kernel/model';
import { VueApp } from '@/app/vue';
import BAdminUsersList from '@/ui/components/kernel/admin_users_list.vue';

export default VueApp.extend({
  name: 'BAdminUsersView',
  components: {
    BAdminUsersList,
  },
  data() {
    return {
      loading: false,
      error: '',
      users: [] as User[],
    };
  },
  mounted() {
    this.fetchData();
  },
  methods: {
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.users = await this.$kernelService.adminFetchUsers();
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
