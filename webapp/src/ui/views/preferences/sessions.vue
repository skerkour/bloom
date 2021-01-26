<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" md="8" lg="8" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row class="mx-4" justify="center">
      <v-col cols="12" md="8" xl="4">
        <b-sessions-list
          :sessions="sessions" :currentSession="currentSession" :loading="loading"
          @revoke="revokeSession" />
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { Session } from '@/domain/kernel/model';
import { VueApp } from '@/app/vue';
import BSessionsList from '@/ui/components/kernel/sessions_list.vue';

export default VueApp.extend({
  name: 'MSessionsView',
  components: {
    BSessionsList,
  },
  data() {
    return {
      sessions: [] as Session[],
      loading: false,
      error: '',
    };
  },
  computed: {
    currentSession(): Session {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      return this.$store.state.session!;
    },
  },
  mounted() {
    this.fetchData();
  },
  methods: {
    async fetchData(): Promise<void> {
      this.loading = true;
      this.error = '';

      try {
        this.sessions = await this.$kernelService.fetchMySessions();
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async revokeSession(session: Session) {
      if (session.id === this.currentSession.id) {
        // eslint-disable-next-line no-alert, no-restricted-globals
        if (!confirm('Do you really want to sign out?')) {
          return;
        }
      }

      this.error = '';
      this.loading = true;

      try {
        await this.$kernelService.revokeSession(session.id);
        this.sessions = this.sessions.filter((s: Session) => s.id !== session.id);
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
