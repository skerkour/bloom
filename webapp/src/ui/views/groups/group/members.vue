<template>
  <v-container fluid>
    <v-row justify="start">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row >
      <v-col cols="12">
        <h2 class="text-h4">Members</h2>
      </v-col>
    </v-row>

    <v-row class="mb-5 pb-5">
      <v-col cols="12" md="10" lg="8" class="text-left">
        <v-btn color="primary" depressed @click="openInviteDialog" :loading="loading">
          <v-icon left>mdi-plus</v-icon>
          Invite people
        </v-btn>

      </v-col>
      <v-col cols="12" md="10" lg="8">
        <b-members-list
        :members="members" :loading="loading" :currentUserUsername="currentUserUsername"
        @remove="removeMember" @quit="quitGroup" />
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12">
        <h2 class="text-h4">Invitations</h2>
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12" md="10" lg="8">
        <b-invitations-list
          :invitations="invitations" :loading="loading" group
          @cancel="cancelInvitation"
        />
      </v-col>
    </v-row>

    <v-dialog
      v-model="showInviteDialog"
      max-width="800px"
      scrollable
    >
      <v-card>
        <v-card-title class="headline">
          Invite people
        </v-card-title>

        <v-card-text>
          <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
            {{ error }}
          </v-alert>

          <v-combobox
            v-model="usernamesToInvite"
            chips
            clearable
            label="Usernames..."
            multiple
            prepend-icon="mdi-account-group"
          >
            <template v-slot:selection="{ attrs, item, select, selected }">
              <v-chip
                v-bind="attrs"
                :input-value="selected"
                close
                @click="select"
                @click:close="removeUsernameToinvite(item)"
              >
                <strong>{{ item }}</strong>&nbsp;
              </v-chip>
            </template>
          </v-combobox>
        </v-card-text>

        <v-card-actions>
          <v-btn text @click="canceInvite" :loading="loading">Cancel</v-btn>
          <v-spacer />

          <v-btn @click="invitePeople" color="success" :loading="loading" depressed>
            <v-icon left>mdi-send</v-icon>
            Invite
          </v-btn>

        </v-card-actions>
      </v-card>
    </v-dialog>

  </v-container>
</template>


<script lang="ts">
/* eslint-disable @typescript-eslint/no-non-null-assertion */
import {
  CancelGroupInvitationInput,
  Group, GroupInvitation, GroupMember, InvitePeopleInGroupInput, QuitGroupInput,
  RemoveMemberFromGroupInput,
} from '@/api/graphql/model';
import { VueApp } from '@/app/vue';
import BMembersList from '@/ui/components/groups/members_list.vue';
import BInvitationsList from '@/ui/components/groups/invitations_list.vue';

export default VueApp.extend({
  name: 'BGroupMembersView',
  components: {
    BMembersList,
    BInvitationsList,
  },
  data() {
    return {
      group: null as Group | null,
      showInviteDialog: false,
      usernamesToInvite: [] as string[],
      loading: false,
      error: '',
    };
  },
  computed: {
    currentUserUsername(): string {
      return this.$store.state.me!.username!;
    },
    groupPath(): string {
      return this.$route.params.groupPath;
    },
    members(): GroupMember[] {
      return this.group?.members ?? [];
    },
    invitations(): GroupInvitation[] {
      return this.group?.invitations ?? [];
    },
  },
  mounted() {
    this.fetchData();
  },
  methods: {
    openInviteDialog() {
      this.showInviteDialog = true;
    },
    removeUsernameToinvite(username: string) {
      this.usernamesToInvite.splice(this.usernamesToInvite.indexOf(username), 1);
      this.usernamesToInvite = [...this.usernamesToInvite];
    },
    canceInvite() {
      this.showInviteDialog = false;
      this.usernamesToInvite = [];
    },
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.group = await this.$groupsService.fetchGroupMembers(this.groupPath);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async removeMember(member: GroupMember) {
      this.loading = true;
      this.error = '';
      const input: RemoveMemberFromGroupInput = {
        groupId: this.group!.id!,
        username: member.username,
      };

      try {
        await this.$groupsService.removeMemberFromGroup(input);
        this.group!.members = this.group!.members.filter((groupMember: GroupMember) => {
          if (groupMember.username === member.username) {
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
    async cancelInvitation(invitation: GroupInvitation) {
      this.loading = true;
      this.error = '';
      const input: CancelGroupInvitationInput = {
        invitationId: invitation.id,
      };

      try {
        await this.$groupsService.cancelInvitation(input);
        this.group!.invitations = this.group!.invitations.filter((invit: GroupInvitation) => {
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
    async invitePeople() {
      if (this.usernamesToInvite.length === 0) {
        this.canceInvite();
        return;
      }

      this.loading = true;
      this.error = '';
      const input: InvitePeopleInGroupInput = {
        groupId: this.group!.id!,
        usernames: this.usernamesToInvite,
      };

      try {
        this.group = await this.$groupsService.invitePeopleInGroup(input);
        this.canceInvite();
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async quitGroup() {
      this.loading = true;
      this.error = '';
      const input: QuitGroupInput = {
        groupId: this.group!.id!,
      };

      try {
        await this.$groupsService.quitGroup(input);
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
