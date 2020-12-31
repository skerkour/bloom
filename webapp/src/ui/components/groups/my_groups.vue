<template>
  <v-container fluid>

    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center" v-if="loading" >
      <v-col cols="12" sm="10" md="8" xl="6">
        <v-progress-circular
          :size="50"
          color="primary"
          indeterminate
        />
      </v-col>
    </v-row>

    <v-row justify="center" v-else>
      <v-col cols="12" md="8" lg="6" xl="4" >
        <h4 class="text-h4">Groups</h4>
        <p class="py-1">
          With Groups you can organize related projects together and grant many people access
          to several projects at once.
        </p>
        <div class="pb-4">
          <v-btn to="/groups/new" color="success" depressed>
            <v-icon left>mdi-plus</v-icon>
            New group
          </v-btn>
        </div>

        <b-groups-list :groups="groups" v-if="groups.length !== 0"/>
        <div v-else>
          <p>No group yet</p>
        </div>
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { Group } from '@/api/graphql/model';
import BGroupsList from './groups_list.vue';

export default VueApp.extend({
  name: 'BMyGroups',
  components: {
    BGroupsList,
  },
  data() {
    return {
      error: '',
      loading: false,
      groups: [] as Group[],
    };
  },
  created() {
    this.fetchGroups();
  },
  methods: {
    async fetchGroups(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.groups = await this.$usersService.fetchMyGroupsWithProjects();
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
