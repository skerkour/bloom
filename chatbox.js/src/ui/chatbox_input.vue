<template>
  <div class="input-container">
    <resizable-textarea
      class="input"
      :placeholder="placeholder"
      v-model="input"
      @keydown="onInputKedown"
    />
    <div class="buttons-container">
      <!-- <img src="@/assets/send.svg" alt="send icon" @click="sendMessage" class="pointer" /> -->
      <div @click="sendMessage">
        <b-send-icon alt="send icon"  class="pointer" />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';
import BSendIcon from '@/assets/send.vue';
import ResizableTextarea from './resizable_textarea.vue';

export default VueApp.extend({
  name: 'Input',
  components: {
    ResizableTextarea,
    BSendIcon,
  },
  data() {
    return {
      placeholder: 'Compose your message...',
      input: '',
    };
  },
  methods: {
    sendMessage() {
      if (this.input && this.input.trim()) {
        this.$chatbox.sendMessage(this.input);
      }
      this.input = '';
    },
    onInputKedown(e: KeyboardEvent) {
      if (e.keyCode === 13 && !e.shiftKey) {
        e.preventDefault();
        this.sendMessage();
      }
    },
  },
});
</script>

<style lang="scss" scoped>
@import '@/app/variables.scss';

.input-container {
  margin: 0 $space-small 0 $space-small;
  align-items: center;
  display: flex;
}

.input {
  outline: none;
  width: 100%;

  border: 0;
  height: $space-large;
  min-height: $space-large;
  max-height: 2.4 * $space-larger;
  resize: none;
  padding-top: $space-small;
  padding-bottom: $space-small;

  box-sizing: border-box;
  display: block;
  font-size: 1rem;
}

.buttons-container {
  width: 42px;
  display: flex;
  align-items: center;
  text-align: center;

  svg {
    margin: auto;
    display: block;
  }
}

.pointer {
  cursor: pointer;
}

// .emoji-toggle {
//   font-size: $font-size-large;
//   color: #ababab;
//   padding-right: $space-smaller;
//   cursor: pointer;
// }

// .emoji-dialog {
//   right: $space-one;
// }

// .file-uploads {
//   margin-right: $space-small;
// }
</style>
