<template>
  <div class="ask-email-message" :style="style">
    <div>
      What is your email address?
    </div>
    <div v-if="error">
      Error: {{ error }}
    </div>
    <div class="ask-email-input">
      <input
        v-model="email"
        class="email-input"
        type="email"
        @keyup.enter="linkContact"
      />
      <span @click="linkContact" class="pointer send-btn">
        <b-send-icon alt="send icon" color="white" />
      </span>
    </div>
  </div>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';
import BSendIcon from '@/assets/send.vue';


export default VueApp.extend({
  components: {
    BSendIcon,
  },
  name: 'AskEmailMessage',
  data() {
    return {
      error: '',
      loading: false,
      email: '',
    };
  },
  computed: {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    style(): any {
      return {
        backgroundColor: this.$store.state.preferences?.color,
      };
    },
  },
  methods: {
    close() {
      this.$chatbox.closeAskEmailMessage();
    },
    async linkContact() {
      this.loading = true;
      this.error = '';

      try {
        await this.$chatbox.linkContact(this.email);
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
.ask-email-message {
  align-items: flex-end;
  // display: flex;
  flex-direction: row;
  justify-content: flex-start;
  align-items: flex-start;
  align-self: flex-start;

  text-align: left;
  margin: 4px 0 4px 0px;
  max-width: 88%;
  color: #fff;
  // word-wrap: break-word;
  padding: 10px;
  border-radius: 21px 21px 21px 0px;
  min-width: 1%;
}

.ask-email-input {
  display: flex;
  align-items: center;
}

.email-input {
  margin-top: 10px;
  border: 1px solid #e9e9e9;
  border-radius: 5px;
  padding: 0.4rem 0.8rem;
}

.pointer {
  cursor: pointer;
}

.send-btn {
  margin-left: 6px;
  padding-top: 12px;
}
</style>
