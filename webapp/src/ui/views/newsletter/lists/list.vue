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

    <v-row v-if="list">
      <b-list :list="list.list" :contacts="list.contacts" :messages="list.messages"
        :acquisition="list.acquisition"
        @updated="onListUpdated" @imported="onImported" @removed="onContactRemoved" />
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Contact } from '@/api/graphql/model';
import { VueApp } from '@/app/vue';
import { List, ListWithDetails } from '@/domain/newsletter/model';
import BList from '@/ui/components/newsletter/list.vue';

export default VueApp.extend({
  name: 'BListView',
  components: {
    BList,
  },
  data() {
    return {
      loading: false,
      error: '',
      list: null as ListWithDetails | null,
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
        this.list = await this.$newsletterService.fetchList(this.$route.params.listId);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    onListUpdated(list: List) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      this.list!.list = list;
    },
    onImported() {
      this.fetchData();
    },
    onContactRemoved(contact: Contact) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      this.list!.contacts = this.list!.contacts.filter((c) => c.id !== contact.id);
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
