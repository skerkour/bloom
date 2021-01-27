<template>
  <v-container fluid>
    <v-row justify="center">
      <v-col cols="12" md="8" lg="6" xl="4" v-if="error !== ''">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row justify="center" class="text-center" v-if="loading">
      <v-col cols="12" sm="10" md="8" xl="6">
        <v-progress-circular
          :size="50"
          color="primary"
          indeterminate
        />
      </v-col>
    </v-row>

    <v-row v-if="ticket">
      <ticket-details
        :projectFullPath="projectFullPath" :projectLabels="labels" :ticket="ticket"
        @updated="ticketUpdated" @close="closeTicket" @reopen="reopenTicket"
        :projectMilestones="milestones" />
    </v-row>

    <v-row v-if="ticket" >
      <v-col cols="12" v-for="comment in ticket.comments" :key="comment.id" class="py-1">
        <comment :comment="comment" @deleted="onCommentDeleted" @updated="onCommentUpdated" />
      </v-col>
    </v-row>

    <v-row v-if="ticket">
      <v-col cols="12">
        <v-card flat>
          <v-card-text class="py-1">
            <b-markdown-editor v-model="newCommentBody" placeholder="Leave a comment..." outlined />
          </v-card-text>

          <v-card-actions>
            <v-btn depressed :loading="loading" :disabled="!canComment"
              color="success" @click="commentTicket" class="ml-2">
              Comment
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import { VueApp } from '@/app/vue';
import TicketDetails from '@/ui/components/collaboration/ticket_details.vue';
import {
  Project, Label, Ticket, CommentTicketInput, CloseTicketInput,
  ReopenTicketInput, TicketComment, Milestone,
} from '@/api/graphql/model';
import Comment from '@/ui/components/collaboration/ticket_comment.vue';
import BMarkdownEditor from '@/ui/components/kernel/markdown_editor.vue';


export default VueApp.extend({
  name: 'ProjectNewTicketView',
  components: {
    TicketDetails,
    Comment,
    BMarkdownEditor,
  },
  data() {
    return {
      error: '',
      loading: false,
      ticket: null as Ticket | null,
      project: null as Project | null,
      newCommentBody: '',
    };
  },
  computed: {
    labels(): Label[] {
      return this.project?.labels ?? [];
    },
    milestones(): Milestone[] {
      return this.project?.milestones.filter((milestone: Milestone) => !milestone.closedAt) ?? [];
    },
    projectFullPath(): string {
      return `${this.$route.params.namespacePath}/${this.$route.params.projectPath}`;
    },
    canComment(): boolean {
      return this.newCommentBody.length !== 0;
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
        const res = await this.$collaborationService.fetchTicketWithProjectLabelsAndMilestones(
          this.$route.params.ticketId,
          this.projectFullPath,
        );
        this.project = res.project;
        this.ticket = res.ticket;
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async commentTicket(): Promise<void> {
      this.loading = true;
      this.error = '';
      const input: CommentTicketInput = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        ticketId: this.ticket!.id,
        body: this.newCommentBody,
      };

      try {
        // eslint-disable-next-line max-len
        const comment = await this.$collaborationService.commentTicket(input);
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        this.ticket!.comments.push(comment);
        this.newCommentBody = '';
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async closeTicket(): Promise<void> {
      this.loading = true;
      this.error = '';
      const input: CloseTicketInput = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        ticketId: this.ticket!.id,
      };

      try {
        const ticket = await this.$collaborationService.closeTicket(input);
        this.ticketUpdated(ticket);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async reopenTicket() {
      this.loading = true;
      this.error = '';
      const input: ReopenTicketInput = {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        ticketId: this.ticket!.id,
      };

      try {
        const ticket = await this.$collaborationService.reopenTicket(input);
        this.ticketUpdated(ticket);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    ticketUpdated(updatedTicket: Ticket) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      updatedTicket.comments = this.ticket!.comments;
      this.ticket = updatedTicket;
    },
    onCommentDeleted(deletedComment: TicketComment) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion, max-len
      this.ticket!.comments = this.ticket!.comments.filter((comment: TicketComment) => deletedComment.id !== comment.id);
    },
    onCommentUpdated(updatedComment: TicketComment) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion, max-len
      this.ticket!.comments = this.ticket!.comments.map((comment: TicketComment): TicketComment => {
        if (updatedComment.id === comment.id) {
          return updatedComment;
        }
        return comment;
      });
    },
  },
});
</script>

<style lang="scss" scoped>
</style>
