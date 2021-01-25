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
        namespace_id: this.$store.state.currentNamespaceId!,
        color: this.color,
        name: this.name,
        show_branding: this.branding,
        welcome_message: this.welcomeMessage,
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
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
