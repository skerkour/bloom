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

    <v-row v-if="group" class="text-body-1">
      <v-col cols="12">
        <b>ID</b>: {{ group.id }}
      </v-col>

      <v-col cols="12">
        <b>Name</b>: {{ group.name }}
      </v-col>

      <v-col cols="12">
        <b>Path</b>: {{ group.path }}
      </v-col>

      <v-col cols="12">
        <b>Created at</b>: {{ date(group.createdAt) }}
      </v-col>

      <v-col cols="12">
        <b>Description</b>: {{ group.description }}
      </v-col>

     <!-- <v-col cols="12">
        <h3 class="text-h5">Billing </h3>
        <p v-if="group.billing">
          <b>Plan</b>: {{ group.billing.plan }}
        </p>
        <p v-else>
          <b>Plan</b>: free
        </p>
        <b-customer :value="group.billing" :loading="loading" readonly
          v-if="group.billing" />
      </v-col>

      <v-col cols="12">
        <h3 class="text-h5">Members </h3>
        <b-group-members-list :members="members" :loading="loading" />
      </v-col> -->

    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Group } from '@/domain/kernel/model';
import { VueApp } from '@/app/vue';
import date from '@/app/filters/date';
import BGroupMembersList from '@/ui/components/groups/members_list.vue';
import BCustomer from '@/ui/components/kernel/customer.vue';

export default VueApp.extend({
  name: 'BAdminGroupView',
  components: {
    BGroupMembersList,
    BCustomer,
  },
  data() {
    return {
      loading: false,
      error: '',
      group: null as Group | null,
    };
  },
  computed: {
    groupId(): string {
      return this.$route.params.groupId;
    },
  },
  mounted() {
    this.fetchData();
  },
  methods: {
    date,
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.group = await this.$kernelService.adminFetchGroup(this.groupId);
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
