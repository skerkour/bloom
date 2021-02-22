<template>
  <v-container fluid>
    <v-row>
      <v-col cols="12">
        <h2 class="text-h4">Public profile</h2>
      </v-col>
    </v-row>

    <v-row justify="start">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="info !== ''">
        <v-alert type="info" :value="info !== ''" border="top" colored-border elevation="1">
          {{ info }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row v-if="user" class="mx-5">
      <v-col cols="10" md="6" lg="5" xl="4">
        <b-avatar-form :loading="loading" @update-avatar="updateAvatar"
          :avatarUrl="user.avatar_url" />
      </v-col>
    </v-row>

    <v-row class="d-flex align-start flex-column mx-5">
      <v-col cols="10" md="6" lg="5" xl="4">
        <v-text-field v-model="name" label="Name" :loading="loading" />
      </v-col>
      <v-col cols="10" md="6" lg="5" xl="4">
        <v-text-field v-model="username" label="Username" :loading="loading" />
      </v-col>
      <v-col cols="10" md="6" lg="5" xl="4">
        <v-text-field v-model="email" label="Email" :loading="loading" />
      </v-col>
      <v-col cols="10" md="6" lg="5" xl="4">
        <v-textarea v-model="bio" label="Bio" :loading="loading" />
      </v-col>

      <v-col cols="10" md="6" lg="5" xl="4">
        <v-btn @click="resetFields" text class="mr-5" :loading="loading">
          Cancel
        </v-btn>

        <v-btn @click="updateProfile" color="success" depressed class="ml-5" :loading="loading">
          Save
        </v-btn>
      </v-col>
    </v-row>


    <v-row class="mt-5 pt-5">
      <v-col cols="12">
        <h2 class="text-h4">Security</h2>
      </v-col>
    </v-row>
    <v-row>
      <v-col cols="12">
        <v-btn color="error" depressed :loading="loading" @click="openDisableTwoFaDialog"
          v-if="twoFAEnabled">
          <v-icon left>mdi-shield-off</v-icon>
          Disable 2FA
        </v-btn>
        <v-btn color="success" depressed :loading="loading" @click="setupTwoFA" v-else>
          <v-icon left>mdi-shield-check</v-icon>
          Enable 2FA
        </v-btn>
      </v-col>
    </v-row>


    <v-row class="mt-5 pt-5">
      <v-col cols="12">
        <h2 class="text-h4 red--text">Danger zone</h2>
      </v-col>
    </v-row>
    <v-row>
      <v-col cols="12">
        <v-btn color="error" depressed @click="askToDeleteMyAccount" :loading="loading">
          <v-icon left>mdi-close-octagon</v-icon>
          Delete my account
        </v-btn>
      </v-col>
    </v-row>

    <b-two-fa-setup-dialog
      :qrcode="twoFaQrCodeBase64Image"
      :error="error"
      :loading="loading"
      v-model="showTwoFaSetupDialog"
      @enable="enableTwoFa"
      @disable="disableTwoFa"
    />

    <b-delete-account-two-fa-dialog
      :error="error"
      :loading="loading"
      v-model="showDeleteAccountTwoFaDialog"
      @delete-account="deleteMyAccount"
    />
  </v-container>
</template>


<script lang="ts">
/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { VueApp } from '@/app/vue';
import BAvatarForm from '@/ui/components/kernel/avatar_form.vue';
import BTwoFaSetupDialog from '@/ui/components/kernel/two_fa_setup_dialog.vue';
import BDeleteAccountTwoFaDialog from '@/ui/components/kernel/delete_account_two_fa_dialog.vue';
import { UpdateMyProfile, User } from '@/domain/kernel/model';

export default VueApp.extend({
  name: 'BProfileView',
  components: {
    BAvatarForm,
    BTwoFaSetupDialog,
    BDeleteAccountTwoFaDialog,
  },
  data() {
    return {
      user: null as User | null,
      username: '',
      name: '',
      bio: '',
      email: '',
      loading: false,
      error: '',
      info: '',
      twoFaQrCodeBase64Image: '',
      showTwoFaSetupDialog: false,
      showDeleteAccountTwoFaDialog: false,
    };
  },
  computed: {
    twoFAEnabled(): boolean {
      return this.user?.two_fa_enabled ?? false;
    },
  },
  mounted() {
    this.fetchData();
  },
  methods: {
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        const me = await this.$kernelService.fetchMe();
        this.user = me.user;
        this.resetFields();
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async updateProfile(): Promise<void> {
      this.loading = true;
      this.error = '';
      const username = this.user!.username === this.username ? null : this.username;
      const email = this.user!.email === this.email ? null : this.email;
      const name = this.user!.name === this.name ? null : this.name;
      const description = this.user!.description === this.bio ? null : this.bio;
      const input: UpdateMyProfile = {
        username,
        email,
        name,
        description,
      };

      try {
        this.user = await this.$kernelService.updateMyProfile(input);
        if (email) {
          this.info = 'Please click on the link in the email we just sent you to complete your email update.';
        }
        this.resetFields();
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async updateAvatar(file: File): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.user = await this.$kernelService.updateMyAvatar(file);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async setupTwoFA(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.twoFaQrCodeBase64Image = await this.$kernelService.setupTwoFa();
        this.showTwoFaSetupDialog = true;
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async enableTwoFa(code: string): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        await this.$kernelService.completeTwoFaSetup(code);
        this.showTwoFaSetupDialog = false;
        this.twoFaQrCodeBase64Image = '';
        this.user!.two_fa_enabled = true;
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    openDisableTwoFaDialog() {
      this.showTwoFaSetupDialog = true;
    },
    async disableTwoFa(code: string): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        await this.$kernelService.disableTwoFa(code);
        this.showTwoFaSetupDialog = false;
        this.user!.two_fa_enabled = false;
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    resetFields() {
      if (this.user) {
        this.email = this.user.email!;
        this.username = this.user.username;
        this.bio = this.user.description;
        this.name = this.user.name;
      } else {
        this.email = '';
        this.username = '';
        this.bio = '';
      }
    },
    askToDeleteMyAccount() {
      // eslint-disable-next-line no-alert, no-restricted-globals
      if (!confirm('Do you really want to delete account? All the data will be irrecoverable!')) {
        return;
      }

      // eslint-disable-next-line no-alert, no-restricted-globals
      if (!confirm('By deleteing my account I understand that I will irrevocably lose all my data')) {
        return;
      }

      if (this.user!.two_fa_enabled) {
        this.showDeleteAccountTwoFaDialog = true;
      } else {
        this.deleteMyAccount(null);
      }
    },
    async deleteMyAccount(code: string | null): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        await this.$kernelService.deleteMyAccount(code);
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
