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
      <v-col cols="12" sm="6">
        <v-select
          :items="messageTypeItems"
          label="Type"
          value="Email"
        />
      </v-col>


      <v-col cols="12" sm="6">
        <v-text-field v-model="fromName" label="From (name)" />
      </v-col>
      <v-col cols="12" sm="6">
        <v-text-field v-model="fromAddress" label="From (email address)" />
      </v-col>

      <v-col cols="12">
        <v-text-field v-model="subject" label="Subject" />
      </v-col>

      <v-col cols="12">
        <b-select-lists v-model="list" :items="projectLists" />
        <p>
          Select the default list if you want to send your message to all your contacts
        </p>
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
import {
  OutboundMessage, DeleteOutboundMessageInput, SendTestOutboundMessageInput,
  CreateOutboundMessageInput, OutboundMessageType, UpdateOutboundMessageInput,
  SendOutboundMessageInput, List,
} from '@/api/graphql/model';
import BMarkdownEditor from '@/ui/components/kernel/markdown_editor.vue';
import BSelectLists from '@/ui/components/growth/select_lists.vue';


export default VueApp.extend({
  name: 'BOutboundMessage',
  components: {
    BMarkdownEditor,
    BSelectLists,
  },
  props: {
    message: {
      type: Object as PropType<OutboundMessage | null>,
      required: false,
      default: null,
    },
    projectLists: {
      type: Array as PropType<List[]>,
      required: true,
    },
  },
  data() {
    return {
      loading: false,
      error: '',

      name: '',
      fromName: '',
      fromAddress: '',
      subject: '',
      body: '',
      bodyHtml: '',
      type: OutboundMessageType.Standard,
      list: null as List | null,
      messageTypeItems: ['Email'],
    };
  },
  computed: {
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
    canCreate(): boolean {
      return this.name.length !== 0;
    },
  },
  mounted() {
    this.clearFields();
  },
  methods: {
    cancel() {
      this.$router.push({ path: `/${this.projectFullPath}/-/outbound` });
    },
    clearFields() {
      if (this.message) {
        this.name = this.message.name;
        this.subject = this.message.subject;
        this.fromName = this.message.fromName;
        this.fromAddress = this.message.fromAddress;
        this.body = this.message.body;
        this.bodyHtml = this.message.bodyHtml;
        this.type = this.message.type;
        if (this.message.lists.length === 1) {
          [this.list] = this.message.lists;
        } else {
          this.list = null;
        }
      } else {
        this.name = '';
        this.fromName = '';
        this.fromAddress = '';
        this.subject = '';
        this.body = '';
        this.bodyHtml = '';
        this.type = OutboundMessageType.Standard;
        this.list = null;
      }
    },
    async createMessage() {
      this.loading = true;
      this.error = '';
      const listIds = this.list === null ? [] : [this.list.id];
      const input: CreateOutboundMessageInput = {
        projectFullPath: this.projectFullPath,
        name: this.name,
        fromName: this.fromName,
        fromAddress: this.fromAddress,
        subject: this.subject,
        body: this.body,
        sendAt: null,
        type: this.type,
        lists: listIds,
      };

      try {
        await this.$growthService.createOutboundMessage(input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async updateMessage() {
      this.loading = true;
      this.error = '';
      const listIds = this.list === null ? [] : [this.list.id];
      const input: UpdateOutboundMessageInput = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        messageId: this.message!.id,
        name: this.name,
        fromName: this.fromName,
        fromAddress: this.fromAddress,
        subject: this.subject,
        body: this.body,
        sendAt: null,
        type: this.type,
        lists: listIds,
      };

      try {
        const message = await this.$growthService.updateOutboundMessage(input);
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
      const input: DeleteOutboundMessageInput = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        messageId: this.message!.id,
      };

      try {
        await this.$growthService.deleteOutboundMessage(this.projectFullPath, input);
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
      const input: SendOutboundMessageInput = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        messageId: this.message!.id,
      };

      try {
        await this.$growthService.sendOutboundMessage(input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async sendTestMessage() {
      this.loading = true;
      this.error = '';
      const input: SendTestOutboundMessageInput = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        messageId: this.message!.id,
      };

      try {
        await this.$growthService.sendTestOutboundMessage(input);
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
