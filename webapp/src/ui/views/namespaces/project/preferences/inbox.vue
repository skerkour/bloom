<template>
  <v-container fluid>

    <v-row>
      <v-col cols="12" class="text-center" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
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

    <v-row v-if="project">
      <v-col cols="12" class="text-left">
        <h2 class="headline">Chatbox</h2>

        <b-chatbox-preferences
          :preferences="project.chatboxPreferences"
          @updated="onChatboxPreferencesUpdated"
        />
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import BChatboxPreferences from '@/ui/components/projects/chatbox_preferences.vue';
import { Project, ChatboxPreferences } from '@/api/graphql/model';


export default VueApp.extend({
  name: 'ProjectPreferencesInboxView',
  components: {
    BChatboxPreferences,
  },
  data() {
    return {
      loading: false,
      error: '',
      project: null as Project | null,
    };
  },
  computed: {
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
  },
  created() {
    this.fetchProject();
  },
  methods: {
    async fetchProject() {
      this.loading = true;
      this.error = '';

      try {
        this.project = await this.$projectsService.fetchProjectPreferences(this.projectFullPath);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    onChatboxPreferencesUpdated(preferences: ChatboxPreferences) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      this.project!.chatboxPreferences = preferences;
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
