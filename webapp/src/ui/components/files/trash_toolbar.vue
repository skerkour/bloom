<template>
  <div>
    <v-app-bar flat color="white">
      <!-- <v-toolbar-title class="headline">Trash</v-toolbar-title> -->

      <v-spacer />

      <!-- <v-tooltip bottom>
        <template v-slot:activator="{ on }">
          <v-btn icon @click="restoreFiles" :loading="loading" v-on="on" :disabled="!canRestore">
            <v-icon>mdi-history</v-icon>
          </v-btn>
        </template>
        <span>Restore</span>
      </v-tooltip> -->

      <v-tooltip bottom>
        <template v-slot:activator="{ on }">
          <v-btn icon @click="emptyTrash" :loading="loading" v-on="on">
            <v-icon>mdi-delete</v-icon>
          </v-btn>
        </template>
        <span>Delete forever</span>
      </v-tooltip>

    </v-app-bar>
    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
      {{ error }}
    </v-alert>
  </div>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { File } from '@/domain/files/model';
import { PropType } from 'vue';


export default VueApp.extend({
  name: 'BTrashToolbar',
  props: {
    selected: {
      type: Array as PropType<File[]>,
      required: true,
    },
  },
  computed: {
    canRestore(): boolean {
      return this.selected.length > 0;
    },
  },
  data() {
    return {
      loading: false,
      error: '',
    };
  },
  methods: {
    async emptyTrash(): Promise<void> {
      // eslint-disable-next-line no-alert, no-restricted-globals
      if (!confirm('Are you sure you want to empty trash? Files will be delete forever!')) {
        return;
      }

      this.loading = true;
      this.error = '';

      try {
        await this.$filesService.emptyTrash();
        this.$emit('emptied');
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async restoreFiles(): Promise<void> {
      this.loading = true;
      this.error = '';
      const files = this.selected.map((file: File) => file.id);

      try {
        await this.$filesService.restoreFilesFromTrash(files);
        this.$emit('restored', this.selected);
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
