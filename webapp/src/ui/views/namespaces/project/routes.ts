import { RouteConfig } from 'vue-router';

const Tickets = () => import(/* webpackChunkName: "chunk-projects" */ './tickets/tickets.vue');
const NewTicket = () => import(/* webpackChunkName: "chunk-projects" */ './tickets/new.vue');
const Ticket = () => import(/* webpackChunkName: "chunk-projects" */ './tickets/ticket.vue');

const Labels = () => import(/* webpackChunkName: "chunk-projects" */ './labels/labels.vue');
const NewLabel = () => import(/* webpackChunkName: "chunk-projects" */ './labels/new.vue');
const EditLabel = () => import(/* webpackChunkName: "chunk-projects" */ './labels/edit.vue');

const Monitors = () => import(/* webpackChunkName: "chunk-projects-operations" */ './monitors/monitors.vue');
const NewMonitor = () => import(/* webpackChunkName: "chunk-projects-operations" */ './monitors/new.vue');
const Monitor = () => import(/* webpackChunkName: "chunk-projects-operations" */ './monitors/monitor.vue');


const Milestones = () => import(/* webpackChunkName: "chunk-projects" */ './milestones/milestones.vue');
const NewMilestone = () => import(/* webpackChunkName: "chunk-projects" */ './milestones/new.vue');
const Milestone = () => import(/* webpackChunkName: "chunk-projects" */ './milestones/milestone.vue');

const Preferences = () => import(/* webpackChunkName: "chunk-projects-preferences" */ './preferences/preferences.vue');


const routes: Array<RouteConfig> = [
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
];

export default routes;
