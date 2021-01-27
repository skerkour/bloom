<template>
  <v-chip
    :input-value="inputValue"
    :close="close"
    @click:close="onClickClose"
    :color="label.backgroundColor"
    :text-color="label.textColor"
    >
    <strong>{{ label.name }}</strong>
  </v-chip>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';
import { Label } from '@/api/graphql/model';

export default VueApp.extend({
  name: 'BLabel',
  props: {
    label: {
      type: Object as PropType<Label>,
      required: true,
    },
    close: {
      type: Boolean as PropType<boolean>,
      required: false,
      default: false,
    },
    inputValue: {
      type: Boolean as PropType<boolean>,
      required: false,
      default: false,
    },
  },
  methods: {
    // onClick() {
    //   this.$emit('click', this.label);
    // },
    onClickClose() {
      this.$emit('closed', this.label);
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
