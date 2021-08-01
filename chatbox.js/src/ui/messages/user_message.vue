<template>
  <div class="user-message">
    <span>{{ message.body_html }}</span>
  </div>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';
import { ChatboxMessage } from '@/domain/chatbox/model';


export default VueApp.extend({
  name: 'UserMessage',
  props: {
    message: {
      type: Object as PropType<ChatboxMessage>,
      required: true,
    },
  },
});
</script>

<style lang="scss" scoped>
.user-message {
  align-items: flex-end;
  display: flex;
  flex-direction: row;
  justify-content: flex-end;
  margin: 4px 0 4px auto;
  max-width: 85%;
  text-align: right;

  background-color: #fff;
  word-wrap: break-word;
  padding: 10px;
  border-radius: 21px 21px 0px 21px;
  min-width: 1%;
}
</style>
