<template>
  <v-container fluid>

    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <!-- <v-row justify="center" class="text-center" v-if="loading">
      <v-col cols="12" sm="10" md="8" xl="6">
        <v-progress-circular
          :size="50"
          color="primary"
          indeterminate
        />
      </v-col>
    </v-row> -->

    <v-row justify="center" v-if="file">
      <v-col cols="12">
        <folder :folder="file" v-if="isFolder" />
        <file :file="file" v-else />
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { File } from '@/domain/files/model';
import FileComponent from '@/ui/components/files/file.vue';
import Folder from '@/ui/components/files/folder.vue';


export default VueApp.extend({
  name: 'BFilesView',
  components: {
    file: FileComponent,
    Folder,
  },
  computed: {
    fileId(): string | null {
      return this.$route.params.fileId ?? null;
    },
    isFolder(): boolean {
      return this.file ? this.file.type === this.$filesService.fileTypeFolder : false;
    },
  },
  data() {
    return {
      loading: false,
      error: '',
      file: null as File | null,
    };
  },
  watch: {
    fileId() {
      this.fetchData();
    },
  },
  created() {
    this.fetchData();
  },
  methods: {
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.file = await this.$filesService.fetchFile(this.fileId);
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
