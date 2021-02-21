<template>
  <v-container fluid class="pt-5">
    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row>
      <v-btn @click="cancel" depressed :loading="loading" class="mr-auto ml-3">
        Back
      </v-btn>

      <v-btn class="mr-3"
        text plain
        :loading="autosaving" v-if="message">
        Saved
        <v-icon right color="success">mdi-check</v-icon>
      </v-btn>
      <v-btn class="mr-3"
        @click="sendMessage"
        depressed
        color="primary"
        :loading="loading"
        v-if="message">
        <v-icon left>mdi-send</v-icon>
        Send message
      </v-btn>
      <v-btn
        @click="createMessage" depressed color="success" :loading="loading" :disabled="!canCreate"
        v-else class="mr-5">
        Create message
      </v-btn>

      <v-menu bottom v-if="message">
        <template v-slot:activator="{ on }">
          <v-btn icon v-on="on">
            <v-icon>mdi-dots-vertical</v-icon>
          </v-btn>
        </template>

        <v-list>
          <v-list-item @click="deleteMessage">
            <v-list-item-icon>
              <v-icon>mdi-delete</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Delete message</v-list-item-title>
          </v-list-item>

          <v-list-item @click="sendTestMessage">
            <v-list-item-icon>
              <v-icon>mdi-send-outline</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Send test message</v-list-item-title>
          </v-list-item>
        </v-list>
      </v-menu>
    </v-row>


    <v-row>
      <v-col cols="12">
        <v-text-field v-model="subject" label="Subject" outlined/>
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
import {
  CreateMessage, Message, UpdateMessage,
} from '@/domain/newsletter/model';


export default VueApp.extend({
  name: 'BNewsletterMessage',
  components: {
    BMarkdownEditor,
  },
  props: {
    message: {
      type: Object as PropType<Message | null>,
      required: false,
      default: null,
    },
    list: {
      type: String as PropType<string>,
      required: true,
    },
  },
  data() {
    return {
      loading: false,
      error: '',
      autosaving: false,

      subject: '',
      body: '',
      bodyHtml: '',
      autoSaveInterval: null as number | null,
    };
  },
  computed: {
    canCreate(): boolean {
      return this.subject.length !== 0;
    },
  },
  mounted() {
    this.clearFields();
  },
  created() {
    if (this.message) {
      this.autoSaveInterval = setInterval(() => {
        this.autosave();
      }, 700);
    }
  },
  beforeDestroy() {
    if (this.autoSaveInterval) {
      clearInterval(this.autoSaveInterval);
      this.autoSaveInterval = null;
      this.autosave();
    }
  },
  methods: {
    cancel() {
      this.$router.push({ path: `/newsletter/lists/${this.list}` });
    },
    clearFields() {
      if (this.message) {
        this.subject = this.message.subject;
        this.body = this.message.body;
        this.bodyHtml = this.message.body_html;
      } else {
        this.subject = '';
        this.body = '';
        this.bodyHtml = '';
      }
    },
    async autosave() {
      if (this.message
        && (this.subject !== this.message.subject
        || this.body !== this.message.body)) {
        this.updateMessage(true);
      }
    },
    async createMessage() {
      this.loading = true;
      this.error = '';
      const input: CreateMessage = {
        list_id: this.list,
        name: this.subject,
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
    async updateMessage(autosave: boolean) {
      if (!autosave) {
        this.loading = true;
        this.error = '';
      } else {
        this.autosaving = true;
      }
      const input: UpdateMessage = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        message_id: this.message!.id,
        list_id: this.list,
        name: this.subject,
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
        this.autosaving = false;
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
        await this.$newsletterService.deleteMessage(this.message!.id, this.list);
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
