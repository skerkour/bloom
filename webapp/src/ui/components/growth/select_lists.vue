<template>
  <v-combobox
    v-model="lists"
    :items="items"
    chips
    :label="label"
    :multiple="multiple"
    item-text="name"
    :readonly="readonly"
  >
    <template v-slot:selection="{ attrs, item, select, selected }">
      <v-chip
        v-bind="attrs"
        :input-value="selected"
        :close="!readonly"
        @click="select"
        @click:close="remove(item)"
        >
        <strong>{{ item.name }}</strong>
      </v-chip>
      <!-- <b-label
        v-bind="attrs"
        :input-value="selected"
        :close="!readonly"
        @click="select"
        @closed="remove(item)"
        :label="item"
      /> -->
    </template>
  </v-combobox>
</template>


<script lang="ts">
/* eslint-disable @typescript-eslint/no-explicit-any */
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';
import { List } from '@/api/graphql/model';

export default VueApp.extend({
  name: 'BSelectLists',
  props: {
    value: {
      type: [Array, Object] as PropType<any>,
      required: false,
    },
    items: {
      type: Array as PropType<List[]>,
      required: true,
    },
    readonly: {
      type: Boolean as PropType<boolean>,
      required: false,
      default: false,
    },
    multiple: {
      type: Boolean as PropType<boolean>,
      required: false,
      default: false,
    },
  },
  data() {
    return {
      label: 'Lists',
    };
  },
  computed: {
    lists: {
      set(value: any) {
        this.$emit('input', value);
      },
      get(): any {
        return this.value;
      },
    },
  },
  created() {
    if (!Array.isArray(this.lists)) {
      this.label = 'List';
    }
  },
  methods: {
    remove(listToRemove: List) {
      if (Array.isArray(this.lists)) {
        this.lists = this.lists.filter((list: List) => list.id !== listToRemove.id);
      } else {
        this.lists = null;
      }
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
