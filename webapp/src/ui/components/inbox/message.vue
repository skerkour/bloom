<template>
  <div :class="{'operator-message': isOperatorMessage, 'user-message': isUserMessage}">
    <span>{{ message.body_html }}</span>
  </div>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';
import { Message } from '@/domain/inbox/model';
import { PropType } from 'vue';


export default VueApp.extend({
  name: 'BMessage',
  props: {
    message: {
      type: Object as PropType<Message>,
      required: true,
    },
  },
  computed: {
    isOperatorMessage(): boolean {
      return this.message.from_operator;
    },
    isUserMessage(): boolean {
      return !this.message.from_operator;
    },
  },
});
</script>

<style lang="scss" scoped>
$space-smaller: 0.4rem;
$font-size-large: 2rem;
$space-one: 1rem;
$space-small: 0.8rem;

.user-message {
  align-items: flex-end;
  display: flex;
  flex-direction: row;
  justify-content: flex-start;
  align-items: flex-start;
  align-self: flex-start;

  text-align: left;
  margin: 4px 0 4px 0px;
  max-width: 88%;
  background-color: #4e8cff;
  color: #fff;
  word-wrap: break-word;
  padding: 10px;
  border-radius: 21px 21px 21px 0px;
  min-width: 1%;
}

.operator-message {
  align-items: flex-end;
  display: flex;
  flex-direction: row;
  justify-content: flex-end;
  margin: 4px 4px 4px auto;
  max-width: 85%;
  text-align: right;

  background-color: #fff;
  word-wrap: break-word;
  padding: 10px;
  border-radius: 21px 21px 0px 21px;
  min-width: 1%;
}
</style>
