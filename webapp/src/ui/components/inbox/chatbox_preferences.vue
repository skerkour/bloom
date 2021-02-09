<template>
  <v-container fluid>

    <v-row v-if="error !== '' || success">
      <v-col cols="12" class="text-center">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
          {{ error }}
        </v-alert>
        <v-alert type="success" :value="success" dismissible text>
          Success
        </v-alert>
      </v-col>
    </v-row>

    <v-row>
      <v-color-picker
        v-model="color"
        hide-mode-switch
        mode="hexa"
      />
    </v-row>

    <v-row>
      <v-col cols="12" sm="6" xl="4">
        <v-text-field
          label="Chatbox Name"
          v-model="name"
        />
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12" sm="6" xl="4">
        <v-textarea
          v-model="welcomeMessage"
          label="Welcome message"
        />
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="10" md="6" lg="5" xl="4" class="flex-wrap">
        <v-text-field v-model="websiteUrl" label="Website URL" :loading="loading" />
      </v-col>

      <v-col cols="10" md="6" lg="5" xl="4">
        <v-text-field v-model="twitter" label="Twitter username" :loading="loading" />
      </v-col>

      <v-col cols="10" md="6" lg="5" xl="4">
        <v-text-field v-model="facebookUrl" label="Facebook page" :loading="loading" />
      </v-col>

      <v-col cols="10" md="6" lg="5" xl="4">
        <v-text-field v-model="instagram" label="Instagram username" :loading="loading" />
      </v-col>

      <v-col cols="10" md="6" lg="5" xl="4">
        <v-text-field v-model="whatsappNumber" label="WhatsApp number" :loading="loading" />
      </v-col>

      <v-col cols="10" md="6" lg="5" xl="4">
        <v-text-field v-model="mastodonUrl" label="Mastodon page" :loading="loading" />
      </v-col>

    </v-row>

    <v-row>
      <v-col cols="12" sm="6" xl="4">
        <v-switch
          v-model="branding"
          label="Branding"
        />
      </v-col>
    </v-row>

    <v-row>
      <v-btn text @click="cancel" :loading="loading" class="mr-5">
        Cancel
      </v-btn>


      <v-btn color="success" @click="updateChatboxPreferences" :loading="loading" depressed
        class="ml-5">
        Save
      </v-btn>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { ChatboxPreferences, UpdateChatboxPreferences } from '@/domain/inbox/model';
import { PropType } from 'vue';

export default VueApp.extend({
  name: 'BChatboxPreferences',
  props: {
    preferences: {
      required: true,
      type: Object as PropType<ChatboxPreferences>,
    },
  },
  data() {
    return {
      loading: false,
      error: '',
      success: false,

      color: '',
      name: '',
      branding: true,
      welcomeMessage: '',
      twitter: '',
      facebookUrl: '',
      instagram: '',
      whatsappNumber: '',
      mastodonUrl: '',
      websiteUrl: '',
      telegram: '',
    };
  },
  created() {
    this.cancel();
  },
  methods: {
    async updateChatboxPreferences() {
      this.loading = true;
      this.error = '';
      this.success = false;

      const input: UpdateChatboxPreferences = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        namespace_id: this.$store.state.currentNamespace!.id!,
        color: this.color,
        name: this.name,
        show_branding: this.branding,
        welcome_message: this.welcomeMessage,
        twitter: this.twitter,
        facebook_url: this.facebookUrl,
        instagram: this.instagram,
        whatsapp_number: this.whatsappNumber,
        mastodon_url: this.mastodonUrl,
        website_url: this.websiteUrl,
        telegram: this.telegram,
      };

      try {
        const preferences = await this.$inboxService.updateChatboxPreferences(input);
        this.success = true;
        this.$emit('updated', preferences);
        setTimeout(() => {
          this.success = false;
        }, 2000);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    cancel() {
      this.color = this.preferences.color;
      this.name = this.preferences.name;
      this.branding = this.preferences.show_branding;
      this.welcomeMessage = this.preferences.welcome_message;

      this.twitter = this.preferences.twitter;
      this.facebookUrl = this.preferences.facebook_url;
      this.instagram = this.preferences.instagram;
      this.whatsappNumber = this.preferences.whatsapp_number;
      this.mastodonUrl = this.preferences.mastodon_url;
      this.websiteUrl = this.preferences.website_url;
      this.telegram = this.preferences.telegram;
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
