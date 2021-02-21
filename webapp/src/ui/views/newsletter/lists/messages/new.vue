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

    <v-row v-if="loaded">
      <b-newsletter-message :list="listId" />
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { List } from '@/domain/newsletter/model';
import BNewsletterMessage from '@/ui/components/newsletter/message.vue';

export default VueApp.extend({
  name: 'BNewOutboundMessageView',
  components: {
    BNewsletterMessage,
  },
  data() {
    return {
      loading: false,
      loaded: false,
      error: '',
      lists: [] as List[],
    };
  },
  computed: {
    listId(): string {
      return this.$route.params.listId;
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
        this.lists = await this.$newsletterService.fetchLists();
        this.loaded = true;
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
