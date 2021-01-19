/* eslint-disable class-methods-use-this */
/* eslint-disable max-len */
import ApiClient from '@/api/client';
import {
  Bot, BotApp, BotConnection, CreateBotConnectionInput, CreateBotInput, DeleteBotConnectionInput, DeleteBotInput, Project, UpdateBotConnectionInput, UpdateBotInput,
} from '@/api/graphql/model';
import Router from '@/app/router';

export class BotsService {
  private apiClient: ApiClient;
  private router: Router;

  constructor(apiClient: ApiClient, router: Router) {
    this.apiClient = apiClient;
    this.router = router;
  }

  async fetchBotsForProject(projectfullPath: string): Promise<Project> {
    const query = `
      query($projectfullPath: String!) {
        project(fullPath: $projectfullPath) {
          id
          createdAt
          name
          path
          avatarUrl

          bots {
            id
            createdAt
            name
            description
            lastExecutedAt
            active
          }
        }
      }
    `;
    const variables = { projectfullPath };

    const res: { project: Project} = await this.apiClient.query(query, variables);
    return res.project;
  }

  async fetchBot(botId: string): Promise<Bot> {
    const query = `
      query($botId: ID!) {
        bot(botId: $botId) {
          id
          createdAt
          name
          description
          lastExecutedAt
          active
        }
      }
    `;
    const variables = { botId };

    const res: { bot: Bot } = await this.apiClient.query(query, variables);
    return res.bot;
  }

  async createBot(input: CreateBotInput): Promise<void> {
    const query = `
      mutation($input: CreateBotInput!) {
        createBot(input: $input) {
          id
        }
      }
    `;
    const variables = { input };

    const res: { createBot: Bot } = await this.apiClient.query(query, variables);
    this.router.push({ path: `/${input.projectFullPath}/-/bots/${res.createBot.id}` });
  }

  async updateBot(input: UpdateBotInput): Promise<Bot> {
    const query = `
      mutation($input: UpdateBotInput!) {
        updateBot(input: $input) {
          id
          createdAt
          name
          description
          lastExecutedAt
          active
        }
      }
    `;
    const variables = { input };

    const res: { updateBot: Bot } = await this.apiClient.query(query, variables);
    return res.updateBot;
  }

  async deleteBot(projectFullPath: string, input: DeleteBotInput): Promise<void> {
    const query = `
      mutation($input: DeleteBotInput!) {
        deleteBot(input: $input)
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
    this.router.push({ path: `/${projectFullPath}/-/bots` });
  }

  async fetchBotsHistoryForProject(projectfullPath: string): Promise<Project> {
    const query = `
      query($projectfullPath: String!) {
        project(fullPath: $projectfullPath) {
          id
          createdAt
          name
          path
          avatarUrl

          botsHistory {
            id
            createdAt
            completedAt
            status

            bot {
              id
              createdAt
              name
            }
          }
        }
      }
    `;
    const variables = { projectfullPath };

    const res: { project: Project} = await this.apiClient.query(query, variables);
    return res.project;
  }

  async fetchBotsConnectionsForProject(projectfullPath: string): Promise<Project> {
    const query = `
      query($projectfullPath: String!) {
        project(fullPath: $projectfullPath) {
          id
          createdAt
          name
          path
          avatarUrl

          botsConnections {
            id
            createdAt
            name
            description
            app {
              id
              name
              description
              avatarUrl
            }
          }
        }
      }
    `;
    const variables = { projectfullPath };

    const res: { project: Project } = await this.apiClient.query(query, variables);
    return res.project;
  }

  async fetchBotConnection(connectionId: string): Promise<BotConnection> {
    const query = `
      query($connectionId: ID!) {
        botConnection(connectionId: $connectionId) {
          id
          createdAt
          name
          description
          app {
            id
            name
            description
            avatarUrl
          }
        }
      }
    `;
    const variables = { connectionId };

    const res: { botConnection: BotConnection } = await this.apiClient.query(query, variables);
    return res.botConnection;
  }

  async updateConnection(input: UpdateBotConnectionInput): Promise<BotConnection> {
    const query = `
      mutation($input: UpdateBotConnectionInput!) {
        updateBotConnection(input: $input) {
          id
          createdAt
          name
          description
          app {
            id
            name
            description
            avatarUrl
          }
        }
      }
    `;
    const variables = { input };

    const res: { updateBotConnection: BotConnection } = await this.apiClient.query(query, variables);
    return res.updateBotConnection;
  }

  async deleteConnection(projectFullPath: string, input: DeleteBotConnectionInput): Promise<void> {
    const query = `
      mutation($input: DeleteBotConnectionInput!) {
        deleteBotConnection(input: $input)
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
    this.router.push({ path: `/${projectFullPath}/-/bots/connections` });
  }

  async fetchAllApps(): Promise<BotApp[]> {
    const query = `
      query {
        botsApps {
          id
          name
          description
          avatarUrl
        }
      }
    `;
    const variables = {};

    const res: { botsApps: BotApp[] } = await this.apiClient.query(query, variables);
    return res.botsApps;
  }

  async createConnection(input: CreateBotConnectionInput): Promise<void> {
    const query = `
      mutation($input: CreateBotConnectionInput!) {
        createBotConnection(input: $input) {
          id
          createdAt
        }
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
    this.router.push({ path: `/${input.projectFullPath}/-/bots/connections` });
  }
}

export const BotsServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: BotsService) {
    Vue.prototype.$botsService = service;
  },
};
