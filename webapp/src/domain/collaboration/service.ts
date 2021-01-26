import ApiClient from '@/api/client';
import {
  Project, CreateLabelInput, UpdateLabelInput, Label, DeleteLabelInput, CreateTicketInput,
  Ticket, UpdateTicketInput, CloseTicketInput, CommentTicketInput, TicketComment, ReopenTicketInput,
  DeleteTicketCommentInput, UpdateTicketCommentInput, File as ApiFile, CreateFolderInput,
  EmptyTrashInput, RestoreFilesFromTrashInput, RenameFileInput, MoveFilesToTrashInput,
  CompleteFileUploadInput, Milestone, CreateMilestoneInput, DeleteMilestoneInput,
  UpdateMilestoneInput, CloseMilestoneInput, ReopenMilestoneInput, DeleteTicketInput,
} from '@/api/graphql/model';
import Router from '@/app/router';


const ticketFragment = `
  id
  createdAt
  updatedAt
  title
  body
  bodyHtml
  dueDate
  lastEditedAt
  closedAt

  author {
    id
    username
    name
    avatarUrl
  }
  labels {
    id
    createdAt
    name
    description
    backgroundColor
    textColor
  }
  milestones {
    id
    createdAt
    title
    startDate
    dueDate
    closedAt
  }
`;

const ticketWithCommentFragment = `
${ticketFragment}
comments {
  id
  createdAt
  lastEditedAt
  body
  bodyHtml
  author {
    id
    username
    name
    avatarUrl
  }
}
`;


type TicketAndProject = {
  project: Project;
  ticket: Ticket;
}

export class CollaborationService {
  private apiClient: ApiClient;
  private router: Router;
  fileTypeFolder: string;

  constructor(apiClient: ApiClient, router: Router) {
    this.apiClient = apiClient;
    this.router = router;
    this.fileTypeFolder = 'application/com.bloom42.folder';
  }

  async fetchTickets(projectFullPath: string): Promise<Project> {
    const query = `
      query($projectFullPath: String!) {
        project(fullPath: $projectFullPath) {
          id
          name

          tickets {
            id
            createdAt
            title
            dueDate
            closedAt
            commentsCount

            author {
              id
              username
              name
              avatarUrl
            }
          }
        }
      }
    `;
    const variables = { projectFullPath };

    const res: { project: Project } = await this.apiClient.query(query, variables);
    return res.project;
  }

  async deleteTicket(input: DeleteTicketInput): Promise<void> {
    const query = `
      mutation($input: DeleteTicketInput!) {
        deleteTicket(input: $input)
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
  }

  async fetchLabels(projectFullPath: string): Promise<Project> {
    const query = `
      query($projectFullPath: String!) {
        project(fullPath: $projectFullPath) {
          id
          name

          labels {
            id
            createdAt
            name
            description
            backgroundColor
            textColor
          }
        }
      }
    `;
    const variables = { projectFullPath };

    const res: { project: Project } = await this.apiClient.query(query, variables);
    return res.project;
  }

  async fetchLabelsAndMilestones(projectFullPath: string): Promise<Project> {
    const query = `
      query($projectFullPath: String!) {
        project(fullPath: $projectFullPath) {
          id
          name

          labels {
            id
            createdAt
            name
            description
            backgroundColor
            textColor
          }

          milestones {
            id
            createdAt
            title
            descriptionHtml
            startDate
            dueDate
            closedAt
          }
        }
      }
    `;
    const variables = { projectFullPath };

    const res: { project: Project } = await this.apiClient.query(query, variables);
    return res.project;
  }

  async fetchLabelsAndLists(projectFullPath: string): Promise<Project> {
    const query = `
      query($projectFullPath: String!) {
        project(fullPath: $projectFullPath) {
          id
          name

          labels {
            id
            createdAt
            name
            description
            backgroundColor
            textColor
          }

          lists {
            id
            createdAt
            name
            description
          }
        }
      }
    `;
    const variables = { projectFullPath };

    const res: { project: Project } = await this.apiClient.query(query, variables);
    return res.project;
  }

  async createLabel(input: CreateLabelInput): Promise<void> {
    const query = `
      mutation($input: CreateLabelInput!) {
        createLabel(input: $input) {
          id
        }
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
    this.router.push({ path: `/${input.projectFullPath}/-/labels` });
  }

  async updateLabel(projectFullPath: string, input: UpdateLabelInput): Promise<void> {
    const query = `
      mutation($input: UpdateLabelInput!) {
        updateLabel(input: $input) {
          id
        }
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
    this.router.push({ path: `/${projectFullPath}/-/labels` });
  }

  async deleteLabel(input: DeleteLabelInput): Promise<void> {
    const query = `
      mutation($input: DeleteLabelInput!) {
        deleteLabel(input: $input)
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
  }

  async fetchLabel(labelId: string): Promise<Label> {
    const query = `
      query($labelId: ID!) {
        label(labelId: $labelId) {
          id
          createdAt
          name
          description
          backgroundColor
          textColor
        }
      }
    `;
    const variables = { labelId };

    const res: { label: Label } = await this.apiClient.query(query, variables);
    return res.label;
  }

  async createTicket(input: CreateTicketInput): Promise<void> {
    const query = `
      mutation($input: CreateTicketInput!) {
        createTicket(input: $input) {
          id
        }
      }
    `;
    const variables = { input };

    const res: { createTicket: Ticket } = await this.apiClient.query(query, variables);
    this.router.push({ path: `/${input.projectFullPath}/-/tickets/${res.createTicket.id}` });
  }

  async updateTicket(input: UpdateTicketInput): Promise<Ticket> {
    const query = `
      mutation($input: UpdateTicketInput!) {
        updateTicket(input: $input) {
          ${ticketFragment}
        }
      }
    `;
    const variables = { input };

    const res: { updateTicket: Ticket } = await this.apiClient.query(query, variables);
    return res.updateTicket;
  }

  async fetchTicketWithProjectLabelsAndMilestones(ticketId: string, projectFullPath: string):
    Promise<TicketAndProject> {
    const query = `
      query($ticketId: ID!, $projectFullPath: String!) {
        ticket(ticketId: $ticketId) {
          ${ticketWithCommentFragment}
        }
        project(fullPath: $projectFullPath) {
          id
          name

          labels {
            id
            createdAt
            name
            description
            backgroundColor
            textColor
          }

          milestones {
            id
            createdAt
            title
            descriptionHtml
            startDate
            dueDate
            closedAt
          }
        }
      }
    `;
    const variables = { ticketId, projectFullPath };

    const res: TicketAndProject = await this.apiClient.query(query, variables);
    return res;
  }

  async closeTicket(input: CloseTicketInput): Promise<Ticket> {
    const query = `
      mutation($input: CloseTicketInput!) {
        closeTicket(input: $input) {
          ${ticketFragment}
        }
      }
    `;
    const variables = { input };

    const res: { closeTicket: Ticket } = await this.apiClient.query(query, variables);
    return res.closeTicket;
  }

  async reopenTicket(input: ReopenTicketInput): Promise<Ticket> {
    const query = `
      mutation($input: ReopenTicketInput!) {
        reopenTicket(input: $input) {
          ${ticketFragment}
        }
      }
    `;
    const variables = { input };

    const res: { reopenTicket: Ticket } = await this.apiClient.query(query, variables);
    return res.reopenTicket;
  }

  async commentTicket(input: CommentTicketInput): Promise<TicketComment> {
    const query = `
      mutation($input: CommentTicketInput!) {
        commentTicket(input: $input) {
          id
          createdAt
          lastEditedAt
          body
          bodyHtml
          author {
            id
            username
            name
            avatarUrl
          }
        }
      }
    `;
    const variables = { input };

    const res: { commentTicket: TicketComment } = await this.apiClient.query(query, variables);
    return res.commentTicket;
  }

  async renderMarkdown(input: string): Promise<string> {
    const query = `
      query($input: String!) {
        markdown(input: $input)
      }
    `;
    const variables = { input };

    const res: { markdown: string } = await this.apiClient.query(query, variables);
    return res.markdown;
  }

  async updateComment(input: UpdateTicketCommentInput): Promise<TicketComment> {
    const query = `
      mutation($input: UpdateTicketCommentInput!) {
        updateTicketComment(input: $input) {
          id
          createdAt
          lastEditedAt
          body
          bodyHtml
          author {
            id
            username
            name
            avatarUrl
          }
        }
      }
    `;
    const variables = { input };

    const res: { updateTicketComment: TicketComment } = await
    this.apiClient.query(query, variables);
    return res.updateTicketComment;
  }

  async deleteComment(input: DeleteTicketCommentInput): Promise<void> {
    const query = `
      mutation($input: DeleteTicketCommentInput!) {
        deleteTicketComment(input: $input)
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
  }

  async fetchFile(projectFullPath: string, fileId: string | null): Promise<ApiFile> {
    const query = `
      query($projectFullPath: String!, $fileId: ID) {
        file(projectFullPath: $projectFullPath, fileId: $fileId) {
          id
          createdAt
          name
          size
          type
          trashedAt
          path {
            id
            name
          }

          children {
            id
            createdAt
            name
            size
            type
            trashedAt
          }
        }
      }
    `;
    const variables = {
      projectFullPath,
      fileId,
    };

    const res: { file: ApiFile } = await this.apiClient.query(query, variables);
    return res.file;
  }

  async createFolder(input: CreateFolderInput): Promise<ApiFile> {
    const query = `
      mutation($input: CreateFolderInput!) {
        createFolder(input: $input) {
          id
          createdAt
          name
          size
          type
          trashedAt
          path {
            id
            name
          }
        }
      }
    `;
    const variables = { input };

    const res: { createFolder: ApiFile } = await this.apiClient.query(query, variables);
    return res.createFolder;
  }

  async fetchTrash(projectFullPath: string): Promise<ApiFile[]> {
    const query = `
      query($projectFullPath: String!) {
        trash(projectFullPath: $projectFullPath) {
          id
          createdAt
          name
          size
          type
          trashedAt
        }
      }
    `;
    const variables = {
      projectFullPath,
    };

    const res: { trash: ApiFile[] } = await this.apiClient.query(query, variables);
    return res.trash;
  }

  async emptyTrash(input: EmptyTrashInput): Promise<void> {
    const query = `
      mutation($input: EmptyTrashInput!) {
        emptyTrash(input: $input)
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
  }

  async restoreFilesFromTrash(input: RestoreFilesFromTrashInput): Promise<void> {
    const query = `
      mutation($input: RestoreFilesFromTrashInput!) {
        restoreFilesFromTrash(input: $input)
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
  }

  async moveFilesToTrash(input: MoveFilesToTrashInput): Promise<void> {
    const query = `
      mutation($input: MoveFilesToTrashInput!) {
        moveFilesToTrash(input: $input)
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
  }

  async renameFile(input: RenameFileInput): Promise<ApiFile> {
    const query = `
      mutation($input: RenameFileInput!) {
        renameFile(input: $input) {
          id
          createdAt
          name
          size
          type
          trashedAt
          path {
            id
            name
          }
        }
      }
    `;
    const variables = { input };

    const res: { renameFile: ApiFile } = await this.apiClient.query(query, variables);
    return res.renameFile;
  }

  async fetchMilestones(projectFullPath: string): Promise<Project> {
    const query = `
      query($projectFullPath: String!) {
        project(fullPath: $projectFullPath) {
          id
          name

          milestones {
            id
            createdAt
            title
            descriptionHtml
            startDate
            dueDate
            closedAt
          }
        }
      }
    `;
    const variables = { projectFullPath };

    const res: { project: Project } = await this.apiClient.query(query, variables);
    return res.project;
  }

  async fetchMilestone(milestoneId: string): Promise<Milestone> {
    const query = `
      query($milestoneId: ID!) {
        milestone(milestoneId: $milestoneId) {
          id
          createdAt
          updatedAt
          title
          description
          descriptionHtml
          startDate
          dueDate
          closedAt

          tickets {
            id
            title
            dueDate
            closedAt
            createdAt

            author {
              id
              username
              name
            }
          }
        }
      }
    `;
    const variables = { milestoneId };

    const res: { milestone: Milestone } = await this.apiClient.query(query, variables);
    return res.milestone;
  }

  async createMilestone(input: CreateMilestoneInput): Promise<void> {
    const query = `
      mutation($input: CreateMilestoneInput!) {
        createMilestone(input: $input) {
          id
        }
      }
    `;
    const variables = { input };

    const res: { createMilestone: Milestone } = await this.apiClient.query(query, variables);
    this.router.push({ path: `/${input.projectFullPath}/-/milestones/${res.createMilestone.id}` });
  }

  async updateMilestone(input: UpdateMilestoneInput): Promise<Milestone> {
    const query = `
    mutation($input: UpdateMilestoneInput!) {
        updateMilestone(input: $input) {
          id
          createdAt
          updatedAt
          title
          description
          descriptionHtml
          startDate
          dueDate
          closedAt

          tickets {
            id
            title
            dueDate
            closedAt
          }
        }
      }
    `;
    const variables = { input };

    const res: { updateMilestone: Milestone } = await this.apiClient.query(query, variables);
    return res.updateMilestone;
  }

  async closeMilestone(input: CloseMilestoneInput): Promise<Milestone> {
    const query = `
    mutation($input: CloseMilestoneInput!) {
        closeMilestone(input: $input) {
          id
          createdAt
          updatedAt
          title
          description
          descriptionHtml
          startDate
          dueDate
          closedAt

          tickets {
            id
            title
            dueDate
            closedAt
          }
        }
      }
    `;
    const variables = { input };

    const res: { closeMilestone: Milestone } = await this.apiClient.query(query, variables);
    return res.closeMilestone;
  }

  async reopenMilestone(input: ReopenMilestoneInput): Promise<Milestone> {
    const query = `
    mutation($input: ReopenMilestoneInput!) {
        reopenMilestone(input: $input) {
          id
          createdAt
          updatedAt
          title
          description
          descriptionHtml
          startDate
          dueDate
          closedAt

          tickets {
            id
            title
            dueDate
            closedAt
          }
        }
      }
    `;
    const variables = { input };

    const res: { reopenMilestone: Milestone } = await this.apiClient.query(query, variables);
    return res.reopenMilestone;
  }

  async deleteMilestone(input: DeleteMilestoneInput): Promise<void> {
    const query = `
      mutation($input: DeleteMilestoneInput!) {
        deleteMilestone(input: $input)
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
  }

  async completeFileUpload(input: CompleteFileUploadInput): Promise<ApiFile> {
    const query = `
      mutation($input: CompleteFileUploadInput!) {
        completeFileUpload(input: $input) {
          id
          createdAt
          name
          size
          type
          trashedAt
          path {
            id
            name
          }
        }
      }
    `;
    const variables = { input };

    const res: { completeFileUpload: ApiFile } = await this.apiClient.query(query, variables);
    return res.completeFileUpload;
  }


  // eslint-disable-next-line max-len
  // async uploadFile(parentId: string, file: File, options?: AxiosRequestConfig): Promise<ApiFile> {
  //   const query = `
  //     mutation($input: UploadFileInput!) {
  //       uploadFile(input: $input) {
  //         id
  //         createdAt
  //         name
  //         size
  //         type
  //         trashedAt
  //         url
  //         path {
  //           id
  //           name
  //         }
  //       }
  //     }
  //   `;
  //   const input: UploadFileInput = {
  //     parentId,
  //     file: null,
  //   };
  //   const variables = { input };
  //   const operations = { query, variables };
  //   const map = {
  //     0: ['variables.input.file'],
  //   };

  //   const formData = new FormData();
  //   formData.append('operations', JSON.stringify(operations));
  //   formData.append('map', JSON.stringify(map));
  //   formData.append('0', file);

  //   const res: { uploadFile: ApiFile } = await this.apiClient.upload(formData, options);
  //   return res.uploadFile;
  // }

  // eslint-disable-next-line class-methods-use-this
  async downloadFile(projectFullPath: string, file: ApiFile): Promise<void> {
    if (!file.url) {
      const query = `
        query($projectFullPath: String!, $fileId: ID) {
          file(projectFullPath: $projectFullPath, fileId: $fileId) {
            id
            url
          }
        }
      `;
      const variables = {
        projectFullPath,
        fileId: file.id,
      };

      const res: { file: ApiFile } = await this.apiClient.query(query, variables);
      file = res.file;
    }

    const downloadLink: HTMLAnchorElement = document.createElement('a');
    downloadLink.href = file.url;
    document.body.appendChild(downloadLink);
    downloadLink.click();
    document.body.removeChild(downloadLink);
  }
}

export const CollaborationServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: CollaborationService) {
    Vue.prototype.$collaborationService = service;
  },
};
