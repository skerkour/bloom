<template>
  <v-data-table
    :headers="headers"
    :items="labels"
    item-key="id"
    :items-per-page="50"
    :loading="loading"
  >
  <template v-slot:no-data>
      <p class="text-center">
        No label yet.
      </p>
    </template>

    <template v-slot:item="{ item }" class="text-left">
      <tr @click="gotoLabel(item)" class="bloom-pointer">
        <td>
          <b-label :label="item" />
        </td>
        <td>
          {{ item.description }}
        </td>
        <td>
          <!-- <v-btn icon :to="`/${projectFullPath}/-/labels/${item.id}`">
            <v-icon>mdi-pencil</v-icon>
          </v-btn> -->

          <v-menu bottom left>
            <template v-slot:activator="{ on }">
              <v-btn icon v-on="on">
                <v-icon>mdi-dots-vertical</v-icon>
              </v-btn>
            </template>

            <v-list>
              <v-list-item @click="deleteLabel(item)">
                <v-list-item-icon>
                  <v-icon>mdi-delete</v-icon>
                </v-list-item-icon>
                <v-list-item-title>Delete label</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-menu>
        </td>
      </tr>
    </template>

  </v-data-table>
</template>


<script lang="ts">
import { PropType } from 'vue';
import { VueApp } from '@/app/vue';
import { Label } from '@/api/graphql/model';
import BLabel from '@/ui/components/collaboration/label.vue';


export default VueApp.extend({
  name: 'BLabelsList',
  components: {
    BLabel,
  },
  props: {
    labels: {
      type: Array as PropType<Label[]>,
      default: [],
    },
    projectFullPath: {
      type: String,
      default: '',
    },
    loading: {
      type: Boolean as PropType<boolean>,
      default: false,
      required: false,
    },
  },
  data() {
    return {
      headers: [
        {
          text: 'Name',
          align: 'start',
          sortable: true,
          value: 'name',
        },
        {
          text: 'Description',
          align: 'start',
          sortable: true,
          value: 'description',
          size: '70%',
        },
        {
          text: 'Actions',
          align: 'start',
          sortable: false,
        },
      ],
    };
  },
  methods: {
    deleteLabel(label: Label) {
      this.$emit('delete', label);
    },
    gotoLabel(label: Label) {
      this.$router.push({ path: `/${this.projectFullPath}/-/labels/${label.id}` });
    },
  },
});
</script>


<style lang="scss" scoped>
</style>
