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
      <v-col cols="12" md="8" lg="6" xl="4">
        <v-card elevation="0">
          <v-card-title>
            <p class="display-1 text--primary">
              New Group
            </p>
          </v-card-title>

          <v-card-text>
            <v-row>
              <v-col cols="12">
                <v-text-field
                  label="Group name" v-model="name" :disabled="loading"
                  @input="nameToNamespace"
                />
              </v-col>
              <v-col cols="6">
                <v-text-field
                  prefix="https://bloom.sh"
                  disabled
                ></v-text-field>
              </v-col>
              <v-col cols="6">
                <v-text-field
                  label="Group url"
                  v-model="path"
                  @keyup="lowercasePath"
                  :disabled="loading"
                ></v-text-field>
              </v-col>
              <v-col cols="12">
                <v-textarea
                  label="Description"
                  v-model="description"
                  :disabled="loading"
                ></v-textarea>
              </v-col>
            </v-row>
          </v-card-text>

          <v-card-actions>
            <v-btn text to="/">
              Cancel
            </v-btn>
            <v-spacer />
            <v-btn color="success" @click="createGroup" :loading="loading" depressed>
              Create group
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { VueApp } from '@/app/vue';
import { CreateGroupInput } from '../../../api/graphql/model';


export default VueApp.extend({
  name: 'BNewGroupView',
  data() {
    return {
      loading: false,
      name: '',
      error: '',
      path: '',
      description: '',
    };
  },
  methods: {
    lowercasePath() {
      this.path = this.path.toLowerCase();
    },
    nameToNamespace() {
      this.path = this.name.toLowerCase()
        .replaceAll(' ', '-')
        .replaceAll('.', '-')
        .replaceAll('_', '-');
    },
    async createGroup() {
      this.loading = true;
      this.error = '';
      const input: CreateGroupInput = {
        name: this.name,
        path: this.path,
        description: this.description,
      };

      try {
        await this.$groupsService.createGroup(input);
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
