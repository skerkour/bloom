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

    <v-row justify="center" v-if="group">
      <b-billing :group="group" />
    </v-row>

  </v-container>
</template>


<script lang="ts">
/* eslint-disable max-len, @typescript-eslint/no-non-null-assertion */
import { VueApp } from '@/app/vue';
import BBilling from '@/ui/components/kernel/billing.vue';
import {
  Group,
} from '@/domain/kernel/model';


export default VueApp.extend({
  name: 'BGroupBilling',
  components: {
    BBilling,
  },
  data() {
    return {
      loading: false,
      error: '',
      group: null as Group | null,
    };
  },
  computed: {
    groupPath(): string {
      return this.$route.params.groupPath;
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
        this.group = await this.$kernelService.fetchGroup(this.groupPath);
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
.v-application p {
  margin-bottom: 0;
}
</style>
