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
          <v-btn :to="`/${projectFullPath}/-/contacts/new`" color="success" depressed>
            <v-icon left>mdi-plus</v-icon>
            New contact
          </v-btn>
        </v-app-bar>

        <b-contacts-list :contacts="contacts" :loading="loading" />

      </v-col>
    </v-row>


    <v-dialog
      v-model="showImportDialog"
      max-width="800px"
      scrollable
    >
      <v-card>
        <v-card-title class="headline">
          Import contacts
        </v-card-title>

        <v-card-text>
          <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
            {{ error }}
          </v-alert>
          <p class="text-body1 font-weight-medium text--primary">
            Import contacts in CSV format. <br />
            The CSV file can have either of the following shapes

<pre class="py-3">name,email
name,email</pre>
            or <br />
<pre class="py-3">email
email</pre>
          </p>
          <v-textarea
            :placeholder="importContactsLabel"
            v-model="contactsToImport"
            outlined
            class="my-2"
          />
        </v-card-text>

        <v-card-actions>
          <v-btn text @click="closeImportDialog" :loading="loading">Cancel</v-btn>
          <v-spacer />

          <v-btn @click="importContacts" color="success" :loading="loading" depressed>
            Import contacts
          </v-btn>

        </v-card-actions>
      </v-card>
    </v-dialog>

  </v-container>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';
import {
  Project,
  Contact, ImportContactsInput,
} from '@/api/graphql/model';
import BContactsList from '@/ui/components/support/contacts_list.vue';

export default VueApp.extend({
  name: 'ProjectContactsView',
  components: {
    BContactsList,
  },
  data() {
    return {
      loading: false,
      error: '',
      project: null as Project | null,
      showImportDialog: false,
      contactsToImport: '',
      importContactsLabel: `name,email
name,email`,
    };
  },
  computed: {
    contacts(): Contact[] {
      return this.project?.contacts ?? [];
    },
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
        this.project = await this.$growthService.fetchContacts(this.projectFullPath);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async importContacts(): Promise<void> {
      this.loading = true;
      this.error = '';
      const input: ImportContactsInput = {
        projectFullPath: this.projectFullPath,
        contacts: this.contactsToImport,
      };

      try {
        const contacts = await this.$growthService.importContacts(input);
        const contactsSet = new Set(contacts.map((c: Contact) => c.id));
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion, max-len
        const oldContacts = this.project!.contacts.filter((contact: Contact) => !contactsSet.has(contact.id));
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        this.project!.contacts = oldContacts.concat(contacts);
        this.closeImportDialog();
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    openImportDialog() {
      this.showImportDialog = true;
    },
    closeImportDialog() {
      this.contactsToImport = '';
      this.showImportDialog = false;
    },
  },
});
</script>

<style lang="scss" scoped>
</style>
