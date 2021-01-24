<template>
  <div class="chatbox">
    <chatbox-header />

    <reconnecting-message v-if="reconnecting" />

    <chatbox-conversation />

    <chatbox-input />

    <chatbox-footer v-if="showFooter" />
  </div>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';
import ChatboxHeader from './chatbox_header.vue';
import ChatboxInput from './chatbox_input.vue';
import ChatboxConversation from './chatbox_conversation.vue';
import ChatboxFooter from './chatbox_footer.vue';
import ReconnectingMessage from './reconnecting.vue';

export default VueApp.extend({
  name: 'Chatbox',
  components: {
    ChatboxHeader,
    ChatboxInput,
    ChatboxConversation,
    ChatboxFooter,
    ReconnectingMessage,
  },
  computed: {
    showFooter(): boolean {
      return this.$store.state.showFooter;
    },
    reconnecting(): boolean {
      return this.$store.state.reconnecting;
    },
  },
});
</script>

<style lang="scss" scoped>
.chatbox {
  width: 370px;
  height: calc(100% - 120px);
  max-height: 590px;
  position: fixed;
  right: 25px;
  bottom: 25px;
  box-sizing: border-box;
  box-shadow: 0px 7px 40px 2px rgba(148, 149, 150, 0.3);
  background: white;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  transition: 0.4s ease-in-out;
  border-radius: 10px;
  font-family: "Helvetica Neue", Helvetica, Arial, sans-serif;
  z-index: 9999;

  @media (max-width: 450px) {
    width: 100%;
    height: 100%;
    max-height: 100%;
    right: 0px;
    bottom: 0px;
    border-radius: 0px;
  }
}
</style>
