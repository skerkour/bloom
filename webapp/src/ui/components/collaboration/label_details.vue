<template>
  <v-container fluid>

    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row>
      <v-text-field v-model="name" label="Name" :disabled="loading" />
    </v-row>

    <v-row>
      <v-text-field v-model="description" label="Descirption" :disabled="loading" />
    </v-row>

    <v-row>
      <v-color-picker
        v-model="backgroundColor"
        hide-mode-switch
        mode="hexa"
        :disabled="loading"
      />
    </v-row>


    <v-row>
      <v-btn :to="`/${projectFullPath}/-/labels`" text :loading="loading">
        Cancel
      </v-btn>

      <v-spacer />


      <v-btn color="success" v-if="label" :loading="loading"
        depressed :disabled="!canCreate" @click="updateLabel">
        Save changes
      </v-btn>
      <v-btn color="success" v-else :loading="loading"
        :disabled="!canCreate" depressed @click="createLabel">
        Create label
      </v-btn>

    </v-row>
  </v-container>
</template>


<script lang="ts">
import { PropType } from 'vue';
import { VueApp } from '@/app/vue';
import { Label, CreateLabelInput, UpdateLabelInput } from '@/api/graphql/model';

export default VueApp.extend({
  name: 'BLabelDetails',
  props: {
    label: {
      type: Object as PropType<Label>,
      default: null,
      required: false,
    },
    projectFullPath: {
      type: String,
      required: true,
    },
  },
  data() {
    return {
      error: '',
      loading: false,
      name: '',
      description: '',
      backgroundColor: '#ff0000',
    };
  },
  computed: {
    canCreate(): boolean {
      return this.name.length !== 0;
    },
  },
  created() {
    if (this.label) {
      this.name = this.label.name;
      this.description = this.label.description;
      this.backgroundColor = this.label.backgroundColor;
    }
  },
  methods: {
    async createLabel(): Promise<void> {
      this.loading = true;
      this.error = '';
      const input: CreateLabelInput = {
        projectFullPath: this.projectFullPath,
        name: this.name,
        description: this.description,
        backgroundColor: this.backgroundColor,
      };

      try {
        await this.$collaborationService.createLabel(input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async updateLabel(): Promise<void> {
      this.loading = true;
      this.error = '';
      const input: UpdateLabelInput = {
        labelId: this.label.id,
        name: this.name,
        description: this.description,
        backgroundColor: this.backgroundColor,
      };

      try {
        await this.$collaborationService.updateLabel(this.projectFullPath, input);
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
