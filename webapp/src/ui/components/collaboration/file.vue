<template>
  <v-container fluid class="fill-height">

    <v-row>
      <v-toolbar flat>
        <v-toolbar-title>
          <b-file-path :file="file" :projectFullPath="projectFullPath" />
        </v-toolbar-title>
      </v-toolbar>
    </v-row>

    <v-row justify="center" class="text-center">
      <v-avatar size="256">
        <v-icon>mdi-file</v-icon>
      </v-avatar>
    </v-row>

    <v-row justify="center" class="text-center">
      <v-col cols="12">
        <p class="headline">
          {{ file.name }}
        </p>
      </v-col>

       <v-col cols="12">
         <p class="h4 text--secondary">
            {{ calendar(file.createdAt) }} - {{ filesize(file.size) }}
         </p>
      </v-col>

      <v-col cols="12">
         <v-btn color="primary" depressed @click="downloadFile(file)">
           <v-icon left>mdi-download</v-icon>
           Download
         </v-btn>
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { PropType } from 'vue';
import { VueApp } from '@/app/vue';
import { File } from '@/api/graphql/model';
import BFilePath from '@/ui/components/collaboration/file_path.vue';
import { calendar, filesize } from '@/app/filters';


export default VueApp.extend({
  name: 'BFile',
  components: {
    BFilePath,
  },
  props: {
    file: {
      type: Object as PropType<File>,
      required: true,
    },
  },
  computed: {
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
  },
  methods: {
    calendar,
    filesize,
    downloadFile(file: File) {
      this.$collaborationService.downloadFile(this.projectFullPath, file);
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
