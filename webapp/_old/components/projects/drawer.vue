<template>
  <v-navigation-drawer app clipped v-model="drawer"
    :mobile-breakpoint="this.$vuetify.breakpoint.thresholds.sm">
    <v-list nav dense class="text-left">

      <v-list-item exact link :to="`${projectPath}/-/inbox`">
        <v-list-item-icon>
          <v-icon>mdi-inbox</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Inbox</v-list-item-title>
        </v-list-item-content>
      </v-list-item>


      <v-list-group prepend-icon="mdi-robot">
        <template v-slot:activator>
          <v-list-item-title>Bots</v-list-item-title>
        </template>

        <v-list-item exact link :to="`${projectPath}/-/bots`">
          <v-list-item-content>
            <v-list-item-title>Bots</v-list-item-title>
          </v-list-item-content>
        </v-list-item>

        <v-list-item exact link :to="`${projectPath}/-/bots/history`">
          <v-list-item-content>
            <v-list-item-title>History</v-list-item-title>
          </v-list-item-content>
        </v-list-item>

        <v-list-item exact link :to="`${projectPath}/-/bots/connections`">
          <v-list-item-content>
            <v-list-item-title>Connections</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list-group>


      <v-list-group prepend-icon="mdi-folder-outline">
        <template v-slot:activator>
          <v-list-item-title>Files</v-list-item-title>
        </template>

        <v-list-item exact link :to="`${projectPath}/-/files`">
          <v-list-item-content>
            <v-list-item-title>Files</v-list-item-title>
          </v-list-item-content>
        </v-list-item>

        <v-list-item exact link :to="`${projectPath}/-/files/trash`">
          <v-list-item-content>
            <v-list-item-title>Trash</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list-group>

      <v-list-item exact link :to="`${projectPath}/-/milestones`">
        <v-list-item-icon>
          <v-icon>mdi-format-list-checks</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>To dos</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-list-item exact link :to="`${projectPath}/-/tickets`">
        <v-list-item-icon>
          <v-icon>mdi-cards-outline</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Tickets</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-list-item exact link :to="`${projectPath}/-/contacts`">
        <v-list-item-icon>
          <v-icon>mdi-contacts-outline</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Contacts</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-list-item exact link :to="`${projectPath}/-/analytics`">
        <v-list-item-icon>
          <v-icon>mdi-chart-line</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Analytics</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-list-item exact link :to="`${projectPath}/-/outbound`">
        <v-list-item-icon>
          <v-icon>mdi-send-check-outline</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Outbound</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-list-item exact link :to="`${projectPath}/-/lists`">
        <v-list-item-icon>
          <v-icon>mdi-format-list-bulleted</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Lists</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-list-item exact link :to="`${projectPath}/-/monitors`">
        <v-list-item-icon>
          <v-icon>mdi-heart-pulse</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Monitors</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-list-item exact link :to="`${projectPath}/-/labels`">
        <v-list-item-icon>
          <v-icon>mdi-label-multiple-outline</v-icon>
        </v-list-item-icon>
        <v-list-item-content>
          <v-list-item-title>Labels</v-list-item-title>
        </v-list-item-content>
      </v-list-item>

      <v-list-group prepend-icon="mdi-cog-outline">
        <template v-slot:activator>
          <v-list-item-title>Preferences</v-list-item-title>
        </template>

        <v-list-item exact link :to="`${projectPath}/-/preferences`">
          <v-list-item-content>
            <v-list-item-title>General</v-list-item-title>
          </v-list-item-content>
        </v-list-item>

        <v-list-item exact link :to="`${projectPath}/-/preferences/inbox`">
          <v-list-item-content>
            <v-list-item-title>Inbox</v-list-item-title>
          </v-list-item-content>
        </v-list-item>

      </v-list-group>

    </v-list>
  </v-navigation-drawer>
</template>


<script lang="ts">
import { Mutation } from '@/app/store';
import { VueApp } from '@/app/vue';


export default VueApp.extend({
  name: 'BProjectDrawer',
  computed: {
    projectPath(): string {
      const { projectPath, namespacePath } = this.$route.params;
      return `/${namespacePath}/${projectPath}`;
    },
    drawer: {
      get(): boolean {
        return this.$store.state.drawer;
      },
      set(value: boolean) {
        this.$store.commit(Mutation.SET_DRAWER, value);
      },
    },
  },
  methods: {
    goto(path: string) {
      this.$router.push({ path });
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
