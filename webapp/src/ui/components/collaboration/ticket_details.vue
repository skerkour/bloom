<template>
  <v-container fluid>

    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-app-bar elevation="0" fixed color="white" v-if="ticket"
      :class="{'blm-sticky-ticket-toolbar': true, 'blm-toolbar-left': toolbarLeft}">
      <v-toolbar-title>{{ ticket.title }}</v-toolbar-title>
      <v-chip color="error" class="ml-4" v-if="ticket.closedAt">
        Closed
        <v-icon right>mdi-alert-circle-check-outline</v-icon>
      </v-chip>
      <v-chip color="success" class="ml-4" v-else>
        Open
        <v-icon right>mdi-alert-circle-outline</v-icon>
      </v-chip>

      <v-spacer />


      <v-btn outlined :loading="loading" color="primary"
        @click="reopenTicket" v-if="ticket.closedAt">
        Reopen ticket
      </v-btn>
      <v-btn outlined :loading="loading" color="warning"
        @click="closeTicket" v-else>
        Close ticket
      </v-btn>


      <v-menu bottom>
        <template v-slot:activator="{ on }">
          <v-btn icon v-on="on">
            <v-icon>mdi-dots-vertical</v-icon>
          </v-btn>
        </template>

        <v-list>
          <v-list-item @click="deleteTicket">
            <v-list-item-icon>
              <v-icon>mdi-delete</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Delete Ticket</v-list-item-title>
          </v-list-item>
        </v-list>
      </v-menu>
    </v-app-bar>

    <div class="mt-5 pt-5" v-if="ticket">
    </div>

    <v-row>
      <v-col cols="12">
        <v-card outlined v-if="editing">

          <v-card-title class="headline">
            <v-text-field label="Title" v-model="title" />
          </v-card-title>

          <v-card-text>
          <b-markdown-editor v-model="body" placeholder="Leave a comment..." outlined />

            <b-select-labels v-model="labels" :items="projectLabels" />
            <b-select-milestones v-model="milestones" :items="projectMilestones" />
          </v-card-text>

          <v-card-actions>
            <v-btn depressed :loading="loading" @click="cancel">
              Cancel
            </v-btn>

            <v-spacer />

            <v-btn color="success" v-if="ticket" :loading="loading"
              depressed :disabled="!canCreate" @click="updateTicket">
              <v-icon left>mdi-content-save</v-icon>
              Save changes
            </v-btn>
            <v-btn color="success" v-else :loading="loading"
              :disabled="!canCreate" depressed @click="createTicket">
              <v-icon left>mdi-plus</v-icon>
              Create ticket
            </v-btn>
          </v-card-actions>
         </v-card>


         <v-card outlined v-else>
          <v-card-text>
            <p v-html="bodyHtml" />
            <b-select-labels v-model="labels" :items="projectLabels" readonly />
            <b-select-milestones v-model="milestones" :items="projectMilestones" readonly />
          </v-card-text>

          <v-card-actions>
            <v-spacer />

            <v-btn depressed @click="updateClicked" color="primary" class="ml-3">
              <v-icon left>mdi-pencil</v-icon>
              Edit
            </v-btn>
          </v-card-actions>

        </v-card>
      </v-col>
    </v-row>

  </v-container>
</template>


<script lang="ts">
import { PropType } from 'vue';
import { VueApp } from '@/app/vue';
import {
  CreateTicketInput,
  Ticket,
  Label,
  UpdateTicketInput, Milestone, DeleteTicketInput,
} from '@/api/graphql/model';
import BSelectLabels from '@/ui/components/collaboration/select_labels.vue';
import BMarkdownEditor from '@/ui/components/kernel/markdown_editor.vue';
import BSelectMilestones from '@/ui/components/collaboration/select_milestones.vue';


export default VueApp.extend({
  name: 'BTicketDetails',
  components: {
    BSelectLabels,
    BMarkdownEditor,
    BSelectMilestones,
  },
  props: {
    ticket: {
      type: Object as PropType<Ticket>,
      default: null,
      required: false,
    },
    projectLabels: {
      type: Array as PropType<Label[]>,
      required: true,
    },
    projectMilestones: {
      type: Array as PropType<Milestone[]>,
      required: true,
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
      title: '',
      body: '',
      bodyHtml: '',
      labels: [] as Label[],
      milestones: [] as Milestone[],
      updating: false,
    };
  },
  computed: {
    canCreate(): boolean {
      return this.title.length !== 0;
    },
    editing(): boolean {
      return this.ticket === null || this.updating;
    },
    // we need to do this ugly hack because if we doo <v-app-bar app ... />
    // it will bug and then when we change page, the content of the page will overflow toward top
    toolbarLeft(): boolean {
      return this.$store.state.drawer && this.$vuetify.breakpoint.mdAndUp;
    },
  },
  watch: {
    ticket(newTicket: Ticket) {
      this.resetFields(newTicket);
    },
  },
  created() {
    this.resetFields(this.ticket);
  },
  methods: {
    cancel() {
      this.resetFields(this.ticket);
      if (!this.ticket) {
        this.$router.push({ path: `/${this.projectFullPath}/-/tickets` });
      }
    },
    resetFields(ticket?: Ticket) {
      if (ticket) {
        this.title = ticket.title;
        this.body = ticket.body;
        this.labels = ticket.labels;
        this.updating = false;
        this.bodyHtml = ticket.bodyHtml;
        this.milestones = ticket.milestones;
      } else {
        this.title = '';
        this.body = '';
        this.labels = [];
        this.updating = false;
        this.bodyHtml = '';
        this.milestones = [];
      }
    },
    updateClicked() {
      this.updating = true;
    },
    closeTicket() {
      this.cancel();
      this.$emit('close', this.ticket);
    },
    reopenTicket() {
      this.$emit('reopen', this.ticket);
    },
    async createTicket(): Promise<void> {
      this.loading = true;
      this.error = '';
      const labelIds = this.labels.map((label: Label) => label.id);
      const milestonesIds = this.milestones.map((milestone: Milestone) => milestone.id);
      const input: CreateTicketInput = {
        projectFullPath: this.projectFullPath,
        title: this.title,
        body: this.body,
        labels: labelIds,
        milestones: milestonesIds,
      };

      try {
        await this.$collaborationService.createTicket(input);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async updateTicket(): Promise<void> {
      this.loading = true;
      this.error = '';
      const labelIds = this.labels.map((label: Label) => label.id);
      const milestonesIds = this.milestones.map((milestone: Milestone) => milestone.id);
      const input: UpdateTicketInput = {
        ticketId: this.ticket.id,
        title: this.title,
        body: this.body,
        labels: labelIds,
        milestones: milestonesIds,
      };

      try {
        const updatedTicket = await this.$collaborationService.updateTicket(input);
        this.updating = false;
        this.$emit('updated', updatedTicket);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async deleteTicket(): Promise<void> {
      this.cancel();
      // eslint-disable-next-line no-alert, no-restricted-globals
      if (!confirm('Do you really want to delete ticket?')) {
        return;
      }

      this.loading = true;
      this.error = '';
      const input: DeleteTicketInput = {
        id: this.ticket.id,
      };

      try {
        await this.$collaborationService.deleteTicket(input);
        this.$router.push({ path: `/${this.projectFullPath}/-/tickets` });
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
.blm-sticky-ticket-toolbar {
  margin-top: 48px !important;
  border-bottom: 1px solid #e4e4e4 !important;
}

.blm-toolbar-left {
  left: 256px !important;
}
</style>
