<template>
  <!-- <?xml version="1.0" encoding="UTF-8"?> -->
  <!-- eslint-disable -->
  <svg width="21px" height="18px" viewBox="0 0 21 18" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
    <!-- Generator: Sketch 64 (93537) - https://sketch.com -->
    <title>send</title>
    <desc>Created with Sketch.</desc>
    <g id="chatbox" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd">
        <g id="Artboard" transform="translate(-368.000000, -250.000000)" :fill="color" fill-rule="nonzero">
            <g id="send" transform="translate(368.000000, 250.000000)">
                <polygon id="Path" points="0 18 21 9 0 0 0 7 15 9 0 11"></polygon>
            </g>
        </g>
    </g>
</svg>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';

export default VueApp.extend({
  name: 'Send',
  props: {
    color: {
      type: String as PropType<string>,
      required: false,
      default: '#888888',
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
