import { RouteConfig } from 'vue-router';
import Index from './index.vue';
import PageNotFound from './page_not_found.vue';
import ConfirmListSubscriptionView from './confirm_list_subscription.vue';
import UnsubscribeView from './unsubscribe.vue';
import PrivacyView from './privacy.vue';
import ContactView from './contact.vue';
import SecurityView from './security.vue';
import AboutView from './about.vue';
import TermsView from './terms.vue';
import FaqView from './faq.vue';
import LicensingView from './licensing.vue';
import PricingView from './pricing.vue';
import VerifyEmailView from './verify_email.vue';
import FeaturesView from './features.vue';
import AppsView from './apps.vue';

import LoginRoutes from './login/routes';
import RegisterRoutes from './register/routes';
import PreferencesRoutes from './preferences/routes';
import GroupsRoutes from './groups/routes';
import NamespacesRoutes from './namespaces/routes';
import ToolsRoutes from './tools/routes';
import AdminRoutes from './admin/routes';
import InboxRoutes from './inbox/routes';
import AnalyticsRoutes from './analytics/routes';
import NewsletterRoutes from './newsletter/routes';
import FilesRoutes from './files/routes';
import CalendarRoutes from './calendar/routes';

const StatusView = () => import(/* webpackChunkName: "chunk-projects-operations" */ './status.vue');

const routes: Array<RouteConfig> = [
  {
    path: '/',
    component: Index,
    meta: {
      auth: false,
    },
  },

  {
    path: '/confirm-list-subscription/:subscriptionId',
    component: ConfirmListSubscriptionView,
    meta: {
      auth: false,
    },
  },
  {
    path: '/unsubscribe',
    component: UnsubscribeView,
    meta: {
      auth: false,
    },
  },

  {
    path: '/verify-email/:token*',
    component: VerifyEmailView,
  },

  {
    path: '/status/:projectFullPath*',
    component: StatusView,
    meta: {
      layout: 'bare',
      auth: false,
    },
  },

  {
    path: '/privacy',
    component: PrivacyView,
    meta: { auth: false },
  },
  {
    path: '/contact',
    component: ContactView,
    meta: { auth: false },
  },
  {
    path: '/security',
    component: SecurityView,
    meta: { auth: false },
  },
  {
    path: '/about',
    component: AboutView,
    meta: { auth: false },
  },
  {
    path: '/terms',
    component: TermsView,
    meta: { auth: false },
  },
  {
    path: '/faq',
    component: FaqView,
    meta: { auth: false },
  },
  {
    path: '/licensing',
    component: LicensingView,
    meta: { auth: false },
  },
  {
    path: '/pricing',
    component: PricingView,
    meta: { auth: false },
  },
  {
    path: '/features',
    component: FeaturesView,
    meta: { auth: false },
  },

  {
    path: '/features/*',
    redirect: '/features',
    meta: { auth: false },
  },

  {
    path: '/apps',
    component: AppsView,
  },

  ...LoginRoutes,
  ...AdminRoutes,
  ...RegisterRoutes,
  ...PreferencesRoutes,
  ...ToolsRoutes,
  ...InboxRoutes,
  ...NewsletterRoutes,
  ...FilesRoutes,
  ...CalendarRoutes,
  ...AnalyticsRoutes,
  ...GroupsRoutes,
  ...NamespacesRoutes,

  {
    path: '*',
    component: PageNotFound,
    meta: { auth: false },
  },
];

export default routes;
