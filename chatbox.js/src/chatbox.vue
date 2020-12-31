<template>
  <div>
    <transition name="slide-fade">
      <floating-button v-if="showFAB" />
    </transition>
    <transition name="slide-fade">
      <chatbox v-if="isOpen" />
    </transition>
  </div>
</template>

<script lang="ts">
import FloatingButton from '@/ui/floating_button.vue';
import { VueApp } from '@/app/vue';
import Chatbox from '@/ui/chatbox.vue';
import { Mutation } from '@/app/store';

export default VueApp.extend({
  name: 'BloomChatbox',
  components: {
    FloatingButton,
    Chatbox,
  },
  computed: {
    isOpen(): boolean {
      return this.$store.state.isOpen;
    },
    showFAB(): boolean {
      const ret = !this.$store.state.isOpen && this.$store.state.preferences !== null;
      return ret;
    },
  },
  created() {
    this.fetchData();
    this.subscribe();
  },
  beforeDestroy() {
    this.$chatbox.unsubscribeFromChatboxMessages();
  },
  methods: {
    subscribe() {
      this.$chatbox.subscribeToChatboxMessages();
    },
    async fetchData() {
      try {
        const chatbox = await this.$chatbox.fetchChatbox();
        this.$store.commit(Mutation.CHATBOX_FETCHED, chatbox);
      } catch (err) {
        console.error(err);
      }
    },
    // onDisconnected() {
    //   this.$store.commit(Mutation.DISCONNECTED);
    // },
    // onConnected() {
    //   this.$store.commit(Mutation.CONNECTED);
    //   this.fetchData();
    // },
  },
});
</script>

<style lang="scss" scoped>
.slide-fade-enter-active, .slide-fade-leave-active {
  transition: opacity .4s;
}
.slide-fade-enter, .slide-fade-leave-to /* .fade-leave-active below version 2.1.8 */ {
  opacity: 0;
}
</style>
