import ApiClient from '@/api/client';
import Router from '@/app/router';
import {
  Project, OutboundMessage, CreateOutboundMessageInput, UpdateOutboundMessageInput,
  DeleteOutboundMessageInput, SendOutboundMessageInput, SendTestOutboundMessageInput, List,
  CreateListInput, DeleteListInput, UpdateListInput, ConfirmListSubscriptionInput,
  UnsubscribeFromListInput, Contact, DeleteContactInput, UpdateContactInput, CreateContactInput,
  ImportContactsInput,
} from '@/api/graphql/model';

type OutboundMessageAndProject = {
  project: Project;
  outboundMessage: OutboundMessage;
}

type ContactAndProject = {
  project: Project;
  contact: Contact;
}


export class GrowthService {
  private apiClient: ApiClient;

  constructor(apiClient: ApiClient, private router: Router) {
    this.apiClient = apiClient;
  }

  async fetchContacts(projectFullPath: string): Promise<Project> {
    const query = `
      query($projectFullPath: String!) {
        project(fullPath: $projectFullPath) {
          id
          name

          contacts {
            id
            createdAt
            avatarUrl
            name
            email
            pgpKey
            phone
            address
            website
            twitter
            instagram
            facebook
            linkedin
            skype
            telegram
            notes
            country
            countryCode
            plan
            userId
          }
        }
      }
    `;
    const variables = { projectFullPath };

    const res: { project: Project } = await this.apiClient.query(query, variables);
    return res.project;
  }

  async importContacts(input: ImportContactsInput): Promise<Contact[]> {
    const query = `
      mutation($input: ImportContactsInput!) {
        importContacts(input: $input) {
          id
          createdAt
          avatarUrl
          name
          email
          pgpKey
          phone
          address
          website
          twitter
          instagram
          facebook
          linkedin
          skype
          telegram
          notes
          country
          countryCode
          plan
          userId
        }
      }
    `;
    const variables = { input };

    const res: { importContacts: Contact[] } = await this.apiClient.query(query, variables);
    return res.importContacts;
  }

  // eslint-disable-next-line max-len
  async fetchContactWithLabelsAndLists(projectFullPath: string, contactId: string): Promise<ContactAndProject> {
    const query = `
      query($contactId: ID!, $projectFullPath: String!) {
        contact(contactId: $contactId) {
          id
          createdAt
          avatarUrl
          name
          email
          pgpKey
          phone
          address
          website
          twitter
          instagram
          facebook
          linkedin
          skype
          telegram
          notes
          country
          countryCode
          plan
          userId

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
            name
            description
          }
        }
        project(fullPath: $projectFullPath) {
          id

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
            name
            description
          }
        }
      }
    `;
    const variables = { contactId, projectFullPath };

    const res: ContactAndProject = await this.apiClient.query(query, variables);
    return res;
  }

  async createContact(input: CreateContactInput): Promise<void> {
    const query = `
      mutation($input: CreateContactInput!) {
        createContact(input: $input) {
          id
        }
      }
    `;
    const variables = { input };

    const res: { createContact: Contact } = await this.apiClient.query(query, variables);
    this.router.push({ path: `/${input.projectFullPath}/-/contacts/${res.createContact.id}` });
  }

  async updateContact(input: UpdateContactInput): Promise<Contact> {
    const query = `
      mutation($input: UpdateContactInput!) {
        updateContact(input: $input) {
          id
          createdAt
          avatarUrl
          name
          email
          pgpKey
          phone
          address
          website
          twitter
          instagram
          facebook
          linkedin
          skype
          telegram
          notes
          country
          countryCode
          plan
          userId

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
            name
            description
          }
        }
      }
    `;
    const variables = { input };

    const res: { updateContact: Contact } = await this.apiClient.query(query, variables);
    return res.updateContact;
  }

  async deleteContact(projectFullPath: string, input: DeleteContactInput): Promise<void> {
    const query = `
      mutation($input: DeleteContactInput!) {
        deleteContact(input: $input)
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
    this.router.push({ path: `/${projectFullPath}/-/contacts` });
  }

  async fetchOutboundMessages(projectFullPath: string): Promise<Project> {
    const query = `
      query($projectFullPath: String!) {
        project(fullPath: $projectFullPath) {
          id
          name

          outboundMessages {
            id
            createdAt
            name
            fromName
            fromAddress
            subject
            body
            bodyHtml
            status
            sendAt
            lastSentAt
            sentCount
            errorCount
            type
          }
        }
      }
    `;
    const variables = { projectFullPath };

    const res: { project: Project } = await this.apiClient.query(query, variables);
    return res.project;
  }

  // eslint-disable-next-line max-len
  async fetchOutboundMessage(messageId: string): Promise<OutboundMessage> {
    const query = `
      query($messageId: ID!, $projectFullPath: String!) {
        outboundMessage(messageId: $messageId) {
          id
          createdAt
          name
          fromName
          fromAddress
          subject
          body
          bodyHtml
          status
          sendAt
          lastSentAt
          sentCount
          errorCount
          type

          lists {
            id
            createdAt
            name
            description
          }
        }
      }
    `;
    const variables = { messageId };

    const res: { outboundMessage: OutboundMessage } = await this.apiClient.query(query, variables);
    return res.outboundMessage;
  }

  // eslint-disable-next-line max-len
  async fetchOutboundMessageWithLists(projectFullPath: string, messageId: string): Promise<OutboundMessageAndProject> {
    const query = `
      query($messageId: ID!, $projectFullPath: String!) {
        outboundMessage(messageId: $messageId) {
          id
          createdAt
          name
          fromName
          fromAddress
          subject
          body
          bodyHtml
          status
          sendAt
          lastSentAt
          sentCount
          errorCount
          type

          lists {
            id
            createdAt
            name
            description
          }
        }
        project(fullPath: $projectFullPath) {
          id

          lists {
            id
            createdAt
            name
            description
          }
        }
      }
    `;
    const variables = { messageId, projectFullPath };

    const res: OutboundMessageAndProject = await this.apiClient.query(query, variables);
    return res;
  }

  async createOutboundMessage(input: CreateOutboundMessageInput): Promise<void> {
    const query = `
    mutation($input: CreateOutboundMessageInput!) {
        createOutboundMessage(input: $input) {
          id
        }
      }
    `;
    const variables = { input };

    // eslint-disable-next-line max-len
    const res: { createOutboundMessage: OutboundMessage } = await this.apiClient.query(query, variables);
    this.router.push({ path: `/${input.projectFullPath}/-/outbound/${res.createOutboundMessage.id}` });
  }

  async updateOutboundMessage(input: UpdateOutboundMessageInput): Promise<OutboundMessage> {
    const query = `
    mutation($input: UpdateOutboundMessageInput!) {
        updateOutboundMessage(input: $input) {
          id
          createdAt
          name
          fromName
          fromAddress
          subject
          body
          bodyHtml
          status
          sendAt
          lastSentAt
          sentCount
          errorCount
          type
        }
      }
    `;
    const variables = { input };

    // eslint-disable-next-line max-len
    const res: { updateOutboundMessage: OutboundMessage } = await this.apiClient.query(query, variables);
    return res.updateOutboundMessage;
  }

  // eslint-disable-next-line max-len
  async deleteOutboundMessage(projectFullPath: string, input: DeleteOutboundMessageInput): Promise<void> {
    const query = `
    mutation($input: DeleteOutboundMessageInput!) {
        deleteOutboundMessage(input: $input)
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
    this.router.push({ path: `/${projectFullPath}/-/outbound` });
  }

  async sendOutboundMessage(input: SendOutboundMessageInput): Promise<OutboundMessage> {
    const query = `
    mutation($input: SendOutboundMessageInput!) {
        sendOutboundMessage(input: $input) {
          id
          createdAt
          name
          fromName
          fromAddress
          subject
          body
          bodyHtml
          status
          sendAt
          lastSentAt
          sentCount
          errorCount
          type
        }
      }
    `;
    const variables = { input };

    // eslint-disable-next-line max-len
    const res: { sendOutboundMessage: OutboundMessage } = await this.apiClient.query(query, variables);
    return res.sendOutboundMessage;
  }

  async sendTestOutboundMessage(input: SendTestOutboundMessageInput): Promise<void> {
    const query = `
      mutation($input: SendTestOutboundMessageInput!) {
        sendTestOutboundMessage(input: $input)
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
  }

  async fetchAnalytics(projectFullPath: string): Promise<Project> {
    const query = `
      query($projectFullPath: String!) {
        project(fullPath: $projectFullPath) {
          id
          name

          analytics {
            visits {
              date
              views
              visitors
            }
            pages {
              url
              path
              views
              visitors
            }
            referrers {
              referrer
              views
              visitors
            }
            devices {
              deviceType
              views
              visitors
            }
            browsers {
              browser
              views
              visitors
            }
            countries {
              country
              views
              visitors
            }
            oses {
              os
              views
              visitors
            }
            events {
              eventName
              views
              visitors
            }
          }
        }
      }
    `;
    const variables = { projectFullPath };

    const res: { project: Project } = await this.apiClient.query(query, variables);
    return res.project;
  }

  async fetchLists(projectFullPath: string): Promise<Project> {
    const query = `
      query($projectFullPath: String!) {
        project(fullPath: $projectFullPath) {
          id
          name

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

  async fetchList(listId: string): Promise<List> {
    const query = `
      query($listId: ID!) {
        list(listId: $listId) {
          id
          createdAt
          name
          description

          contacts {
            id
            name
          }
        }
      }
    `;
    const variables = { listId };

    const res: { list: List } = await this.apiClient.query(query, variables);
    return res.list;
  }

  async createList(input: CreateListInput): Promise<void> {
    const query = `
    mutation($input: CreateListInput!) {
      createList(input: $input) {
        id
      }
    }
    `;
    const variables = { input };

    const res: { createList: List } = await this.apiClient.query(query, variables);
    this.router.push({ path: `/${input.projectFullPath}/-/lists/${res.createList.id}` });
  }

  async updateList(input: UpdateListInput): Promise<List> {
    const query = `
    mutation($input: UpdateListInput!) {
      updateList(input: $input) {
        id
        createdAt
        name
        description

        contacts {
          id
          name
        }
      }
    }
    `;
    const variables = { input };

    const res: { updateList: List } = await this.apiClient.query(query, variables);
    return res.updateList;
  }

  async deleteList(projectFullPath: string, input: DeleteListInput): Promise<void> {
    const query = `
    mutation($input: DeleteListInput!) {
        deleteList(input: $input)
      }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
    this.router.push({ path: `/${projectFullPath}/-/lists` });
  }

  async confirmListSubscription(input: ConfirmListSubscriptionInput): Promise<void> {
    const query = `
    mutation($input: ConfirmListSubscriptionInput!) {
      confirmListSubscription(input: $input)
    }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
  }

  async unsubscribeFromList(input: UnsubscribeFromListInput): Promise<void> {
    const query = `
    mutation($input: UnsubscribeFromListInput!) {
      unsubscribeFromList(input: $input)
    }
    `;
    const variables = { input };

    await this.apiClient.query(query, variables);
  }
}

export const GrowthServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: GrowthService) {
    Vue.prototype.$growthService = service;
  },
};
