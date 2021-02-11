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

        <v-menu bottom v-if="list">
          <template v-slot:activator="{ on }">
            <v-btn icon v-on="on">
              <v-icon>mdi-dots-vertical</v-icon>
            </v-btn>
          </template>

          <v-list>
            <v-list-item @click="openImportDialog">
              <v-list-item-icon>
                <v-icon>mdi-cloud-upload</v-icon>
              </v-list-item-icon>
              <v-list-item-title>Import contact</v-list-item-title>
            </v-list-item>

            <v-list-item @click="deleteList">
              <v-list-item-icon>
                <v-icon>mdi-delete</v-icon>
              </v-list-item-icon>
              <v-list-item-title>Delete list</v-list-item-title>
            </v-list-item>
          </v-list>
        </v-menu>

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
          :items="contacts"
          item-key="id"
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
              <td>


                <v-menu bottom left>
                  <template v-slot:activator="{ on }">
                    <v-btn icon v-on="on">
                      <v-icon>mdi-dots-vertical</v-icon>
                    </v-btn>
                  </template>

                  <v-list>
                    <v-list-item @click="removeContact(item)">
                      <v-list-item-icon>
                        <v-icon>mdi-delete</v-icon>
                      </v-list-item-icon>
                      <v-list-item-title>Remove contact</v-list-item-title>
                    </v-list-item>
                  </v-list>
                </v-menu>
              </td>
            </tr>
          </template>
        </v-data-table>
      </v-col>
    </v-row>

    <b-import-contacts-dialog
      v-if="list"
      v-model="showImportDialog"
      @imported="onImported"
      :list="list.id"
    />

  </v-container>
</template>


<script lang="ts">
/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { PropType } from 'vue';
import { VueApp } from '@/app/vue';
import {
  List, Contact, CreateList, UpdateList, RemoveContactFromList,
} from '@/domain/newsletter/model';
import BImportContactsDialog from '@/ui/components/inbox/import_contacts_dialog.vue';

export default VueApp.extend({
  name: 'BNewsletterList',
  components: {
    BImportContactsDialog,
  },
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
      showImportDialog: false,

      name: '',
      description: '',
      contactsHeaders: [
        {
          text: 'Name',
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
        {
          text: 'Actions',
          align: 'start',
          sortable: false,
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
        namespace_id: this.$store.state.currentNamespace!.id!,
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
    openImportDialog() {
      this.showImportDialog = true;
    },
    async onImported() {
      this.$emit('imported');
    },
    async removeContact(contact: Contact) {
      // eslint-disable-next-line no-alert, no-restricted-globals
      if (!confirm(`Do you really want to remove ${contact.email} from ${this.list!.name}?`)) {
        return;
      }

      this.loading = true;
      this.error = '';
      const input: RemoveContactFromList = {
        contact_id: contact.id,
        list_id: this.list!.id,
      };

      try {
        await this.$newsletterService.removeContactFromList(input);
        this.$emit('removed', contact);
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
