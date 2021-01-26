<template>
  <v-dialog
    v-model="show"
    max-width="400px"
    @keydown.esc="show = false"
  >

    <v-card>
      <v-card-title class="headline">
        Rename
      </v-card-title>

      <v-card-text>
        <v-alert icon="mdi-alert-circle" :value="error !== ''" type="error" dismissible>
          {{ error }}
        </v-alert>
        <v-text-field
          label="Name"
          outline
          v-model="newFileName"
          :disabled="loading"
          @keyup.enter.native="renameFile"
        />
      </v-card-text>

      <v-card-actions>
        <v-btn text @click="cancel" :loading="loading">Cancel</v-btn>

        <v-spacer />


        <v-btn @click="renameFile" color="success" :loading="loading" depressed>
          Rename
        </v-btn>
      </v-card-actions>

    </v-card>
  </v-dialog>
</template>


<script lang="ts">
import { PropType } from 'vue';
import { VueApp } from '@/app/vue';
import { RenameFile, File } from '@/domain/files/model';

export default VueApp.extend({
  name: 'BRenameFileDialog',
  props: {
    file: {
      type: Object as PropType<File>,
      required: true,
    },
    value: {
      type: Boolean as PropType<boolean>,
      required: true,
    },
  },
  data() {
    return {
      loading: false,
      error: '',
      newFileName: '',
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
  created() {
    this.newFileName = this.file.name;
  },
  methods: {
    close() {
      this.show = false;
    },
    async renameFile() {
      this.loading = true;
      this.error = '';
      const input: RenameFile = {
        file_id: this.file.id,
        name: this.newFileName,
      };

      try {
        const file = await this.$filesService.renameFile(input);
        this.$emit('renamed', file);
        this.close();
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    cancel() {
      this.newFileName = this.file.name;
      this.close();
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
