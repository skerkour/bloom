<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''" dismissible>
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>


    <v-row>
      <v-col  cols="12" sm="8" v-if="!connection">
        <h2 class="text-h4">New Telegram Connection</h2>
      </v-col>

     <v-col cols="12" sm="8">
        <v-text-field v-model="name" label="Connection name" />
      </v-col>

      <v-col cols="12" sm="8">
        <v-textarea v-model="description" label="Description" />
      </v-col>

      <v-col cols="12" sm="8" v-if="!connection">
        <v-text-field v-model="token" label="Token" outlined />
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12" sm="8">
        <v-btn depressed :loading="loading" :to="`/${projectFullPath}/-/bots/connections`"
          class="mr-5">
          {{ cancelLabel }}
        </v-btn>

        <v-btn class="mr-3 ml-5"
          @click="deleteConnection"
          depressed
          color="error"
          :loading="loading"
          v-if="connection">
          <v-icon left>mdi-delete</v-icon>
          Delete connection
        </v-btn>
        <v-btn @click="updateConnection" depressed color="success" :loading="loading"
          :disabled="!canUpdate" v-if="connection">
          <v-icon left>mdi-content-save</v-icon>
          Save changes
        </v-btn>

        <v-btn @click="createConnection" depressed color="success" :loading="loading"
          :disabled="!canCreate" v-else>
          <v-icon left>mdi-plus</v-icon>
          Create connection
        </v-btn>
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import {
  BotConnection, CreateBotConnectionInput, DeleteBotConnectionInput, UpdateBotConnectionInput,
} from '@/api/graphql/model';
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';

export default VueApp.extend({
  name: 'BTelegramConnection',
  props: {
    connection: {
      type: Object as PropType<BotConnection | null>,
      required: false,
      default: null,
    },
  },
  data() {
    return {
      error: '',
      loading: false,

      name: '',
      description: '',
      token: '',
    };
  },
  computed: {
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
    canCreate(): boolean {
      return this.name.length !== 0 && this.token.length !== 0;
    },
    canUpdate(): boolean {
      return this.name.length !== 0;
    },
    cancelLabel(): string {
      return this.connection ? 'Back' : 'Cancel';
    },
  },
  mounted() {
    this.resetFields(this.connection);
  },
  methods: {
    resetFields(connection: BotConnection | null) {
      if (connection) {
        this.name = connection.name;
        this.description = connection.description;
      } else {
        this.name = '';
        this.description = '';
        this.token = '';
      }
    },
    async createConnection() {
      this.loading = true;
      this.error = '';
      const input: CreateBotConnectionInput = {
        projectFullPath: this.projectFullPath,
        name: this.name,
        description: this.description,
        app: 'telegram',
        credentials: {
          token: this.token,
        },
      };

      try {
        await this.$botsService.createConnection(input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async updateConnection() {
      this.loading = true;
      this.error = '';

      const input: UpdateBotConnectionInput = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        connectionId: this.connection!.id,
        name: this.name,
        description: this.description,
      };

      try {
        const connection = await this.$botsService.updateConnection(input);
        this.$emit('updated', connection);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async deleteConnection() {
      // eslint-disable-next-line no-alert, no-restricted-globals
      if (!confirm('Do you really want to delete this Telegram connection?')) {
        return;
      }

      this.loading = true;
      this.error = '';
      const input: DeleteBotConnectionInput = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        connectionId: this.connection!.id,
      };

      try {
        await this.$botsService.deleteConnection(this.projectFullPath, input);
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
