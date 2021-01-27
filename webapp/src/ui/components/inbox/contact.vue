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
      <v-col class="d-flex">
        <v-btn @click="cancel" depressed :loading="loading" class="mr-auto">
          Back
        </v-btn>

        <v-btn @click="update" depressed color="success" :loading="loading"
          v-if="contact">
          Save changes
        </v-btn>
        <v-btn @click="create" depressed color="success" :loading="loading" v-else>
          Create contact
        </v-btn>

        <v-menu bottom v-if="contact">
          <template v-slot:activator="{ on }">
            <v-btn icon v-on="on">
              <v-icon>mdi-dots-vertical</v-icon>
            </v-btn>
          </template>

          <v-list>
            <v-list-item @click="deleteContact">
              <v-list-item-icon>
                <v-icon>mdi-delete</v-icon>
              </v-list-item-icon>
              <v-list-item-title>Delete contact</v-list-item-title>
            </v-list-item>
          </v-list>
        </v-menu>
      </v-col>
    </v-row>


    <v-row>
      <v-col cols="12" sm="6">
        <v-text-field v-model="name" label="Name" prepend-icon="mdi-account" />
      </v-col>
      <v-col cols="12" sm="6">
        <v-text-field v-model="email" label="Email" prepend-icon="mdi-at" />
      </v-col>

      <v-col cols="12" sm="6">
        <v-text-field v-model="phone" label="Phone" prepend-icon="mdi-phone" />
      </v-col>
      <!-- <v-col cols="12" sm="6">
        <v-select
          :items="countrySelectItems"
          label="Country"
          v-model="countryCode"
          prepend-icon="mdi-earth"
        />
      </v-col> -->

      <v-col cols="12" sm="6">
        <v-text-field v-model="address" label="Address" prepend-icon="mdi-home" />
      </v-col>

      <v-col cols="12" sm="6">
        <v-text-field v-model="website" label="Website" prepend-icon="mdi-web" />
      </v-col>
      <v-col cols="12" sm="6">
        <v-text-field v-model="twitter" label="Twitter" prepend-icon="mdi-twitter" />
      </v-col>

      <v-col cols="12" sm="6">
        <v-text-field v-model="instagram" label="Instagram" prepend-icon="mdi-instagram" />
      </v-col>
      <v-col cols="12" sm="6">
        <v-text-field v-model="facebook" label="Facebook" prepend-icon="mdi-facebook" />
      </v-col>

      <v-col cols="12" sm="6">
        <v-text-field v-model="linkedin" label="LinkedIn" prepend-icon="mdi-linkedin" />
      </v-col>
      <v-col cols="12" sm="6">
        <v-text-field v-model="skype" label="Skype" prepend-icon="mdi-skype" />
      </v-col>

      <v-col cols="12" sm="6">
        <v-text-field v-model="telegram" label="Telegram" prepend-icon="mdi-telegram" />
      </v-col>

      <v-col cols="12" sm="6">
        <v-text-field v-model="plan" label="Plan" prepend-icon="mdi-currency-usd" />
      </v-col>

      <v-col cols="12" sm="6">
        <v-text-field v-model="userId" label="User ID" prepend-icon="mdi-numeric" />
      </v-col>

      <v-col cols="12">
        <v-textarea v-model="notes" label="Notes" prepend-icon="mdi-note-text"  outlined/>
      </v-col>

    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';
import countries from '@/app/utils/countries';
import {
  CreateContact, DeleteContact, UpdateContact, Contact,
} from '@/domain/inbox/model';


export default VueApp.extend({
  name: 'BContact',
  props: {
    contact: {
      type: Object as PropType<Contact | null>,
      required: false,
      default: null,
    },
  },
  data() {
    return {
      name: '',
      email: '',
      pgpKey: '',
      phone: '',
      address: '',
      website: '',
      twitter: '',
      instagram: '',
      facebook: '',
      linkedin: '',
      skype: '',
      telegram: '',
      notes: '',
      countryCode: '',
      plan: '',
      userId: '',

      loading: false,
      error: '',
    };
  },
  computed: {
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    countrySelectItems(): any[] {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      return countries.map((country: any) => {
        country.text = country.name;
        country.value = country.code;
        return country;
      });
    },
  },
  mounted() {
    this.clearFields();
  },
  methods: {
    cancel() {
      this.$router.push({ path: '/inbox/contacts' });
    },
    clearFields() {
      if (this.contact) {
        this.name = this.contact.name;
        this.email = this.contact.email;
        this.pgpKey = this.contact.pgp_key;
        this.phone = this.contact.phone;
        this.address = this.contact.address;
        this.website = this.contact.website;
        this.twitter = this.contact.twitter;
        this.instagram = this.contact.instagram;
        this.facebook = this.contact.facebook;
        this.linkedin = this.contact.linkedin;
        this.skype = this.contact.skype;
        this.telegram = this.contact.telegram;
        this.notes = this.contact.notes;
        this.countryCode = ''; // TODO
        this.plan = this.contact.plan;
        this.userId = this.contact.user_id;
      } else {
        this.name = '';
        this.email = '';
        this.pgpKey = '';
        this.phone = '';
        this.address = '';
        this.website = '';
        this.twitter = '';
        this.instagram = '';
        this.facebook = '';
        this.linkedin = '';
        this.skype = '';
        this.telegram = '';
        this.notes = '';
        this.countryCode = '';
        this.plan = '';
        this.userId = '';
      }
    },
    async create() {
      this.loading = true;
      this.error = '';
      const input: CreateContact = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        namespace_id: this.$store.state.currentNamespace!.id!,
        name: this.name,
        email: this.email,
        pgp_key: this.pgpKey,
        phone: this.phone,
        address: this.address,
        website: this.website,
        twitter: this.twitter,
        instagram: this.instagram,
        facebook: this.facebook,
        linkedin: this.linkedin,
        skype: this.skype,
        telegram: this.telegram,
        notes: this.notes,
        plan: this.plan,
        user_id: this.userId,
        birthday: null, // TODO
        bloom: '', // TODO
      };

      try {
        await this.$inboxService.createContact(input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async update() {
      this.loading = true;
      this.error = '';
      const input: UpdateContact = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        contact_id: this.contact!.id,
        name: this.name,
        email: this.email,
        pgp_key: this.pgpKey,
        phone: this.phone,
        address: this.address,
        website: this.website,
        twitter: this.twitter,
        instagram: this.instagram,
        facebook: this.facebook,
        linkedin: this.linkedin,
        skype: this.skype,
        telegram: this.telegram,
        notes: this.notes,
        plan: this.plan,
        user_id: this.userId,
        birthday: null,
        bloom: '', // TODO
      };

      try {
        const contact = await this.$inboxService.updateContact(input);
        this.$emit('updated', contact);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async deleteContact() {
      // eslint-disable-next-line no-alert, no-restricted-globals
      if (!confirm('Do you really want to delete contact?')) {
        return;
      }

      this.loading = true;
      this.error = '';
      const input: DeleteContact = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        contact_id: this.contact!.id,
      };

      try {
        await this.$inboxService.deleteContact(input);
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
