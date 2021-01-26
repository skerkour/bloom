<template>
  <v-container fluid>
    <v-row justify="start">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12">
        <h2 class="text-h4">Group invitations</h2>
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12" md="10" lg="8">
        <b-invitations-list
          :invitations="invitations" :loading="loading" user
          @accept="acceptInvitation"  @decline="declineInvitation"
        />
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { VueApp } from '@/app/vue';
import BInvitationsList from '@/ui/components/groups/invitations_list.vue';
import { GroupInvitation } from '@/domain/kernel/model';


export default VueApp.extend({
  name: 'BPreferencesInvitationsView',
  components: {
    BInvitationsList,
  },
  data() {
    return {
      loading: false,
      error: '',
      invitations: [] as GroupInvitation[],
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
        this.invitations = await this.$kernelService.fetchMyGroupInvitations();
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async acceptInvitation(invitation: GroupInvitation) {
      this.loading = true;
      this.error = '';

      try {
        await this.$kernelService.acceptGroupInvitation(invitation.id);
        this.invitations = this.invitations.filter((invit: GroupInvitation) => {
          if (invit.id === invitation.id) {
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
    async declineInvitation(invitation: GroupInvitation) {
      this.loading = true;
      this.error = '';

      try {
        await this.$kernelService.declineGroupInvitation(invitation.id);
        this.invitations = this.invitations.filter((invit: GroupInvitation) => {
          if (invit.id === invitation.id) {
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
