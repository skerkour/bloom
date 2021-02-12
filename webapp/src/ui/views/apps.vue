<template>
  <v-container fluid>
    <v-row>
      <v-col cols="12" offset-md="3" md="6">
        <v-container>
          <v-row justify="center">
            <v-col cols="4" v-for="(app, index) in apps" :key="index"
              class="b-app-icon text-center bloom-pointer">
              <v-avatar class="b-pointer" @click="goto(app.url)" size="52">
                <img :src="app.icon" :alt="app.name" />
              </v-avatar>
              <p class="subtitle-1 font-weight-medium b-app-icon-name">{{ app.name }}</p>
            </v-col>
          </v-row>
        </v-container>
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { apps } from '@/domain/kernel/apps';

export default VueApp.extend({
  name: 'BAppsView',
  computed: {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    apps(): any[] {
      return apps;
    },
  },
  methods: {
    goto(path: string) {
      // eslint-disable-next-line @typescript-eslint/no-empty-function
      this.$router.push({ path }).catch(() => {});
    },
  },
});
</script>


<style lang="scss" scoped>
.b-app-icon {
  overflow-x: visible;
  white-space: nowrap;
}
.b-app-icon-name {
  margin-top: 10px;
}
.theme--dark.v-toolbar.v-sheet {
  background-color: #363636;
}
.v-card.v-sheet.theme--dark {
  background-color: #363636;
}
</style>
