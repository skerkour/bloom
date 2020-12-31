<template>
  <div class="reconnecting">
    <div class="lds-ring"><div></div><div></div><div></div><div></div></div>
    <p>Reconnecting...</p>
  </div>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'Reconnecting',
});
</script>


<style lang="scss" scoped>
.reconnecting {
  background-color: #F44336;
  color: white;
  display: flex;

  p {
    font-size: 20px;
    margin: 5px;
    margin-left: 10px;
  }

  .lds-ring {
    display: inline-block;
    // position: relative;
    width: 25px;
    height: 25px;
    margin-top: 1px;
  }
  .lds-ring div {
    box-sizing: border-box;
    display: inline-block;
    position: absolute;
    width: 25px;
    height: 25px;
    margin: 3px;
    border: 3px solid #fff;
    border-radius: 50%;
    animation: lds-ring 1.2s cubic-bezier(0.5, 0, 0.5, 1) infinite;
    border-color: #fff transparent transparent transparent;
  }
  .lds-ring div:nth-child(1) {
    animation-delay: -0.45s;
  }
  .lds-ring div:nth-child(2) {
    animation-delay: -0.3s;
  }
  .lds-ring div:nth-child(3) {
    animation-delay: -0.15s;
  }
  @keyframes lds-ring {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }

}
</style>
