import ApiClient from '@/api/client';
import Router from '@/app/router';
import {
  Project, Monitor, CreateMonitorInput, DeleteMonitorInput, UpdateMonitorInput,
} from '@/api/graphql/model';

export class OperationsService {
  private apiClient: ApiClient;

  constructor(apiClient: ApiClient, private router: Router) {
    this.apiClient = apiClient;
  }

  async fetchMonitors(projectFullPath: string): Promise<Project> {
    const query = `
      query($projectFullPath: String!) {
        project(fullPath: $projectFullPath) {
          id
          name

          monitors {
            id
            createdAt
            name
            status
            endpoint
            type
            isActive
            showOnStatusPage
          }
        }
      }
    `;
    const variables = { projectFullPath };

    const res: { project: Project } = await this.apiClient.query(query, variables);
    return res.project;
  }

  async createMonitor(input: CreateMonitorInput): Promise<void> {
    const query = `
    mutation($input: CreateMonitorInput!) {
        createMonitor(input: $input) {
          id
        }
      }
    `;
    const variables = { input };

    const res: { createMonitor: Monitor } = await this.apiClient.query(query, variables);
    this.router.push({ path: `/${input.projectFullPath}/-/monitors/${res.createMonitor.id}` });
  }

  async deleteMonitor(projectFullPath: string, input: DeleteMonitorInput): Promise<void> {
    const query = `
    mutation($input: DeleteMonitorInput!) {
        deleteMonitor(input: $input)
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
    this.router.push({ path: `/${projectFullPath}/-/monitors/` });
  }

  async fetchMonitor(monitorId: string): Promise<Monitor> {
    const query = `
      query($monitorId: ID!) {
        monitor(monitorId: $monitorId) {
          id
          createdAt
          name
          status
          endpoint
          type
          httpMethod
          bodyTextToMatch
          minHTTPStatusCode
          maxHTTPStatusCode
          followHTTPRedirects
          isActive
          showOnStatusPage

          pings {
            id
            createdAt
            startedAt
            dnsResolution
            tcpConnection
            tlsHandshake
            serverProcessing
            contentTransfer
            timeToFirstByte
            totalDuration
            contentSize
            error
          }

          statusChanges {
            id
            createdAt
            from
            to
          }

          uptime {
            date
            uptime
          }
        }
      }
    `;
    const variables = { monitorId };

    const res: { monitor: Monitor } = await this.apiClient.query(query, variables);
    return res.monitor;
  }

  async updateMonitor(input: UpdateMonitorInput): Promise<Monitor> {
    const query = `
      mutation($input: UpdateMonitorInput!) {
        updateMonitor(input: $input) {
          id
          createdAt
          name
          status
          endpoint
          type
          httpMethod
          bodyTextToMatch
          minHTTPStatusCode
          maxHTTPStatusCode
          followHTTPRedirects
          isActive
          showOnStatusPage

          pings {
            id
            createdAt
            startedAt
            dnsResolution
            tcpConnection
            tlsHandshake
            serverProcessing
            contentTransfer
            timeToFirstByte
            totalDuration
            contentSize
            error
          }

          statusChanges {
            id
            createdAt
            from
            to
          }

          uptime {
            date
            uptime
          }
        }
      }
    `;
    const variables = { input };

    const res: { updateMonitor: Monitor } = await this.apiClient.query(query, variables);
    return res.updateMonitor;
  }
}

export const OperationsServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: OperationsService) {
    Vue.prototype.$operationsService = service;
  },
};
