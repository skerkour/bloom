<template>
  <v-container fluid>

    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center">
      <v-col cols="12">
        <h2>Lists</h2>
        <p>
          Lists allow you to segment your messages and give better control to your contacts
          to choose which messages they want to receive from you.
        </p>
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12" md="3" v-for="list in lists" :key="list.id">
        <v-card outlined @click="gotoList(list)"
          class="b-list-card mx-auto d-flex flex-column align-self-stretch">
          <v-card-title>
            <h3 class="text-h4">
              {{ list.name }}
            </h3>
          </v-card-title>
          <v-card-text>
            {{ list.description }}
          </v-card-text>
        </v-card>
      </v-col>

      <v-col cols="12" md="3">
        <v-card outlined @click="gotoNewList"
          class="b-list-card mx-auto d-flex flex-column align-self-stretch">
          <v-card-title>
            <h3 class="text-h4">
              New List
            </h3>
          </v-card-title>
          <v-card-text class="text-center">
            <v-icon x-large>mdi-plus</v-icon>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { List } from '@/domain/newsletter/model';

export default VueApp.extend({
  name: 'BListsView',
  data() {
    return {
      loading: false,
      error: '',
      lists: [] as List[],
    };
  },
  created() {
    this.fetchData();
  },
  methods: {
    gotoList(list: List) {
      this.$router.push({ path: `/newsletter/lists/${list.id}` });
    },
    gotoNewList() {
      this.$router.push({ path: '/newsletter/lists/new' });
    },
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.lists = await this.$newsletterService.fetchLists();
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
.b-list-card {
  height: 100%;
}
</style>
