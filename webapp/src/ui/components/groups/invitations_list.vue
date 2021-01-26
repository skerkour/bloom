<template>
  <v-data-table
    :headers="headers"
    :items="invitations"
    item-key="id"
    hide-default-footer
    :loading="loading"
  >
    <template v-slot:no-data>
      <p class="text-center">
        No invitation.
      </p>
    </template>

    <template v-slot:item="{ item }" class="text-left" >
      <tr v-if="group">
        <td>
          {{ item.invitee.name }} ({{ item.invitee.username }})
        </td>
        <td>
          <v-btn depressed color="error" @click="cancelInvitation(item)">
            Cancel
          </v-btn>
        </td>
      </tr>
      <tr v-if="user">
        <td>
          {{ item.group.name }}
        </td>
        <td>
          {{ item.inviter.name }} ({{ item.inviter.username }})
        </td>
        <td>
          <v-btn depressed color="success" @click="acceptInvitation(item)" class="mr-5">
            Accept
          </v-btn>
          <v-btn depressed color="error" @click="declineInvitation(item)">
            Decline
          </v-btn>
        </td>
      </tr>
    </template>
  </v-data-table>
</template>


<script lang="ts">
/* eslint-disable no-else-return */
import { GroupInvitation } from '@/domain/kernel/model';
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';

export default VueApp.extend({
  name: 'BGroupInvitationsList',
  props: {
    invitations: {
      type: Array as PropType<GroupInvitation[]>,
      default: [],
    },
    loading: {
      type: Boolean as PropType<boolean>,
      default: false,
    },
    group: {
      type: Boolean as PropType<boolean>,
      default: false,
    },
    user: {
      type: Boolean as PropType<boolean>,
      default: false,
    },
  },
  data() {
    if (this.group) {
      return {
        headers: [
          {
            text: 'Invitee',
            align: 'start',
            sortable: true,
            value: 'invitee.username',
          },
          {
            text: 'Actions',
            align: 'start',
            sortable: false,
          },
        ],
      };
    } else if (this.user) {
      return {
        headers: [
          {
            text: 'Group',
            align: 'start',
            sortable: true,
            value: 'group.name',
          },
          {
            text: 'Invited By',
            align: 'start',
            sortable: true,
            value: 'inviter.username',
          },
          {
            text: 'Actions',
            align: 'start',
            sortable: false,
          },
        ],
      };
    } else {
      return {};
    }
  },
  methods: {
    cancelInvitation(invitation: GroupInvitation) {
      this.$emit('cancel', invitation);
    },
    acceptInvitation(invitation: GroupInvitation) {
      this.$emit('accept', invitation);
    },
    declineInvitation(invitation: GroupInvitation) {
      this.$emit('decline', invitation);
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
