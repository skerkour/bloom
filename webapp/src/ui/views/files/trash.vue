<template>
  <v-container fluid>

    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center" class="text-center" v-if="loading">
      <v-col cols="12" sm="10" md="8" xl="6">
        <v-progress-circular
          :size="50"
          color="primary"
          indeterminate
        />
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12" class="pb-0">
        <h2>Trash</h2>
        <p>
          Files are automatically deleted forever after they've been in your trash for 30 days.
        </p>
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12" class="py-0">
        <b-trash-toolbar  :selected="selected"
          @emptied="onTrashEmptied" @restored="onFilesRestored" />
      </v-col>
    </v-row>

    <v-row justify="center">
      <v-col cols="12">
        <b-files-list :files="files" trash v-model="selected" @restore="onRestoreFiles" />
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { File } from '@/domain/files/model';
import BFilesList from '@/ui/components/files/files_list.vue';
import BTrashToolbar from '@/ui/components/files/trash_toolbar.vue';


export default VueApp.extend({
  name: 'ProjectFilesView',
  components: {
    BFilesList,
    BTrashToolbar,
  },
  data() {
    return {
      loading: false,
      error: '',
      files: [] as File[],
      selected: [] as File[],
    };
  },
  created() {
    this.fetchData();
  },
  methods: {
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.files = await this.$filesService.fetchTrash();
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    onTrashEmptied() {
      this.files = [];
    },
    async onRestoreFiles(files: File[]): Promise<void> {
      // this.loading = true;
      this.error = '';
      const fileIds = files.map((file: File) => file.id);

      try {
        await this.$filesService.restoreFilesFromTrash(fileIds);
        this.onFilesRestored(fileIds);
      } catch (err) {
        this.error = err.message;
      }
    },
    onFilesRestored(files?: string[]) {
      const restoredSet = new Set<string>();
      if (!files) {
        this.selected.forEach((file: File) => {
          restoredSet.add(file.id);
        });
      } else {
        files.forEach((fileId: string) => {
          restoredSet.add(fileId);
        });
      }

      this.files = this.files.filter((file: File) => !restoredSet.has(file.id));
      this.selected = [];
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
