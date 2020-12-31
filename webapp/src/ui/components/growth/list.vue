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
          Cancel
        </v-btn>
      </v-col>

      <v-col class="text-right">
        <v-btn class="mr-3"
          @click="deleteList"
          depressed
          color="error"
          :loading="loading"
          v-if="list">
          Delete list
        </v-btn>
        <v-btn
          @click="updateList" depressed color="success" :loading="loading" :disabled="!canCreate"
          v-if="list" >
          Save changes
        </v-btn>
        <v-btn
          @click="createList" depressed color="success" :loading="loading" :disabled="!canCreate"
          v-else >
          Create list
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
    </v-row>

    <v-row v-if="list">
      <v-col cols="12">
        <v-data-table
          :headers="contactsHeaders"
          :items="list.contacts"
          item-key="id"
          hide-default-footer
          :loading="loading"
        >
          <template v-slot:no-data>
            <p class="text-center">
              No contact.
            </p>
          </template>

          <template v-slot:item="{ item }" class="text-left">
            <tr @click="gotoContact(item)" class="bloom-pointer">
              <td>
                {{ item.name }}
              </td>
            </tr>
          </template>
        </v-data-table>
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { PropType } from 'vue';
import {
  List, Contact, CreateListInput, UpdateListInput, DeleteListInput,
} from '@/api/graphql/model';
import { VueApp } from '@/app/vue';

export default VueApp.extend({
  name: 'BList',
  props: {
    list: {
      type: Object as PropType<List | null>,
      required: false,
      default: null,
    },
  },
  data() {
    return {
      loading: false,
      error: '',

      name: '',
      description: '',
      contacts: [] as Contact[],
      contactsHeaders: [
        {
          text: 'Contact',
          align: 'start',
          sortable: true,
          value: 'name',
        },
      ],
    };
  },
  computed: {
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
    canCreate(): boolean {
      return this.name.length !== 0;
    },
  },
  mounted() {
    this.clearFields();
  },
  methods: {
    cancel() {
      this.$router.push({ path: `/${this.projectFullPath}/-/lists` });
    },
    clearFields() {
      if (this.list) {
        this.name = this.list.name;
        this.description = this.list.description;
        this.contacts = this.list.contacts;
      } else {
        this.name = '';
        this.description = '';
        this.contacts = [];
      }
    },
    async createList() {
      this.loading = true;
      this.error = '';
      const input: CreateListInput = {
        projectFullPath: this.projectFullPath,
        name: this.name,
        description: this.description,
      };

      try {
        await this.$growthService.createList(input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async updateList() {
      this.loading = true;
      this.error = '';
      const input: UpdateListInput = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        id: this.list!.id,
        name: this.name,
        description: this.description,
      };

      try {
        const message = await this.$growthService.updateList(input);
        this.$emit('updated', message);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async deleteList() {
      // eslint-disable-next-line no-alert, no-restricted-globals
      if (!confirm('Do you really want to delete list?')) {
        return;
      }

      this.loading = true;
      this.error = '';
      const input: DeleteListInput = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        id: this.list!.id,
      };

      try {
        await this.$growthService.deleteList(this.projectFullPath, input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    gotoContact(contact: Contact) {
      this.$router.push({ path: `/${this.projectFullPath}/-/contacts/${contact.id}` });
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
