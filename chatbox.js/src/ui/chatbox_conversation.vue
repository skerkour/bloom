<template>
  <div class="conversation">
    <div class="conversation-top" >
      <span>a</span>
    </div>
    <agent-message :message="welcomeMessage" v-if="welcomeMessage" />
    <template v-for="message in messages">
      <agent-message :message="message" v-if="message.from_operator" :key="message.id" />
      <user-message :message="message" v-else :key="message.id" />
    </template>
    <div class="conversation-bottom" >
      <span>a</span>
    </div>
  </div>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';
import { ChatboxMessage } from '@/domain/chatbox/model';
import AgentMessage from './messages/agent_message.vue';
import UserMessage from './messages/user_message.vue';


export default VueApp.extend({
  name: 'ChatboxConversation',
  components: {
    AgentMessage,
    UserMessage,
  },
  watch: {
    messages() {
      setTimeout(this.scrollToBottom, 50);
    },
  },
  computed: {
    messages(): ChatboxMessage[] {
      return this.$store.state.messages;
    },
    welcomeMessage(): ChatboxMessage | null {
      if (this.$store.state.preferences?.welcome_message) {
        return {
          id: 'com.bloom42.inbox.chatbox_welcome_message',
          received_at: new Date().toISOString(),
          body_html: this.$store.state.preferences.welcome_message,
          from_operator: true,
        };
      }
      return null;
    },
  },
  mounted() {
    this.scrollToBottom();
  },
  methods: {
    scrollToBottom() {
      const container = this.$el;
      container.scrollTop = container.scrollHeight;
    },
  },
});
</script>

<style lang="scss" scoped>
.conversation {
  display: flex;
  flex-direction: column;
  flex: 1;
  overflow-y: auto;
  background-color: #f4f7fd;
  padding: 0 12px 0 10px;
}

// ugly hack to add a bottom to the conversation
.conversation-bottom, .conversation-top {
  background-color: #f4f7fd;
  color: #f4f7fd;
  font-size: 2px;
}
</style>
