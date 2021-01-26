<template>
  <v-dialog
    v-model="show"
    max-width="400px"
    @keydown.esc="show = false"
  >

    <v-card>
      <v-card-title class="headline">
        New Folder
      </v-card-title>

      <v-card-text>
        <v-alert icon="mdi-alert-circle" :value="error !== ''" type="error" dismissible>
          {{ error }}
        </v-alert>
        <v-text-field
        label="Name"
        outline
        v-model="newFolderName"
        :disabled="loading"
        @keyup.enter.native="createFolder"
        ></v-text-field>
      </v-card-text>

      <v-card-actions>
        <v-btn text @click="cancel" :loading="loading">Cancel</v-btn>

        <v-spacer />

        <v-btn @click="createFolder" color="success" :loading="loading" depressed>
          Create folder
        </v-btn>
      </v-card-actions>

    </v-card>
  </v-dialog>
</template>


<script lang="ts">
import { PropType } from 'vue';
import { VueApp } from '@/app/vue';
import { File, CreateFolder } from '@/domain/files/model';

export default VueApp.extend({
  name: 'BNewFolderDialog',
  props: {
    parent: {
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
      newFolderName: '',
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
    async createFolder() {
      this.loading = true;
      this.error = '';
      const input: CreateFolder = {
        parent_id: this.parent.id,
        name: this.newFolderName,
      };

      try {
        const folder = await this.$filesService.createFolder(input);
        this.$emit('created', folder);
        this.cancel();
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    cancel() {
      this.newFolderName = '';
      this.close();
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
