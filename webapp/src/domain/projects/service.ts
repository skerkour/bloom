import ApiClient from '@/api/client';
import {
  CreateProjectInput, Project, DeleteProjectInput, UpdateProjectInput,
} from '@/api/graphql/model';
import Router from '@/app/router';
import { KernelService } from '../kernel/service';


export type ProjectWithNamespace = {
  project: Project;
  namespace: string;
}

export class ProjectsService {
  private apiClient: ApiClient;
  private router: Router;
  private kernelService: KernelService;

  constructor(apiClient: ApiClient, router: Router, kernelService: KernelService) {
    this.apiClient = apiClient;
    this.router = router;
    this.kernelService = kernelService;
  }

  async createProject(input: CreateProjectInput) {
    const query = `
      mutation($input: CreateProjectInput!) {
        createProject(input: $input) {
          path
        }
      }
    `;
    const variables = { input };

    const res: { createProject: Project } = await this.apiClient.query(query, variables);
    this.router.push({ path: `/${input.namespacePath}/${res.createProject.path}` });
  }

  async fetchProjectOverview(projectfullPath: string): Promise<Project> {
    const query = `
      query($projectfullPath: String!) {
        project(fullPath: $projectfullPath) {
          id
          createdAt
          name
          path
          description
          avatarUrl
        }
      }
    `;
    const variables = { projectfullPath };

    const res: { project: Project} = await this.apiClient.query(query, variables);
    return res.project;
  }

  async deleteProject(namespacePath: string, projectPath: string) {
    const query = `
      mutation($input: DeleteProjectInput!) {
        deleteProject(input: $input)
      }
    `;
    const input: DeleteProjectInput = {
      fullPath: `${namespacePath}/${projectPath}`,
    };
    const variables = { input };
    await this.apiClient.query(query, variables);
    this.router.push({ path: `/${namespacePath}` });
  }

  async fetchProjectPreferences(projectfullPath: string): Promise<Project> {
    const query = `
      query($projectfullPath: String!) {
        project(fullPath: $projectfullPath) {
          id
          createdAt
          name
          path
          description
          avatarUrl
          twitterUrl
          facebookUrl
          publicEmail
          instagramUrl
          whatsappNumber
          mastodonUrl
          homepageUrl

          chatboxPreferences {
            color
            name
            avatarUrl
            branding
            welcomeMessage
          }
        }
      }
    `;
    const variables = { projectfullPath };

    const res: { project: Project} = await this.apiClient.query(query, variables);
    return res.project;
  }

  async updateProject(input: UpdateProjectInput): Promise<Project> {
    const query = `
      mutation($input: UpdateProjectInput!) {
        updateProject(input: $input) {
          id
          createdAt
          name
          path
          description
          avatarUrl
          twitterUrl
          facebookUrl
          publicEmail
          instagramUrl
          whatsappNumber
          mastodonUrl
          homepageUrl

          chatboxPreferences {
            color
            name
            avatarUrl
          }
        }
      }
    `;
    const variables = { input };

    const res: { updateProject: Project } = await this.apiClient.query(query, variables);
    return res.updateProject;
  }

  async updateProjectAvatar(projectId: string, file: File): Promise<string> {
    this.kernelService.validateAvatar(file);

    const query = `
      mutation($input: UpdateProjectInput!) {
        updateProject(input: $input) {
          id
          avatarUrl
        }
      }
    `;
    const input: UpdateProjectInput = {
      projectId,
    };
    const variables = { input };
    const operations = { query, variables };
    const map = {
      0: ['variables.input.avatar'],
    };

    const formData = new FormData();
    formData.append('operations', JSON.stringify(operations));
    formData.append('map', JSON.stringify(map));
    formData.append('0', file);

    const res: { updateProject: Project } = await this.apiClient.upload(formData);
    return res.updateProject.avatarUrl;
  }

  async adminFetchAllProjects(): Promise<Project[]> {
    const query = `
      query {
        adminProjects {
          id
          createdAt
          avatarUrl
          path
          name
        }
      }
    `;
    const variables = {};

    const res: { adminProjects: Project[] } = await this.apiClient.query(query, variables);
    return res.adminProjects;
  }

  async adminFetchProject(projectId: string): Promise<Project> {
    const query = `
      query($projectId: ID!) {
        adminProject(projectId: $projectId) {
          id
          createdAt
          avatarUrl
          path
          name
        }
      }
    `;
    const variables = { projectId };

    const res: { adminProject: Project } = await this.apiClient.query(query, variables);
    return res.adminProject;
  }
}

export const ProjectsServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: ProjectsService) {
    Vue.prototype.$projectsService = service;
  },
};
