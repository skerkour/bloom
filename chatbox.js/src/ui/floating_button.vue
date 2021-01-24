<template>
  <div class="floating-button" @click="openChatbox" :style="style">
    <b-chatbox-icon />
  </div>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';
import { Mutation } from '@/app/store';
import BChatboxIcon from '@/assets/chatbox.vue';

export default VueApp.extend({
  name: 'FloatingButton',
  components: {
    BChatboxIcon,
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
    openChatbox() {
      this.$store.commit(Mutation.OPEN);
    },
  },
});
</script>


<style lang="scss" scoped>
.floating-button {
  width: 60px;
  height: 60px;
  background-color: #4e8cff;
  background-position: center;
  background-repeat: no-repeat;
  position: fixed;
  right: 25px;
  bottom: 25px;
  border-radius: 50%;
  box-shadow: none;
  transition: box-shadow 0.2s ease-in-out;
  cursor: pointer;
  z-index: 9999;
}

.floating-button:hover {
  box-shadow: 0 0px 27px 1.5px rgba(0,0,0,0.2);
}

svg {
  width: 28px;
  height: 28px;
  position: fixed;
  right: 41px;
  bottom: 40px;
  transition: opacity 100ms ease-in-out, transform 100ms ease-in-out;
}
</style>
