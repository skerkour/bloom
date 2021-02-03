<template>
  <v-dialog
    v-model="show"
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
        <v-btn text @click="close" :loading="loading">Cancel</v-btn>
        <v-spacer />

        <v-btn @click="importContacts" color="success" :loading="loading" depressed>
          Import contacts
        </v-btn>

      </v-card-actions>
    </v-card>
  </v-dialog>
</template>


<script lang="ts">
/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { VueApp } from '@/app/vue';
import { ImportContacts } from '@/domain/inbox/model';
import { PropType } from 'vue';

export default VueApp.extend({
  name: 'BImportContactsDialog',
  props: {
    value: {
      type: Boolean as PropType<boolean>,
      required: true,
    },
    list: {
      type: String as PropType<string | null>,
      required: false,
      default: null,
    },
  },
  data() {
    return {
      loading: false,
      error: '',
      contactsToImport: '',
      importContactsLabel: `name,email
name,email`,
    };
  },
  computed: {
    show: {
      get(): boolean {
        return this.value;
      },
      set(value: boolean) {
        this.$emit('input', value);
      },
    },
  },
  methods: {
    close() {
      this.show = false;
    },
    async importContacts(): Promise<void> {
      this.loading = true;
      this.error = '';
      const input: ImportContacts = {
        namespace_id: this.$store.state.currentNamespace!.id!,
        contacts_csv: this.contactsToImport,
        list_id: this.list,
      };

      try {
        const contacts = await this.$inboxService.importContacts(input);
        this.close();
        this.$emit('imported', contacts);
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
