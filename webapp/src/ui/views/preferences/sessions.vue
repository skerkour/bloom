<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center" class="text-center" v-if="loading && !user">
      <v-col cols="12" sm="10" md="8" xl="6">
        <v-progress-circular
          :size="50"
          color="primary"
          indeterminate
        />
      </v-col>
    </v-row>

    <v-row v-if="user" class="mx-4">
      <v-col cols="12" md="6" xl="4">
        <b-sessions-list
          :sessions="sessions" :currentSession="currentSession" :loading="loading"
          @revoke="revokeSession" />
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { Session, User } from '@/api/graphql/model';
import { VueApp } from '@/app/vue';
import BSessionsList from '@/ui/components/kernel/sessions_list.vue';

export default VueApp.extend({
  name: 'MSessionsView',
  components: {
    BSessionsList,
  },
  data() {
    return {
      user: null as User | null,
      loading: false,
      error: '',
    };
  },
  computed: {
    sessions(): Session[] {
      return this.user ? this.user.sessions : [];
    },
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
        this.user = await this.$usersService.fetchMySessions();
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
        await this.$usersService.revokeSession(session.id);
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        this.user!.sessions = this.user!.sessions.filter((s: Session) => s.id !== session.id);
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
