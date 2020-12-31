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
      <v-col cols="12" xl="8">
        <v-card outlined v-if="editing">
            <v-card-title class="headline">
              <v-text-field label="Title" v-model="title" />
            </v-card-title>
            <v-card-text>
              <b-markdown-editor v-model="description" placeholder="Description..." outlined />
            </v-card-text>

          <v-card-actions>
            <v-btn depressed :loading="loading" @click="cancel" v-if="showCancelButton">
              Cancel
            </v-btn>

            <v-spacer />

            <v-btn color="success" v-if="milestone" :loading="loading"
              depressed :disabled="!canCreate" @click="updateMilestone">
              Save changes
            </v-btn>
            <v-btn color="success" v-else :loading="loading"
              :disabled="!canCreate" depressed @click="createMilestone">
              Create milestone
            </v-btn>
          </v-card-actions>
         </v-card>

         <v-card outlined v-else>
          <v-card-title class="headline">
            {{ title }}
          </v-card-title>

          <v-card-text>
            <p v-html="descriptionHtml" />
          </v-card-text>

          <v-card-actions>
            <v-btn depressed :loading="loading" @click="cancel" v-if="showCancelButton">
              Cancel
            </v-btn>

            <v-spacer />

            <v-btn class="mr-3"
              @click="deleteMilestone"
              depressed
              color="error"
              :loading="loading"
              v-if="milestone">
              Delete milestone
            </v-btn>
            <v-btn outlined :loading="loading" color="primary"
              @click="reopenMilestone" v-if="milestone.closedAt">
              Reopen milestone
            </v-btn>
            <v-btn outlined :loading="loading" color="warning"
              @click="closeMilestone" v-else>
              Close milestone
            </v-btn>

            <v-btn @click="updateClicked" color="primary" depressed>
              <v-icon>mdi-pencil</v-icon>
              Edit
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>

    <v-row v-if="milestone">
      <v-col cols="12" xl="8">
         <v-progress-linear
          height="25"
          :value="progress"
          color="success"
          >
          <strong class="blue-grey--text text--darken-3">{{ progress }}%</strong>
        </v-progress-linear>
      </v-col>
    </v-row>

    <v-row v-if="milestone">
      <v-col cols="12" xl="8">
        <v-list two-line nav>
          <v-subheader>To do</v-subheader>
          <v-list-item
            v-for="ticket in open"
            :key="ticket.id"
            :to="`/${projectFullPath}/-/tickets/${ticket.id}`"
          >
            <v-list-item-content>
              <v-list-item-title v-text="ticket.title"></v-list-item-title>
              <v-list-item-subtitle >
                opened {{ date(ticket.createdAt) }}
          by <router-link :to="`/${ticket.author.username}`">{{ ticket.author.name }}</router-link>
              </v-list-item-subtitle>
            </v-list-item-content>

            <v-list-item-action>
              <v-tooltip bottom>
                <template v-slot:activator="{ on, attrs }">
                  <v-icon  color="success" v-bind="attrs" v-on="on">
                    mdi-alert-circle-outline
                  </v-icon>
                </template>
                <span>Open</span>
              </v-tooltip>
            </v-list-item-action>
          </v-list-item>
        </v-list>
      </v-col>

      <v-col cols="12" xl="8" class="mt-4">
        <v-list two-line nav>
          <v-subheader>Completed</v-subheader>
          <v-list-item
            v-for="ticket in closed"
            :key="ticket.id"
            :to="`/${projectFullPath}/-/tickets/${ticket.id}`"
          >
            <v-list-item-content>
              <v-list-item-title><s>{{ ticket.title }} </s></v-list-item-title>
              <v-list-item-subtitle >
                completed on {{ date(ticket.closedAt) }}
          by <router-link :to="`/${ticket.author.username}`">{{ ticket.author.name }}</router-link>
              </v-list-item-subtitle>
            </v-list-item-content>

            <v-list-item-action>
              <v-tooltip bottom>
                <template v-slot:activator="{ on, attrs }">
                  <v-icon  color="error" v-bind="attrs" v-on="on">
                    mdi-alert-circle-check-outline
                  </v-icon>
                </template>
                <span>Completed</span>
              </v-tooltip>
            </v-list-item-action>
          </v-list-item>
        </v-list>
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { PropType } from 'vue';
import {
  CloseMilestoneInput,
  CreateMilestoneInput,
  DeleteMilestoneInput,
  Milestone,
  ReopenMilestoneInput,
  Ticket,
  UpdateMilestoneInput,
} from '@/api/graphql/model';
import { VueApp } from '@/app/vue';
import BMarkdownEditor from '@/ui/components/kernel/markdown_editor.vue';
import date from '@/app/filters/date';


export default VueApp.extend({
  name: 'BMilestone',
  components: {
    BMarkdownEditor,
  },
  props: {
    milestone: {
      type: Object as PropType<Milestone | null>,
      required: false,
      default: null,
    },
  },
  data() {
    return {
      loading: false,
      error: '',

      title: '',
      description: '',
      descriptionHtml: '',
      updating: false,
    };
  },
  computed: {
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
    canCreate(): boolean {
      return this.title.length !== 0;
    },
    editing(): boolean {
      return this.milestone === null || this.updating;
    },
    open(): Ticket[] {
      return this.milestone
        ? this.milestone.tickets.filter((ticket: Ticket) => !ticket.closedAt)
        : [];
    },
    closed(): Ticket[] {
      return this.milestone
        ? this.milestone.tickets.filter((ticket: Ticket) => ticket.closedAt)
        : [];
    },
    progress(): number {
      return Math.round((this.closed.length / (this.closed.length + this.open.length)) * 100) || 0;
    },
    showCancelButton(): boolean {
      return !this.milestone || (this.milestone && this.updating);
    },
  },
  watch: {
    milestone(milestone: Milestone) {
      this.clearFields(milestone);
    },
  },
  mounted() {
    this.clearFields(this.milestone);
  },
  methods: {
    date,
    cancel() {
      this.clearFields(this.milestone);
      if (!this.milestone || (this.milestone && this.updating)) {
        this.$router.push({ path: `/${this.projectFullPath}/-/milestones` });
      }
    },
    clearFields(milestone: Milestone | null) {
      if (milestone) {
        this.title = milestone.title;
        this.description = milestone.description;
        this.descriptionHtml = milestone.descriptionHtml;
      } else {
        this.title = '';
        this.description = '';
        this.descriptionHtml = '';
      }
      this.updating = false;
    },
    updateClicked() {
      this.updating = true;
    },
    async closeMilestone() {
      this.loading = true;
      this.error = '';
      const input: CloseMilestoneInput = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        id: this.milestone!.id,
      };

      try {
        const updatedMilestone = await this.$collaborationService.closeMilestone(input);
        this.updating = false;
        this.$emit('updated', updatedMilestone);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async reopenMilestone() {
      this.loading = true;
      this.error = '';
      const input: ReopenMilestoneInput = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        id: this.milestone!.id,
      };

      try {
        const updatedMilestone = await this.$collaborationService.reopenMilestone(input);
        this.updating = false;
        this.$emit('updated', updatedMilestone);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async createMilestone(): Promise<void> {
      this.loading = true;
      this.error = '';
      const input: CreateMilestoneInput = {
        projectFullPath: this.projectFullPath,
        title: this.title,
        description: this.description,
      };

      try {
        await this.$collaborationService.createMilestone(input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async updateMilestone(): Promise<void> {
      this.loading = true;
      this.error = '';
      const input: UpdateMilestoneInput = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        id: this.milestone!.id,
        title: this.title,
        description: this.description,
      };

      try {
        const updatedMilestone = await this.$collaborationService.updateMilestone(input);
        this.updating = false;
        this.$emit('updated', updatedMilestone);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async deleteMilestone(): Promise<void> {
      // eslint-disable-next-line no-alert, no-restricted-globals
      if (!confirm('Do you really want to delete milestone?')) {
        return;
      }

      this.loading = true;
      this.error = '';
      const input: DeleteMilestoneInput = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        id: this.milestone!.id,
      };

      try {
        await this.$collaborationService.deleteMilestone(input);
        this.$router.push({ path: `/${this.projectFullPath}/-/milestones` });
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
