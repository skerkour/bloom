<template>
  <div class="footer" v-if="branding">
    <span>Powered by <a href="https://bloom.sh" target="_blank" rel="noopener" class="bloom">Bloom</a></span>
  </div>
</template>

<script lang="ts">
import { VueApp } from '../app/vue';


export default VueApp.extend({
  name: 'ChatboxFooter',
  computed: {
    branding(): boolean {
      return this.$store.state.preferences?.show_branding ?? true;
    },
  },
});
</script>

<style lang="scss">
.footer {
  background-color: #fff;
  height: 40px;
  width: 100%;
  text-align: center;
  border-top: 1px solid #eee;
  display: flex;
  justify-content: center;
  align-items: center;
  border-radius: 0 0 10px 10px;

  span {
    color: #bbb;
  }
  .bloom {
    color: #888;
    text-decoration: none;

    &:hover {
      text-decoration: underline;
    }
  }
}
</style>
