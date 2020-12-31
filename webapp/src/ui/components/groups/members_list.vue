<template>
  <v-data-table
    :headers="headers"
    :items="members"
    item-key="id"
    hide-default-footer
    :loading="loading"
  >
    <template v-slot:no-data>
      <p class="text-center">
        No member.
      </p>
    </template>

    <template v-slot:item="{ item }" class="text-left">
      <tr>
        <td>
          {{ item.name }} ({{ item.username }})
        </td>
        <td>
          {{ item.role }}
        </td>
        <td>
          <v-btn depressed color="error" @click="quitGroup(item)"
            v-if="item.username === currentUserUsername">
            Quit
          </v-btn>
          <v-btn outlined color="error" @click="removeMember(item)" v-else>
            Remove
          </v-btn>
        </td>
      </tr>
    </template>
  </v-data-table>
</template>


<script lang="ts">
import { GroupMember } from '@/api/graphql/model';
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';

export default VueApp.extend({
  name: 'BGroupMembersList',
  props: {
    members: {
      type: Array as PropType<GroupMember[]>,
      default: [],
    },
    loading: {
      type: Boolean as PropType<boolean>,
      default: false,
    },
    currentUserUsername: {
      type: String as PropType<string>,
      default: '',
    },
  },
  data() {
    return {
      headers: [
        {
          text: 'User',
          align: 'start',
          sortable: true,
          value: 'username',
        },
        {
          text: 'Role',
          align: 'start',
          sortable: true,
          value: 'role',
        },
        {
          text: 'Actions',
          align: 'start',
          sortable: false,
        },
      ],
    };
  },
  methods: {
    removeMember(member: GroupMember) {
      this.$emit('remove', member);
    },
    quitGroup(member: GroupMember) {
      this.$emit('quit', member);
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
