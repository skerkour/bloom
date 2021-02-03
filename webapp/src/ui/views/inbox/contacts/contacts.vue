<template>
  <v-container fluid>

    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center">
      <v-col cols="12" class="pb-0">
        <h2>Contacts</h2>
        <p>
          Contacts let you organize your users, subscribers and all the people you interact with
          in the context of your project.
        </p>
      </v-col>

      <v-col cols="12" class="pt-0">
        <v-app-bar dense color="white" flat>
          <v-spacer />
          <v-btn color="primary" depressed class="mr-3" @click="openImportDialog">
            <v-icon left>mdi-cloud-upload</v-icon>
            Import contacts
          </v-btn>
          <v-btn to="/inbox/contacts/new" color="success" depressed>
            <v-icon left>mdi-plus</v-icon>
            New contact
          </v-btn>
        </v-app-bar>

        <b-contacts-list :contacts="contacts" :loading="loading" />

      </v-col>
    </v-row>

    <b-import-contacts-dialog v-model="showImportDialog" @imported="onContactsImported" />

  </v-container>
</template>

<script lang="ts">
/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { VueApp } from '@/app/vue';
import {
  Contact,
} from '@/domain/inbox/model';
import BContactsList from '@/ui/components/inbox/contacts_list.vue';
import BImportContactsDialog from '@/ui/components/inbox/import_contacts_dialog.vue';

export default VueApp.extend({
  name: 'BContactsView',
  components: {
    BContactsList,
    BImportContactsDialog,
  },
  data() {
    return {
      loading: false,
      error: '',
      contacts: [] as Contact[],
      showImportDialog: false,
    };
  },
  computed: {
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
  },
  created() {
    this.fetchData();
  },
  methods: {
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.contacts = await this.$inboxService.fetchContacts();
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    onContactsImported(contacts: Contact[]) {
      const contactsSet = new Set(contacts.map((c: Contact) => c.id));
      const oldContacts = this.contacts.filter((contact) => !contactsSet.has(contact.id));
      this.contacts = oldContacts.concat(contacts);
    },
    openImportDialog() {
      this.showImportDialog = true;
    },
  },
});
</script>

<style lang="scss" scoped>
</style>
