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
      <v-btn @click="cancel" depressed :loading="loading" class="mr-auto ml-3">
        Cancel
      </v-btn>

      <v-btn class="mr-3"
        @click="deleteMessage"
        depressed
        color="error"
        :loading="loading"
        v-if="message">
        Delete message
      </v-btn>
      <v-btn class="mr-3"
        @click="sendTestMessage"
        outlined
        color="primary"
        :loading="loading"
        v-if="message">
        Send test message
      </v-btn>
      <v-btn class="mr-3"
        @click="sendMessage"
        depressed
        color="primary"
        :loading="loading"
        v-if="message">
        Send message
      </v-btn>
      <v-btn
        @click="updateMessage" depressed color="success" :loading="loading" :disabled="!canCreate"
        v-if="message" class="mr-3">
        Save changes
      </v-btn>
      <v-btn
        @click="createMessage" depressed color="success" :loading="loading" :disabled="!canCreate"
        v-else >
        Create message
      </v-btn>
    </v-row>


    <v-row>
      <v-col cols="12" sm="6">
        <v-text-field v-model="name" label="Name" />
      </v-col>

      <v-col cols="12">
        <v-text-field v-model="subject" label="Subject" />
      </v-col>

      <v-col cols="12">
        <b-select-lists v-model="selectedList" :items="lists" />
      </v-col>

      <v-col cols="12">
        <b-markdown-editor placeholder="Compose your message..." v-model="body" outlined />
      </v-col>

    </v-row>

  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';
import BMarkdownEditor from '@/ui/components/kernel/markdown_editor.vue';
import BSelectLists from '@/ui/components/newsletter/select_lists.vue';
import {
  CreateMessage, List, Message, UpdateMessage,
} from '@/domain/newsletter/model';


export default VueApp.extend({
  name: 'BNewsletterMessage',
  components: {
    BMarkdownEditor,
    BSelectLists,
  },
  props: {
    message: {
      type: Object as PropType<Message | null>,
      required: false,
      default: null,
    },
    list: {
      type: Object as PropType<List>,
      required: false,
      default: null,
    },
    lists: {
      type: Array as PropType<List[]>,
      required: true,
    },
  },
  data() {
    return {
      loading: false,
      error: '',

      name: '',
      subject: '',
      body: '',
      bodyHtml: '',
      selectedList: null as List | null,
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
      this.$router.push({ path: '/newsletter/messages' });
    },
    clearFields() {
      if (this.message) {
        this.name = this.message.name;
        this.subject = this.message.subject;
        this.body = this.message.body;
        this.bodyHtml = this.message.body_html;
      } else {
        this.name = '';
        this.subject = '';
        this.body = '';
        this.bodyHtml = '';
        [this.selectedList] = this.lists;
      }
    },
    async createMessage() {
      this.loading = true;
      this.error = '';
      const input: CreateMessage = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        list_id: this.selectedList!.id,
        name: this.name,
        subject: this.subject,
        body: this.body,
      };

      try {
        await this.$newsletterService.createMessage(input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async updateMessage() {
      this.loading = true;
      this.error = '';
      const input: UpdateMessage = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        message_id: this.message!.id,
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        list_id: this.selectedList!.id,
        name: this.name,
        subject: this.subject,
        body: this.body,
      };

      try {
        const message = await this.$newsletterService.updateMessage(input);
        this.$emit('updated', message);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async deleteMessage() {
      // eslint-disable-next-line no-alert, no-restricted-globals
      if (!confirm('Do you really want to delete message?')) {
        return;
      }

      this.loading = true;
      this.error = '';
      try {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        await this.$newsletterService.deleteMessage(this.message!.id);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async sendMessage() {
      // eslint-disable-next-line no-alert, no-restricted-globals
      if (!confirm('Do you really want to send message message?')) {
        return;
      }

      this.loading = true;
      this.error = '';

      try {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        await this.$newsletterService.sendMessage(this.message!.id);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async sendTestMessage() {
      this.loading = true;
      this.error = '';

      try {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        await this.$newsletterService.sendTestMessage(this.message!.id);
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
