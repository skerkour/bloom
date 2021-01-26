<template>
  <v-data-table
    v-model="selected"
    :headers="headers"
    :items="files"
    item-key="id"
    hide-default-footer
    :loading="loading"
  >
    <template v-slot:no-data>
      <p class="text-center">
        No files.
      </p>
    </template>


    <template v-slot:item="{ item }" class="text-left">
      <tr v-on:dblclick="goto(item)" :class="{'bloom-pointer': !trash }">
        <td>
          <span>
            <b-file-icon :file="item" />
          </span>
          <span class="ml-2">{{ item.name }}</span>
        </td>
        <td>
          <span v-if="trash">{{ calendar(item.trashedAt) }}</span>
          <span v-else>{{ calendar(item.createdAt) }}</span>
        </td>
        <td>
          <span>{{ filesize(item) }}</span>
        </td>

        <td>
          <v-menu bottom left>
            <template v-slot:activator="{ on }">
              <v-btn icon v-on="on">
                <v-icon>mdi-dots-vertical</v-icon>
              </v-btn>
            </template>

            <v-list>
              <v-list-item @click="downloadFile(item)" v-if="!isFolder(item) && !trash">
                <v-list-item-icon>
                  <v-icon>mdi-download</v-icon>
                </v-list-item-icon>
                <v-list-item-title>Download</v-list-item-title>
              </v-list-item>

              <v-list-item @click="renameFile(item)" v-if="!trash">
                <v-list-item-icon>
                  <v-icon>mdi-pencil</v-icon>
                </v-list-item-icon>
                <v-list-item-title>Rename</v-list-item-title>
              </v-list-item>

              <v-list-item @click="restoreFile(item)" v-if="trash">
                <v-list-item-icon>
                  <v-icon>mdi-history</v-icon>
                </v-list-item-icon>
                <v-list-item-title>Restore</v-list-item-title>
              </v-list-item>
              <v-list-item @click="trashFile(item)" v-else>
                <v-list-item-icon>
                  <v-icon>mdi-delete</v-icon>
                </v-list-item-icon>
                <v-list-item-title>Move to trash</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-menu>
        </td>
      </tr>
    </template>
  </v-data-table>
</template>


<script lang="ts">
import { PropType } from 'vue';
import { VueApp } from '@/app/vue';
import { File } from '@/domain/files/model';
import { calendar, filesize } from '@/app/filters';
import BFileIcon from './file_icon.vue';

// type SelectableFile = {
// }

export default VueApp.extend({
  name: 'BFilesList',
  components: {
    BFileIcon,
  },
  props: {
    files: {
      type: Array as PropType<File[]>,
      required: true,
    },
    trash: {
      type: Boolean as PropType<boolean>,
      required: false,
      default: false,
    },
    loading: {
      type: Boolean as PropType<boolean>,
      required: false,
      default: false,
    },
    value: {
      type: Array as PropType<File[]>,
      required: true,
    },
  },
  data() {
    return {
      selected: [],
      headers: [
        {
          text: 'Name',
          align: 'start',
          sortable: true,
          value: 'name',
          width: '60%',
        },
        {
          text: this.trash ? 'Trashed at' : 'Updated at',
          align: 'start',
          sortable: true,
          value: this.trash ? 'trashed_at' : 'created_at',
        },
        {
          text: 'Size',
          align: 'start',
          sortable: true,
          value: 'size',
        },
        {
          text: 'Actions',
          align: 'start',
          sortable: false,
          value: 'actions',
        },
      ],
    };
  },
  methods: {
    calendar,
    filesize(file: File): string {
      if (this.isFolder(file)) {
        return '-';
      }
      return filesize(file.size);
    },
    isFolder(file: File): boolean {
      return file.type === this.$filesService.fileTypeFolder;
    },
    goto(file: File) {
      if (this.trash) {
        return;
      }
      this.$router.push({ path: `/files/${file.id}` });
    },
    trashFile(file: File) {
      this.$emit('move-to-trash', [file]);
    },
    restoreFile(file: File) {
      this.$emit('restore', [file]);
    },
    renameFile(file: File) {
      this.$emit('rename', file);
    },
    downloadFile(file: File) {
      this.$filesService.downloadFile(file.id);
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
