<template>
  <v-data-table
    :headers="headers"
    :items="users"
    item-key="id"
    :loading="loading"
    :items-per-page="50"
  >
    <template v-slot:no-data>
      <p class="text-center">
        No user.
      </p>
    </template>

    <template v-slot:item="{ item }" class="text-left">
      <tr @click="gotoUser(item)" class="bloom-pointer">
        <td>
          {{ item.id }}
        </td>
        <td>
          {{ item.name }}
        </td>
        <td>
          {{ item.username }}
        </td>
        <td>
          <v-chip v-if="item.isAdmin" color="success">
            Admin
          </v-chip>
          <v-chip v-else>
            User
          </v-chip>
        </td>
        <td>
          {{ item.email }}
        </td>
      </tr>
    </template>
  </v-data-table>
</template>


<script lang="ts">
import { PropType } from 'vue';
import { VueApp } from '@/app/vue';
import { User } from '@/api/graphql/model';
import date from '@/app/filters/date';

export default VueApp.extend({
  name: 'BAdminUsersList',
  props: {
    users: {
      type: Array as PropType<User[]>,
      default: [],
    },
    loading: {
      type: Boolean as PropType<boolean>,
      default: false,
      required: true,
    },
  },
  data() {
    return {
      headers: [
        {
          text: 'ID',
          align: 'start',
          sortable: false,
          value: 'id',
        },
        {
          text: 'Name',
          align: 'start',
          sortable: true,
          value: 'name',
        },
        {
          text: 'Username',
          align: 'start',
          sortable: true,
          value: 'username',
        },
        {
          text: 'Admin',
          align: 'start',
          sortable: true,
          value: 'isAdmin',
        },
        {
          text: 'Email',
          align: 'start',
          sortable: true,
          value: 'username',
        },
      ],
    };
  },
  methods: {
    date,
    gotoUser(user: User) {
      this.$router.push({ path: `/admin/users/${user.username}` });
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
