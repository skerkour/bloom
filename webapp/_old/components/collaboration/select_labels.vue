<template>
  <v-combobox
    v-model="labels"
    :items="items"
    chips
    label="Labels"
    multiple
    item-text="name"
    :readonly="readonly"
  >
    <template v-slot:selection="{ attrs, item, select, selected }">
      <b-label
        v-bind="attrs"
        :input-value="selected"
        :close="!readonly"
        @click="select"
        @closed="remove(item)"
        :label="item"
      />
    </template>
  </v-combobox>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';
import { Label } from '@/api/graphql/model';
import BLabel from '@/ui/components/collaboration/label.vue';

export default VueApp.extend({
  name: 'BSelectLabels',
  components: {
    BLabel,
  },
  props: {
    value: {
      type: Array as PropType<Label[]>,
      required: true,
    },
    items: {
      type: Array as PropType<Label[]>,
      required: true,
    },
    readonly: {
      type: Boolean as PropType<boolean>,
      required: false,
      default: false,
    },
  },
  computed: {
    labels: {
      set(labels: Label[]) {
        this.$emit('input', labels);
      },
      get(): Label[] {
        return this.value;
      },
    },
  },
  methods: {
    remove(labelToRemove: Label) {
      this.labels = this.labels.filter((label: Label) => label.id !== labelToRemove.id);
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
