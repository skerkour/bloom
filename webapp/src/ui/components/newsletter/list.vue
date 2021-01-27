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
      <v-col>
        <v-btn @click="cancel" depressed :loading="loading">
          Cancel
        </v-btn>
      </v-col>

      <v-col class="text-right">
        <v-btn class="mr-3"
          @click="deleteList"
          depressed
          color="error"
          :loading="loading"
          v-if="list">
          Delete list
        </v-btn>
        <v-btn
          @click="updateList" depressed color="success" :loading="loading" :disabled="!canCreate"
          v-if="list" >
          Save changes
        </v-btn>
        <v-btn
          @click="createList" depressed color="success" :loading="loading" :disabled="!canCreate"
          v-else >
          Create list
        </v-btn>
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12" sm="8">
        <v-text-field v-model="name" label="Name" />
      </v-col>

      <v-col cols="12" sm="8">
        <v-textarea v-model="description" label="Description" outlined/>
      </v-col>
    </v-row>

    <v-row v-if="list">
      <v-col cols="12">
        <v-data-table
          :headers="contactsHeaders"
          :items="list.contacts"
          item-key="id"
          hide-default-footer
          :loading="loading"
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
            </tr>
          </template>
        </v-data-table>
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { PropType } from 'vue';
import { VueApp } from '@/app/vue';
import {
  List, Contact, CreateList, UpdateList,
} from '@/domain/newsletter/model';

export default VueApp.extend({
  name: 'BList',
  props: {
    list: {
      type: Object as PropType<List | null>,
      required: false,
      default: null,
    },
    contacts: {
      type: Array as PropType<Contact[]>,
      required: false,
      default: [],
    },
  },
  data() {
    return {
      loading: false,
      error: '',

      name: '',
      description: '',
      contactsHeaders: [
        {
          text: 'Contact',
          align: 'start',
          sortable: true,
          value: 'name',
        },
        {
          text: 'Contact',
          align: 'start',
          sortable: true,
          value: 'email',
        },
      ],
    };
  },
  computed: {
    canCreate(): boolean {
      return this.name.length !== 0;
    },
  },
  mounted() {
    this.clearFields();
  },
  methods: {
    cancel() {
      this.$router.push({ path: '/newsletter/lists' });
    },
    clearFields() {
      if (this.list) {
        this.name = this.list.name;
        this.description = this.list.description;
      } else {
        this.name = '';
        this.description = '';
      }
    },
    async createList() {
      this.loading = true;
      this.error = '';
      const input: CreateList = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        namespace_id: this.$store.state.currentNamespaceId!,
        name: this.name,
        description: this.description,
      };

      try {
        await this.$newsletterService.createList(input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async updateList() {
      this.loading = true;
      this.error = '';
      const input: UpdateList = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        list_id: this.list!.id,
        name: this.name,
        description: this.description,
      };

      try {
        const list = await this.$newsletterService.updateList(input);
        this.$emit('updated', list);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async deleteList() {
      // eslint-disable-next-line no-alert, no-restricted-globals
      if (!confirm('Do you really want to delete list?')) {
        return;
      }

      this.loading = true;
      this.error = '';

      try {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        await this.$newsletterService.deleteList(this.list!.id);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    gotoContact(contact: Contact) {
      this.$router.push({ path: `/inbox/contacts/${contact.id}` });
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
