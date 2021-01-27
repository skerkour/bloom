<template>
  <v-combobox
    v-model="milestones"
    :items="comboboxItems"
    chips
    label="Milestones"
    multiple
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
        <strong>{{ item.title }}</strong>
      </v-chip>
    </template>
  </v-combobox>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';
import { Milestone } from '@/api/graphql/model';

export default VueApp.extend({
  name: 'BSelectMilestones',
  props: {
    value: {
      type: Array as PropType<Milestone[]>,
      required: true,
    },
    items: {
      type: Array as PropType<Milestone[]>,
      required: true,
    },
    readonly: {
      type: Boolean as PropType<boolean>,
      required: false,
      default: false,
    },
  },
  computed: {
    milestones: {
      set(milestones: Milestone[]) {
        this.$emit('input', milestones);
      },
      get(): Milestone[] {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        return this.value.map((item: any) => {
          item.name = item.title;
          return item;
        });
      },
    },
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    comboboxItems(): any {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      return this.items.map((item: any) => {
        item.name = item.title;
        return item;
      });
    },
  },
  methods: {
    remove(milestoneToRemove: Milestone) {
      // eslint-disable-next-line max-len
      this.milestones = this.milestones.filter((milestone: Milestone) => milestone.id !== milestoneToRemove.id);
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
