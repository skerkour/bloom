<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center" v-if="connection">
      <b-ssh-connection :connection="connection" @updated="onConnectionUpdated"
        v-if="connection.app.id === 'ssh'" />
      <b-telegram-connection :connection="connection" @updated="onConnectionUpdated"
        v-if="connection.app.id === 'telegram'" />
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { BotConnection } from '@/api/graphql/model';
import { VueApp } from '@/app/vue';
import BSshConnection from '@/ui/components/bots/apps/ssh.vue';
import BTelegramConnection from '@/ui/components/bots/apps/telegram.vue';

export default VueApp.extend({
  name: 'BBotsConnectionView',
  components: {
    BSshConnection,
    BTelegramConnection,
  },
  data() {
    return {
      loading: false,
      error: '',
      connection: null as BotConnection | null,
    };
  },
  computed: {
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
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
        const res = await this.$botsService.fetchBotConnection(this.$route.params.connectionId);
        this.connection = res;
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    onConnectionUpdated(connection: BotConnection) {
      this.connection = connection;
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
