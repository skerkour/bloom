<template>
  <v-dialog
    v-model="show"
    max-width="400px"
    persistent
    scrollable
  >

    <v-card>
      <v-card-title class="headline">
        Uploading files
      </v-card-title>

      <v-card-text>
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>

        <v-list>
          <v-list-item v-for="file in files" :key="file.name">
            <v-list-item-content>
              <v-list-item-title>{{ file.name }}</v-list-item-title>
            </v-list-item-content>

            <v-list-item-action>
              <v-icon v-if="file.progress === 100 && !error" color="green">
                mdi-check-circle
              </v-icon>
              <v-progress-circular color="primary" :value="file.progress" v-else />
            </v-list-item-action>
          </v-list-item>
        </v-list>


      </v-card-text>

      <v-card-actions>
        <v-btn text @click="cancel">Cancel</v-btn>

        <v-spacer />
      </v-card-actions>

    </v-card>
  </v-dialog>
</template>


<script lang="ts">
import { PropType } from 'vue';
import { VueApp } from '@/app/vue';
import { UploadingFile } from '@/domain/files/service';

export default VueApp.extend({
  name: 'BFilesUploadDialog',
  props: {
    value: {
      type: Boolean as PropType<boolean>,
      required: true,
    },
    error: {
      type: String as PropType<string>,
      required: true,
    },
    files: {
      type: Array as PropType<UploadingFile[]>,
      required: true,
    },
  },
  data() {
    return {
      loading: false,
    };
  },
  computed: {
    show: {
      get(): boolean {
        return this.value;
      },
      set(value: boolean) {
        this.$emit('input', value);
      },
    },
  },
  methods: {
    close() {
      this.show = false;
    },
    cancel() {
      this.$emit('cancel');
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
