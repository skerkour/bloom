<template>
  <v-container fluid id="blm-dropzone">
    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
      {{ error }}
    </v-alert>

    <b-folder-toolbar
      :folder="folder"
      @new-folder-clicked="openNewFolderDialog"
      @upload-files-clicked="onUploadFilesClicked"
    />

    <b-files-list
      :files="folder.children"
      @move-to-trash="onMoveToTrash"
      @rename="onFileRename"
      v-model="selected"
      :loading="loading"
    />

    <b-new-folder-dialog
      v-model="showNewFolderDialog"
      :parent="folder"
      @created="onFolderCreated"
    />

    <b-rename-file-dialog
      v-if="showRenameFileDialog"
      v-model="showRenameFileDialog"
      :file="fileToRename"
      @renamed="onFileRenamed"
    />

    <b-files-upload-dialog
      v-model="showFilesUploadDialog"
      :error="filesUploadError"
      :files="uploadingFiles"
      @cancel="onCancelUploads"
    />

    <input type="file" class="files-input" ref="files-input"
      multiple v-on:change="handleFilesUpload(true)" />

    <v-overlay :value="droppingFiles">
      <h3 class="text-h4">
        Drop to upload
      </h3>
    </v-overlay>
  </v-container>
</template>


<script lang="ts">
import { PropType } from 'vue';
import axios, { CancelTokenSource } from 'axios';
import { VueApp } from '@/app/vue';
import { CompleteFileUpload, File as ApiFile } from '@/domain/files/model';
import { UploadingFile } from '@/domain/files/service';
import BFilesList from './files_list.vue';
import BFolderToolbar from './folder_toolbar.vue';
import BNewFolderDialog from './new_folder_dialog.vue';
import BRenameFileDialog from './rename_file_dialog.vue';
import BFilesUploadDialog from './files_upload_dialog.vue';


export default VueApp.extend({
  name: 'BFolder',
  components: {
    BFilesList,
    BFolderToolbar,
    BNewFolderDialog,
    BRenameFileDialog,
    BFilesUploadDialog,
  },
  props: {
    folder: {
      type: Object as PropType<ApiFile>,
      required: true,
    },
  },
  data() {
    return {
      showNewFolderDialog: false,
      error: '',
      loading: false,
      fileToRename: null as ApiFile | null,
      showRenameFileDialog: false,
      showFilesUploadDialog: false,
      selected: [],
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      axiosCancelSource: null as CancelTokenSource | null,
      filesUploadError: '',
      uploadingFiles: [] as UploadingFile[],
      filesToUpload: [] as File[],
      droppingFiles: false,
    };
  },
  mounted() {
    if (this.isDragAndDropCapable()) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const dropzone = document.querySelector('#blm-dropzone')!;
      ['drag', 'dragstart', 'dragend', 'dragover', 'dragenter', 'dragleave', 'drop']
        .forEach((evt) => {
          /*
            For each event add an event listener that prevents the default action
            (opening the file in the browser) and stop the propagation of the event (so
            no other elements open the file in the browser)
          */
          dropzone.addEventListener(evt, (e) => {
            e.preventDefault();
            e.stopPropagation();
            this.droppingFiles = true;
          }, false);
        });
      // Add an event listener for drop to the form
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      dropzone.addEventListener('drop', (e: any) => {
        if (!e.dataTransfer || !e.dataTransfer.files) {
          return;
        }

        this.filesToUpload = [];
        e.dataTransfer.files.forEach((file: File) => {
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          (this.filesToUpload as any).push(file);
        });

        this.droppingFiles = false;
        this.handleFilesUpload(false);
      });
    }
  },
  methods: {
    openNewFolderDialog() {
      this.showNewFolderDialog = true;
    },
    onFolderCreated(file: ApiFile) {
      this.folder.children?.push(file);
    },
    async onMoveToTrash(files: ApiFile[]): Promise<void> {
      this.loading = true;
      this.error = '';
      const fileIds = files.map((file: ApiFile) => file.id);

      try {
        await this.$filesService.moveFilesToTrash(fileIds);
        const trashedSet = new Set<string>();
        files.forEach((file: ApiFile) => trashedSet.add(file.id));
        // eslint-disable-next-line max-len, @typescript-eslint/no-non-null-assertion
        this.folder.children = this.folder.children!.filter((file: ApiFile) => !trashedSet.has(file.id));
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    onFileRename(fileToRename: ApiFile) {
      this.fileToRename = fileToRename;
      this.showRenameFileDialog = true;
    },
    onFileRenamed(renamedFile: ApiFile) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      this.folder.children = this.folder.children!.map((file: ApiFile) => {
        if (file.id === renamedFile.id) {
          file.name = renamedFile.name;
        }
        return file;
      });
      this.showRenameFileDialog = false;
    },
    onUploadFilesClicked() {
      (this.$refs['files-input'] as HTMLElement).click();
    },
    async handleFilesUpload(direct: boolean) {
      if (direct) {
        this.filesToUpload = (this.$refs['files-input'] as HTMLInputElement).files as unknown as File[];
      }

      if (!this.filesToUpload || this.filesToUpload.length === 0) {
        return;
      }

      this.showFilesUploadDialog = true;

      try {
        this.uploadingFiles = Array.from(this.filesToUpload).map((file: File) => ({
          name: file.name,
          progress: 0,
        }));
        for (let i = 0; i < this.filesToUpload.length; i += 1) {
          const fileToUpload = this.filesToUpload[i];
          this.axiosCancelSource = axios.CancelToken.source();

          // get presignedUrl
          // eslint-disable-next-line no-await-in-loop, max-len
          const signedUrlData = await this.$kernelService.signedUploadUrl(fileToUpload.size);

          // upload to s3
          const options = {
            cancelToken: this.axiosCancelSource.token,
            headers: {
              // 'Content-Type': 'multipart/form-data',
              // Authorization: undefined,
            },
            onUploadProgress: (progressEvent: ProgressEvent) => {
              this.uploadingFiles[i].progress = Math.ceil(
                (progressEvent.loaded / progressEvent.total) * 100,
              );
              this.$set(this.uploadingFiles, i, this.uploadingFiles[i]);
            },
          };
          // eslint-disable-next-line no-await-in-loop
          await axios.put(signedUrlData.url, fileToUpload, options);
          // const uploadedFile = await
          // this.$collaborationService.uploadFile(this.folder.id, fileToUpload, options);

          // complete upload
          const completeUploadInput: CompleteFileUpload = {
            upload_id: signedUrlData.upload_id,
            name: fileToUpload.name,
            parent_id: this.folder.id,
            mime_type: fileToUpload.type,
          };

          // eslint-disable-next-line no-await-in-loop, max-len
          const uploadedFile = await this.$filesService.completeFileUpload(completeUploadInput);
          this.folder.children?.push(uploadedFile);
        }
        this.uploadingFiles = [];
        this.showFilesUploadDialog = false;
      } catch (err) {
        if (err.message) {
          this.filesUploadError = err.message;
        }
      } finally {
        this.loading = false;
        this.axiosCancelSource = null;
        this.filesToUpload = [];
      }
    },
    onCancelUploads() {
      this.showFilesUploadDialog = false;
      this.filesUploadError = '';
      if (this.axiosCancelSource) {
        this.axiosCancelSource.cancel();
      }
    },
    isDragAndDropCapable(): boolean {
      const div = document.createElement('div');
      return (('draggable' in div)
        || ('ondragstart' in div && 'ondrop' in div))
        && 'FormData' in window
        && 'FileReader' in window;
    },
  },
});
</script>


<style lang="scss" scoped>
.files-input {
  display: none;
}
</style>
