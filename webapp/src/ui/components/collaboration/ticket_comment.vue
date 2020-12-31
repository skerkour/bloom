<template>
  <v-card outlined>

    <v-card-title>
      <v-avatar size="25" class="mr-4">
        <v-img :src="comment.author.avatarUrl" />
      </v-avatar>
      <router-link :to="`/${comment.author.username}`">
        {{ comment.author.name }}
      </router-link>

      <v-spacer />

      <v-btn icon @click="editClicked" v-if="editable">
        <v-icon>mdi-pencil</v-icon>
      </v-btn>

      <v-menu bottom v-if="editable">
        <template v-slot:activator="{ on }">
          <v-btn icon v-on="on">
            <v-icon>mdi-dots-vertical</v-icon>
          </v-btn>
        </template>

        <v-list>
          <v-list-item @click="deleteComment">
            <v-list-item-icon>
              <v-icon>mdi-delete</v-icon>
            </v-list-item-icon>
            <v-list-item-title>Delete comment</v-list-item-title>
          </v-list-item>
        </v-list>
      </v-menu>

    </v-card-title>


    <v-card-text v-if="editing">
      <b-markdown-editor v-model="body" placeholder="Leave a comment..." />
    </v-card-text>
    <v-card-text v-html="bodyHtml" v-else/>

    <v-card-actions v-if="editing">
      <v-btn text :loading="loading" @click="cancel">
        Cancel
      </v-btn>

      <v-spacer />

      <v-btn color="success" :loading="loading"
        depressed @click="updateComment">
        Save changes
      </v-btn>
    </v-card-actions>

  </v-card>
</template>


<script lang="ts">
import { PropType } from 'vue';
import { VueApp } from '@/app/vue';
import {
  TicketComment, DeleteTicketCommentInput, UpdateTicketCommentInput,
} from '@/api/graphql/model';
import BMarkdownEditor from '@/ui/components/kernel/markdown_editor.vue';


export default VueApp.extend({
  name: 'BTicketComment',
  components: {
    BMarkdownEditor,
  },
  props: {
    comment: {
      type: Object as PropType<TicketComment>,
      required: true,
    },
  },
  data() {
    return {
      error: '',
      loading: false,
      editing: false,
      body: '',
      bodyHtml: '',
    };
  },
  computed: {
    editable(): boolean {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      return this.comment.author.username === this.$store.state.me!.username && !this.editing;
    },
  },
  watch: {
    comment(newComment: TicketComment) {
      this.resetFields(newComment);
    },
  },
  created() {
    this.resetFields(this.comment);
  },
  methods: {
    editClicked() {
      this.editing = true;
    },
    cancel() {
      this.resetFields(this.comment);
    },
    resetFields(comment: TicketComment) {
      this.body = comment.body;
      this.bodyHtml = comment.bodyHtml;
      this.editing = false;
    },
    async deleteComment(): Promise<void> {
      // eslint-disable-next-line no-alert, no-restricted-globals
      if (!confirm('Do you really want to delete comment?')) {
        return;
      }

      this.loading = true;
      this.error = '';
      const input: DeleteTicketCommentInput = {
        commentId: this.comment.id,
      };

      try {
        await this.$collaborationService.deleteComment(input);
        this.$emit('deleted', this.comment);
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    },
    async updateComment(): Promise<void> {
      this.loading = true;
      this.error = '';
      const input: UpdateTicketCommentInput = {
        commentId: this.comment.id,
        body: this.body,
      };

      try {
        const comment = await this.$collaborationService.updateComment(input);
        this.$emit('updated', comment);
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
