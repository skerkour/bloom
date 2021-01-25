<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center" class="text-center" v-if="loading">
      <v-col cols="12" sm="10" md="8" xl="6">
        <v-progress-circular
          :size="50"
          color="primary"
          indeterminate
        />
      </v-col>
    </v-row>

    <v-row v-if="contact">
      <b-contact :contact="contact" @updated="onContactUpdated" />
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import BContact from '@/ui/components/inbox/contact.vue';
import { GetContact, Contact } from '@/domain/inbox/model';


export default VueApp.extend({
  name: 'ProjectContactView',
  components: {
    BContact,
  },
  data() {
    return {
      loading: false,
      error: '',
      contact: null as Contact | null,
    };
  },
  created() {
    this.fetchData();
  },
  methods: {
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';
      const input: GetContact = {
        contact_id: this.$route.params.contactId,
      };

      try {
        this.contact = await this.$inboxService.fetchContact(input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    onContactUpdated(contact: Contact) {
      this.contact = contact;
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
