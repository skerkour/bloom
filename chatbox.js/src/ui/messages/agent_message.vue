<template>
  <div class="agent-message" :style="style">
    <span>{{ message.body_html }}</span>
  </div>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';
import { ChatboxMessage } from '@/domain/chatbox/model';

export default VueApp.extend({
  name: 'AgentMessage',
  props: {
    message: {
      type: Object as PropType<ChatboxMessage>,
      required: true,
    },
  },
  computed: {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    style(): any {
      return {
        backgroundColor: this.$store.state.preferences?.color,
      };
    },
  },
});
</script>

<style lang="scss" scoped>
.agent-message {
  align-items: flex-end;
  display: flex;
  flex-direction: row;
  justify-content: flex-start;
  align-items: flex-start;
  align-self: flex-start;

  text-align: left;
  margin: 4px 0 4px 0px;
  max-width: 88%;
  color: #fff;
  word-wrap: break-word;
  padding: 10px;
  border-radius: 21px 21px 21px 0px;
  min-width: 1%;
}
</style>
