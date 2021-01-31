<template>
  <v-data-table
    :headers="headers"
    :items="contacts"
    item-key="id"
    :loading="loading"
    :items-per-page="50"
  >
    <template v-slot:no-data>
      <p class="text-center">
        No contact.
      </p>
    </template>

    <template v-slot:item="{ item }" class="text-left">
      <tr @click="gotoContact(item)" class="bloom-pointer">
        <td>
          {{ item.name }}
        </td>
        <td>
          {{ item.email }}
        </td>
        <td>
          {{ calendar(item.created_at) }}
        </td>
      </tr>
    </template>
  </v-data-table>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';
import { Contact } from '@/domain/inbox/model';
import { calendar } from '@/app/filters';

export default VueApp.extend({
  name: 'BContactsList',
  props: {
    contacts: {
      type: Array as PropType<Contact[]>,
      required: true,
    },
    loading: {
      type: Boolean as PropType<boolean>,
      required: false,
      default: false,
    },
  },
  data() {
    return {
      selected: [],
      headers: [
        {
          text: 'Name',
          align: 'start',
          sortable: true,
          value: 'name',
          width: '40%',
        },
        {
          text: 'Email',
          align: 'start',
          sortable: true,
          value: 'email',
          width: '40%',
        },
        {
          text: 'Created at',
          align: 'start',
          sortable: true,
          value: 'created_at',
        },
      ],
    };
  },
  methods: {
    calendar,
    gotoContact(contact: Contact) {
      this.$router.push({ path: `/inbox/contacts/${contact.id}` });
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
