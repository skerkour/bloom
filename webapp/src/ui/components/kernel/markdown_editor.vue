<template>
  <div>
    <v-tabs v-model="tab" @change="onTabChanged">
      <v-tab>Write</v-tab>
      <v-tab>Preview</v-tab>
    </v-tabs>
    <v-tabs-items v-model="tab">
      <v-tab-item>
        <v-textarea
          :placeholder="placeholder"
          v-model="body"
          :outlined="outlined"
          class="my-2"
        />
      </v-tab-item>
      <v-tab-item>
        <v-progress-circular
          :size="50"
          color="primary"
          indeterminate
          v-if="loading"
        />
        <div v-html="bodyHtml" v-else/>
      </v-tab-item>
    </v-tabs-items>
  </div>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';

export default VueApp.extend({
  name: 'BMarkdownEditor',
  props: {
    value: {
      type: String as PropType<string>,
      required: true,
    },
    placeholder: {
      type: String as PropType<string>,
      required: false,
      default: '',
    },
    outlined: {
      type: Boolean as PropType<boolean>,
      required: false,
      default: false,
    },
  },
  data() {
    return {
      tab: 0,
      bodyHtml: '',
      loading: false,
      error: '',
    };
  },
  computed: {
    body: {
      get(): string {
        return this.value;
      },
      set(value: string) {
        this.$emit('input', value);
      },
    },
  },
  methods: {
    onTabChanged(tab: number) {
      if (tab === 1) {
        this.renderHtmlPreview();
      }
    },
    async renderHtmlPreview(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.bodyHtml = await this.$kernelService.markdown(this.body);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
