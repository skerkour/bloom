import { Route, RouteConfig } from 'vue-router';

const Inbox = () => import(/* webpackChunkName: "chunk-projects" */ './inbox/inbox.vue');

const Contacts = () => import(/* webpackChunkName: "chunk-projects" */ './contacts/contacts.vue');
const Contact = () => import(/* webpackChunkName: "chunk-projects" */ './contacts/contact.vue');
const NewContact = () => import(/* webpackChunkName: "chunk-projects" */ './contacts/new.vue');

const Tickets = () => import(/* webpackChunkName: "chunk-projects" */ './tickets/tickets.vue');
const NewTicket = () => import(/* webpackChunkName: "chunk-projects" */ './tickets/new.vue');
const Ticket = () => import(/* webpackChunkName: "chunk-projects" */ './tickets/ticket.vue');

const Lists = () => import(/* webpackChunkName: "chunk-projects" */ './lists/lists.vue');
const NewList = () => import(/* webpackChunkName: "chunk-projects" */ './lists/new.vue');
const List = () => import(/* webpackChunkName: "chunk-projects" */ './lists/list.vue');

const OutboundMessages = () => import(/* webpackChunkName: "chunk-projects" */ './outbound/outbound.vue');
const NewOutboundMessage = () => import(/* webpackChunkName: "chunk-projects" */ './outbound/new.vue');
const OutboundMessage = () => import(/* webpackChunkName: "chunk-projects" */ './outbound/message.vue');

const Labels = () => import(/* webpackChunkName: "chunk-projects" */ './labels/labels.vue');
const NewLabel = () => import(/* webpackChunkName: "chunk-projects" */ './labels/new.vue');
const EditLabel = () => import(/* webpackChunkName: "chunk-projects" */ './labels/edit.vue');

const Files = () => import(/* webpackChunkName: "chunk-projects" */ './files/files.vue');
const Trash = () => import(/* webpackChunkName: "chunk-projects" */ './files/trash.vue');

const Analytics = () => import(/* webpackChunkName: "chunk-projects-analytics" */ './analytics/analytics.vue');

const Monitors = () => import(/* webpackChunkName: "chunk-projects-operations" */ './monitors/monitors.vue');
const NewMonitor = () => import(/* webpackChunkName: "chunk-projects-operations" */ './monitors/new.vue');
const Monitor = () => import(/* webpackChunkName: "chunk-projects-operations" */ './monitors/monitor.vue');


const Milestones = () => import(/* webpackChunkName: "chunk-projects" */ './milestones/milestones.vue');
const NewMilestone = () => import(/* webpackChunkName: "chunk-projects" */ './milestones/new.vue');
const Milestone = () => import(/* webpackChunkName: "chunk-projects" */ './milestones/milestone.vue');

const Preferences = () => import(/* webpackChunkName: "chunk-projects-preferences" */ './preferences/preferences.vue');
const PreferencesInbox = () => import(/* webpackChunkName: "chunk-projects-preferences" */ './preferences/inbox.vue');


const routes: Array<RouteConfig> = [
  {
    path: '/:namespacePath/:projectPath',
    redirect: (to: Route) => `/${to.params.namespacePath}/${to.params.projectPath}/-/inbox`,
  },

  {
    path: '/:namespacePath/:projectPath/-/tickets',
    component: Tickets,
    meta: {
      projectDrawer: true,
    },
  },
  {
    path: '/:namespacePath/:projectPath/-/tickets/new',
    component: NewTicket,
    meta: {
      projectDrawer: true,
    },
  },
  {
    path: '/:namespacePath/:projectPath/-/tickets/:ticketId',
    component: Ticket,
    meta: {
      projectDrawer: true,
    },
  },

  {
    path: '/:namespacePath/:projectPath/-/milestones',
    component: Milestones,
    meta: {
      projectDrawer: true,
    },
  },
  {
    path: '/:namespacePath/:projectPath/-/milestones/new',
    component: NewMilestone,
    meta: {
      projectDrawer: true,
    },
  },
  {
    path: '/:namespacePath/:projectPath/-/milestones/:milestoneId',
    component: Milestone,
    meta: {
      projectDrawer: true,
    },
  },

  {
    path: '/:namespacePath/:projectPath/-/contacts',
    component: Contacts,
    meta: {
      projectDrawer: true,
    },
  },
  {
    path: '/:namespacePath/:projectPath/-/contacts/new',
    component: NewContact,
    meta: {
      projectDrawer: true,
    },
  },
  {
    path: '/:namespacePath/:projectPath/-/contacts/:contactId',
    component: Contact,
    meta: {
      projectDrawer: true,
    },
  },

  {
    path: '/:namespacePath/:projectPath/-/labels',
    component: Labels,
    meta: {
      projectDrawer: true,
    },
  },
  {
    path: '/:namespacePath/:projectPath/-/labels/new',
    component: NewLabel,
    meta: {
      projectDrawer: true,
    },
  },
  {
    path: '/:namespacePath/:projectPath/-/labels/:labelId',
    component: EditLabel,
    meta: {
      projectDrawer: true,
    },
  },

  {
    path: '/:namespacePath/:projectPath/-/files',
    component: Files,
    meta: {
      projectDrawer: true,
    },
  },
  {
    path: '/:namespacePath/:projectPath/-/files/trash',
    component: Trash,
    meta: {
      projectDrawer: true,
    },
  },
  {
    path: '/:namespacePath/:projectPath/-/files/:fileId',
    component: Files,
    meta: {
      projectDrawer: true,
    },
  },

  {
    path: '/:namespacePath/:projectPath/-/outbound',
    component: OutboundMessages,
    meta: {
      projectDrawer: true,
    },
  },
  {
    path: '/:namespacePath/:projectPath/-/outbound/new',
    component: NewOutboundMessage,
    meta: {
      projectDrawer: true,
    },
  },
  {
    path: '/:namespacePath/:projectPath/-/outbound/:messageId',
    component: OutboundMessage,
    meta: {
      projectDrawer: true,
    },
  },

  {
    path: '/:namespacePath/:projectPath/-/lists',
    component: Lists,
    meta: {
      projectDrawer: true,
    },
  },
  {
    path: '/:namespacePath/:projectPath/-/lists/new',
    component: NewList,
    meta: {
      projectDrawer: true,
    },
  },
  {
    path: '/:namespacePath/:projectPath/-/lists/:listId',
    component: List,
    meta: {
      projectDrawer: true,
    },
  },

  {
    path: '/:namespacePath/:projectPath/-/inbox',
    component: Inbox,
    meta: {
      projectDrawer: true,
    },
  },

  {
    path: '/:namespacePath/:projectPath/-/analytics',
    component: Analytics,
    meta: {
      projectDrawer: true,
    },
  },

  {
    path: '/:namespacePath/:projectPath/-/monitors',
    component: Monitors,
    meta: {
      projectDrawer: true,
    },
  },
  {
    path: '/:namespacePath/:projectPath/-/monitors/new',
    component: NewMonitor,
    meta: {
      projectDrawer: true,
    },
  },
  {
    path: '/:namespacePath/:projectPath/-/monitors/:monitorId',
    component: Monitor,
    meta: {
      projectDrawer: true,
    },
  },

  {
    path: '/:namespacePath/:projectPath/-/preferences',
    component: Preferences,
    meta: {
      projectDrawer: true,
    },
  },
  {
    path: '/:namespacePath/:projectPath/-/preferences/inbox',
    component: PreferencesInbox,
    meta: {
      projectDrawer: true,
    },
  },
];

export default routes;
