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
      <v-col>
        <v-btn @click="cancel" depressed :loading="loading">
          {{ cancelLabel }}
        </v-btn>
      </v-col>

      <v-col class="text-right">
        <v-btn class="mr-3"
          @click="deleteBot"
          depressed
          color="error"
          :loading="loading"
          v-if="bot">
          Delete bot
        </v-btn>
        <v-btn
          @click="updateBot" depressed color="success" :loading="loading" :disabled="!canCreate"
          v-if="bot">
          <v-icon left>mdi-content-save</v-icon>
          Save changes
        </v-btn>
        <v-btn
          @click="createBot" depressed color="success" :loading="loading" :disabled="!canCreate"
          v-else>
          <v-icon left>mdi-plus</v-icon>
          Create bot
        </v-btn>
      </v-col>
    </v-row>


    <v-row>
     <v-col cols="12" sm="8">
        <v-text-field v-model="name" label="Name" />
      </v-col>

      <v-col cols="12" sm="8">
        <v-textarea v-model="description" label="Description" outlined/>
      </v-col>

      <v-col cols="12" sm="8">
        <v-switch
          v-model="active"
          inset
          :label="activeLabel"
        />
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import {
  Bot, CreateBotInput, DeleteBotInput, UpdateBotInput,
} from '@/api/graphql/model';
import { VueApp } from '@/app/vue';
import { PropType } from 'vue';

export default VueApp.extend({
  name: 'BBot',
  props: {
    bot: {
      type: Object as PropType<Bot | null>,
      required: false,
      default: null,
    },
  },
  data() {
    return {
      loading: false,
      error: '',
      updating: false,

      name: '',
      description: '',
      active: true,
    };
  },
  computed: {
    activeLabel(): string {
      return this.active ? 'On' : 'Off';
    },
    cancelLabel(): string {
      return this.bot ? 'Back' : 'Cancel';
    },
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
    canCreate(): boolean {
      return this.name.length !== 0;
    },
    editing(): boolean {
      return this.bot === null || this.updating;
    },
  },
  mounted() {
    this.resetFields(this.bot);
  },
  methods: {
    resetFields(bot: Bot | null) {
      if (bot) {
        this.name = bot.name;
        this.description = bot.description;
        this.active = bot.active;
      } else {
        this.name = '';
        this.description = '';
        this.active = true;
      }
      this.updating = false;
    },
    cancel() {
      this.$router.push({ path: `/${this.projectFullPath}/-/bots` });
    },
    async createBot() {
      this.loading = true;
      this.error = '';
      const input: CreateBotInput = {
        projectFullPath: this.projectFullPath,
        name: this.name,
        description: this.description,
        active: this.active,
      };

      try {
        await this.$botsService.createBot(input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async updateBot() {
      this.loading = true;
      this.error = '';

      const input: UpdateBotInput = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        botId: this.bot!.id,
        name: this.name,
        description: this.description,
      };

      try {
        const message = await this.$botsService.updateBot(input);
        this.$emit('updated', message);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async deleteBot() {
      // eslint-disable-next-line no-alert, no-restricted-globals
      if (!confirm('Do you really want to delete bot?')) {
        return;
      }

      this.loading = true;
      this.error = '';
      const input: DeleteBotInput = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        botId: this.bot!.id,
      };

      try {
        await this.$botsService.deleteBot(this.projectFullPath, input);
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
