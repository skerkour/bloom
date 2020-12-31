<template>
  <textarea
    ref="textarea"
    :placeholder="placeholder"
    :value="value"
    @input="onInput"
    @focus="onFocus"
    @blur="onBlur"
    @keyup="onKeyup"
    @keydown="onKeydown"
    @keypress="onKeypress"
  />
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  props: {
    placeholder: {
      type: String,
      default: '',
    },
    value: {
      type: String,
      default: '',
    },
    minHeight: {
      type: Number,
      default: 3.2,
    },
  },
  watch: {
    value() {
      this.resizeTextarea();
    },
  },
  methods: {
    resizeTextarea() {
      if (!this.value) {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        (this.$el as any).style.height = `${this.minHeight}rem`;
      } else {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        (this.$el as any).style.height = `${this.$el.scrollHeight}px`;
      }
    },
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    onInput(event: any) {
      this.$emit('input', event.target.value);
      this.resizeTextarea();
    },
    onBlur() {
      this.$emit('blur');
    },
    onFocus() {
      this.$emit('focus');
    },
    onKeyup(event: KeyboardEvent) {
      this.$emit('keyup', event);
    },
    onKeydown(event: KeyboardEvent) {
      this.$emit('keydown', event);
    },
    onKeypress(event: KeyboardEvent) {
      this.$emit('keypress', event);
    },
    focus() {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (this.$refs.textarea as any).focus();
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
