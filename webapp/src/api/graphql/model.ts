/* eslint-disable */
export type Maybe<T> = T | null;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in keyof Pick<T, K>]?: Maybe<Pick<T, K>[SubKey]> };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
  Time: any;
  Bytes: any;
  Int64: any;
  Any: any;
  Upload: any;
  StringMap: any;
};







export enum TwoFaMethod {
  Totp = 'totp'
}

export enum ConversationType {
  Chatbox = 'chatbox',
  Messenger = 'messenger',
  Email = 'email',
  Telegram = 'telegram',
  Whatsapp = 'whatsapp'
}

export enum OutboundMessageStatus {
  Saved = 'saved',
  Programmed = 'programmed',
  Sending = 'sending',
  Sent = 'sent'
}

export enum OutboundMessageType {
  Standard = 'standard',
  Automated = 'automated'
}

export enum MonitorType {
  Https = 'https'
}

export enum MonitorHttpMethod {
  Get = 'GET',
  Head = 'HEAD',
  Post = 'POST',
  Put = 'PUT',
  Delete = 'DELETE',
  Patch = 'PATCH',
  Options = 'OPTIONS'
}

export enum BillingPlan {
  Free = 'free',
  Starter = 'starter',
  Pro = 'pro',
  Ultra = 'ultra'
}

export enum MonitorStatus {
  Operational = 'operational',
  DegradedPerformance = 'degraded_performance',
  PartialOutage = 'partial_outage',
  MajorOutage = 'major_outage',
  UnderMaintenance = 'under_maintenance',
  Unknown = 'unknown'
}

export enum GroupRole {
  Administrator = 'administrator',
  Member = 'member'
}

export type Namespace = {
  id?: Maybe<Scalars['ID']>;
  createdAt: Scalars['Time'];
  name: Scalars['String'];
  path: Scalars['String'];
  avatarUrl: Scalars['String'];
  description: Scalars['String'];
  projects: Array<Project>;
  billing?: Maybe<BillingInformation>;
};

export enum BotExecutionStatus {
  Running = 'running',
  Success = 'success',
  Errored = 'errored'
}

export type User = Namespace & {
  __typename?: 'User';
  id?: Maybe<Scalars['ID']>;
  createdAt: Scalars['Time'];
  name: Scalars['String'];
  path: Scalars['String'];
  avatarUrl: Scalars['String'];
  description: Scalars['String'];
  username: Scalars['String'];
  email?: Maybe<Scalars['String']>;
  isAdmin: Scalars['Boolean'];
  disabledAt?: Maybe<Scalars['Time']>;
  twoFAEnabled?: Maybe<Scalars['Boolean']>;
  groups: Array<Group>;
  projects: Array<Project>;
  billing?: Maybe<BillingInformation>;
  sessions: Array<Session>;
  invitations: Array<GroupInvitation>;
};

export type Session = {
  __typename?: 'Session';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  token?: Maybe<Scalars['String']>;
};

export type SignedIn = {
  __typename?: 'SignedIn';
  session?: Maybe<Session>;
  me?: Maybe<User>;
  twoFaMethod?: Maybe<TwoFaMethod>;
};

export type RegistrationStarted = {
  __typename?: 'RegistrationStarted';
  pendingUserId: Scalars['ID'];
};

export type SignInStarted = {
  __typename?: 'SignInStarted';
  pendingSessionId: Scalars['ID'];
};

export type Group = Namespace & {
  __typename?: 'Group';
  id?: Maybe<Scalars['ID']>;
  createdAt: Scalars['Time'];
  name: Scalars['String'];
  path: Scalars['String'];
  avatarUrl: Scalars['String'];
  description: Scalars['String'];
  projects: Array<Project>;
  billing?: Maybe<BillingInformation>;
  members: Array<GroupMember>;
  invitations: Array<GroupInvitation>;
};

export type GroupMember = {
  __typename?: 'GroupMember';
  username: Scalars['String'];
  name: Scalars['String'];
  avatarUrl: Scalars['String'];
  role: GroupRole;
};

export type GroupInvitation = {
  __typename?: 'GroupInvitation';
  id: Scalars['ID'];
  group: Group;
  inviter: User;
  invitee: User;
};

export type Project = {
  __typename?: 'Project';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  avatarUrl: Scalars['String'];
  path: Scalars['String'];
  name: Scalars['String'];
  description: Scalars['String'];
  twitterUrl: Scalars['String'];
  facebookUrl: Scalars['String'];
  publicEmail: Scalars['String'];
  instagramUrl: Scalars['String'];
  whatsappNumber: Scalars['String'];
  mastodonUrl: Scalars['String'];
  homepageUrl: Scalars['String'];
  contacts: Array<Contact>;
  conversations: Array<InboxConversation>;
  chatboxPreferences: ChatboxPreferences;
  tickets: Array<Ticket>;
  outboundMessages: Array<OutboundMessage>;
  labels: Array<Label>;
  analytics: Analytics;
  monitors: Array<Monitor>;
  lists: Array<List>;
  milestones: Array<Milestone>;
  bots: Array<Bot>;
  botsHistory: Array<BotExecution>;
  botsConnections: Array<BotConnection>;
};

export type InboxConversation = {
  __typename?: 'InboxConversation';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  lastMessageReceivedAt: Scalars['Time'];
  contact: Contact;
  messages: Array<InboxMessage>;
};

export type InboxMessage = {
  __typename?: 'InboxMessage';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  conversationId: Scalars['ID'];
  author?: Maybe<User>;
  bodyHtml: Scalars['String'];
};

/** ChatboxMessage is sent to anonymous users using a chatbox */
export type ChatboxMessage = {
  __typename?: 'ChatboxMessage';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  author?: Maybe<ChatboxAgent>;
  bodyHtml: Scalars['String'];
};

export type ChatboxAgent = {
  __typename?: 'ChatboxAgent';
  name: Scalars['String'];
  avatarUrl: Scalars['String'];
};

export type Chatbox = {
  __typename?: 'Chatbox';
  preferences: ChatboxPreferences;
  messages: Array<ChatboxMessage>;
};

export type ChatboxPreferences = {
  __typename?: 'ChatboxPreferences';
  color: Scalars['String'];
  name: Scalars['String'];
  avatarUrl: Scalars['String'];
  twitterUrl: Scalars['String'];
  facebookUrl: Scalars['String'];
  publicEmail: Scalars['String'];
  instagramUrl: Scalars['String'];
  whatsappNumber: Scalars['String'];
  mastodonUrl: Scalars['String'];
  homepageUrl: Scalars['String'];
  branding: Scalars['Boolean'];
  welcomeMessage: Scalars['String'];
};

export type Contact = {
  __typename?: 'Contact';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  avatarUrl: Scalars['String'];
  name: Scalars['String'];
  email: Scalars['String'];
  pgpKey: Scalars['String'];
  phone: Scalars['String'];
  address: Scalars['String'];
  website: Scalars['String'];
  twitter: Scalars['String'];
  instagram: Scalars['String'];
  facebook: Scalars['String'];
  linkedin: Scalars['String'];
  skype: Scalars['String'];
  telegram: Scalars['String'];
  notes: Scalars['String'];
  country: Scalars['String'];
  countryCode: Scalars['String'];
  plan: Scalars['String'];
  userId: Scalars['String'];
  labels: Array<Label>;
  lists: Array<List>;
};

export type Ticket = {
  __typename?: 'Ticket';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  updatedAt: Scalars['Time'];
  title: Scalars['String'];
  body: Scalars['String'];
  bodyHtml: Scalars['String'];
  dueDate?: Maybe<Scalars['Time']>;
  lastEditedAt: Scalars['Time'];
  closedAt?: Maybe<Scalars['Time']>;
  commentsCount: Scalars['Int64'];
  author: TicketAuthor;
  comments: Array<TicketComment>;
  labels: Array<Label>;
  milestones: Array<Milestone>;
};

export type TicketAuthor = {
  __typename?: 'TicketAuthor';
  id: Scalars['ID'];
  username: Scalars['String'];
  name: Scalars['String'];
  avatarUrl: Scalars['String'];
};

export type TicketComment = {
  __typename?: 'TicketComment';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  lastEditedAt: Scalars['Time'];
  body: Scalars['String'];
  bodyHtml: Scalars['String'];
  author: TicketAuthor;
};

export type Label = {
  __typename?: 'Label';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  name: Scalars['String'];
  description: Scalars['String'];
  backgroundColor: Scalars['String'];
  textColor: Scalars['String'];
};

export type File = {
  __typename?: 'File';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  name: Scalars['String'];
  size: Scalars['Int64'];
  type: Scalars['String'];
  trashedAt?: Maybe<Scalars['Time']>;
  url: Scalars['String'];
  path: Array<FilePath>;
  children: Array<File>;
};

export type FilePath = {
  __typename?: 'FilePath';
  id: Scalars['ID'];
  name: Scalars['String'];
};

export type OutboundMessage = {
  __typename?: 'OutboundMessage';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  name: Scalars['String'];
  fromName: Scalars['String'];
  fromAddress: Scalars['String'];
  subject: Scalars['String'];
  body: Scalars['String'];
  bodyHtml: Scalars['String'];
  status: OutboundMessageStatus;
  sendAt?: Maybe<Scalars['Time']>;
  lastSentAt?: Maybe<Scalars['Time']>;
  sentCount: Scalars['Int64'];
  errorCount: Scalars['Int64'];
  type: OutboundMessageType;
  lists: Array<List>;
};

export type List = {
  __typename?: 'List';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  name: Scalars['String'];
  description: Scalars['String'];
  contacts: Array<Contact>;
};

export type Analytics = {
  __typename?: 'Analytics';
  visits: Array<AnalyticsVisit>;
  pages: Array<AnalyticsPage>;
  referrers: Array<AnalyticsReferrer>;
  devices: Array<AnalyticsDevice>;
  browsers: Array<AnalyticsBrowser>;
  countries: Array<AnalyticsCountry>;
  oses: Array<AnalyticsOs>;
  events: Array<AnalyticsEvent>;
};

export type AnalyticsVisit = {
  __typename?: 'AnalyticsVisit';
  date: Scalars['String'];
  views: Scalars['Int64'];
  visitors: Scalars['Int64'];
};

export type AnalyticsPage = {
  __typename?: 'AnalyticsPage';
  url: Scalars['String'];
  path: Scalars['String'];
  views: Scalars['Int64'];
  visitors: Scalars['Int64'];
};

export type AnalyticsReferrer = {
  __typename?: 'AnalyticsReferrer';
  referrer: Scalars['String'];
  views: Scalars['Int64'];
  visitors: Scalars['Int64'];
};

export type AnalyticsDevice = {
  __typename?: 'AnalyticsDevice';
  deviceType: Scalars['String'];
  views: Scalars['Int64'];
  visitors: Scalars['Int64'];
};

export type AnalyticsBrowser = {
  __typename?: 'AnalyticsBrowser';
  browser: Scalars['String'];
  views: Scalars['Int64'];
  visitors: Scalars['Int64'];
};

export type AnalyticsCountry = {
  __typename?: 'AnalyticsCountry';
  country: Scalars['String'];
  views: Scalars['Int64'];
  visitors: Scalars['Int64'];
};

export type AnalyticsEvent = {
  __typename?: 'AnalyticsEvent';
  eventName: Scalars['String'];
  views: Scalars['Int64'];
  visitors: Scalars['Int64'];
};

export type AnalyticsOs = {
  __typename?: 'AnalyticsOs';
  os: Scalars['String'];
  views: Scalars['Int64'];
  visitors: Scalars['Int64'];
};

export type StatusPage = {
  __typename?: 'StatusPage';
  name: Scalars['String'];
  avatarUrl: Scalars['String'];
  twitterUrl: Scalars['String'];
  facebookUrl: Scalars['String'];
  publicEmail: Scalars['String'];
  instagramUrl: Scalars['String'];
  whatsappNumber: Scalars['String'];
  mastodonUrl: Scalars['String'];
  homepageUrl: Scalars['String'];
  monitors: Array<StatusPageMonitor>;
};

export type StatusPageMonitor = {
  __typename?: 'StatusPageMonitor';
  name: Scalars['String'];
  status: MonitorStatus;
};

export type Monitor = {
  __typename?: 'Monitor';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  name: Scalars['String'];
  status: MonitorStatus;
  endpoint: Scalars['String'];
  type: MonitorType;
  httpMethod: MonitorHttpMethod;
  bodyTextToMatch: Scalars['String'];
  minHTTPStatusCode: Scalars['Int'];
  maxHTTPStatusCode: Scalars['Int'];
  followHTTPRedirects: Scalars['Boolean'];
  isActive: Scalars['Boolean'];
  showOnStatusPage: Scalars['Boolean'];
  pings: Array<Ping>;
  statusChanges: Array<MonitorStatusChange>;
  uptime: Array<MonitorUptime>;
};

export type MonitorUptime = {
  __typename?: 'MonitorUptime';
  date: Scalars['String'];
  uptime: Scalars['Int64'];
};

export type Ping = {
  __typename?: 'Ping';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  startedAt: Scalars['Time'];
  dnsResolution: Scalars['Int64'];
  tcpConnection: Scalars['Int64'];
  tlsHandshake: Scalars['Int64'];
  serverProcessing: Scalars['Int64'];
  contentTransfer: Scalars['Int64'];
  timeToFirstByte: Scalars['Int64'];
  totalDuration: Scalars['Int64'];
  contentSize: Scalars['Int64'];
  tlsCertificateIsValid: Scalars['Boolean'];
  tlsCertificateValidFrom: Scalars['Time'];
  tlsCertificateExpirationDate: Scalars['Time'];
  tlsCertificateDomains: Array<Scalars['String']>;
  tlsCertificateIssuedBy: Scalars['String'];
  error?: Maybe<Scalars['String']>;
};

export type MonitorStatusChange = {
  __typename?: 'MonitorStatusChange';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  from: MonitorStatus;
  to: MonitorStatus;
};

export type ListSubscription = {
  __typename?: 'ListSubscription';
  id: Scalars['ID'];
  name: Scalars['String'];
};

export type Milestone = {
  __typename?: 'Milestone';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  updatedAt: Scalars['Time'];
  title: Scalars['String'];
  description: Scalars['String'];
  descriptionHtml: Scalars['String'];
  startDate?: Maybe<Scalars['Time']>;
  dueDate?: Maybe<Scalars['Time']>;
  closedAt?: Maybe<Scalars['Time']>;
  tickets: Array<Ticket>;
};

export type BillingInformation = {
  __typename?: 'BillingInformation';
  plan: Scalars['String'];
  name: Scalars['String'];
  email: Scalars['String'];
  country: Scalars['String'];
  countryCode: Scalars['String'];
  city: Scalars['String'];
  postalCode: Scalars['String'];
  addressLine1: Scalars['String'];
  addressLine2: Scalars['String'];
  state: Scalars['String'];
  taxId?: Maybe<Scalars['String']>;
  usedStorage: Scalars['Int64'];
  totalStorage: Scalars['Int64'];
};

export type SignedStorageUploadUrl = {
  __typename?: 'SignedStorageUploadUrl';
  url: Scalars['String'];
  tmpKey: Scalars['String'];
  size: Scalars['Int64'];
};

export type Bot = {
  __typename?: 'Bot';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  updatedAt: Scalars['Time'];
  name: Scalars['String'];
  description: Scalars['String'];
  lastExecutedAt?: Maybe<Scalars['Time']>;
  active: Scalars['Boolean'];
  history: Array<BotExecution>;
};

export type BotExecution = {
  __typename?: 'BotExecution';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  completedAt?: Maybe<Scalars['Time']>;
  status: BotExecutionStatus;
  bot: Bot;
};

export type BotConnection = {
  __typename?: 'BotConnection';
  id: Scalars['ID'];
  createdAt: Scalars['Time'];
  name: Scalars['String'];
  description: Scalars['String'];
  app: BotApp;
};

export type BotApp = {
  __typename?: 'BotApp';
  id: Scalars['String'];
  name: Scalars['String'];
  description: Scalars['String'];
  avatarUrl: Scalars['String'];
};

export type Query = {
  __typename?: 'Query';
  /** Get information about current user */
  me: User;
  group: Group;
  namespace: Namespace;
  customerPortalUrl: Scalars['String'];
  checkoutSession: Scalars['String'];
  project: Project;
  contact: Contact;
  ticket: Ticket;
  label: Label;
  markdown: Scalars['String'];
  file: File;
  trash: Array<File>;
  milestone: Milestone;
  outboundMessage: OutboundMessage;
  list: List;
  statusPage: StatusPage;
  monitor: Monitor;
  signedStorageUploadUrl: SignedStorageUploadUrl;
  botsApps: Array<BotApp>;
  bot: Bot;
  botConnection: BotConnection;
  hexdump: Scalars['String'];
  qrCode: Scalars['String'];
  adminUsers: Array<User>;
  adminUser: User;
  adminGroups: Array<Group>;
  adminGroup: Group;
  adminProjects: Array<Project>;
  adminProject: Project;
  chatbox: Chatbox;
  contactSubscriptions: Array<ListSubscription>;
  stripePublicKey: Scalars['String'];
  baseUrl: Scalars['String'];
};


export type QueryGroupArgs = {
  fullPath: Scalars['String'];
};


export type QueryNamespaceArgs = {
  fullPath: Scalars['String'];
};


export type QueryCustomerPortalUrlArgs = {
  input: CustomerPortalUrlInput;
};


export type QueryCheckoutSessionArgs = {
  input: CheckoutSessionInput;
};


export type QueryProjectArgs = {
  fullPath: Scalars['String'];
};


export type QueryContactArgs = {
  contactId: Scalars['ID'];
};


export type QueryTicketArgs = {
  ticketId: Scalars['ID'];
};


export type QueryLabelArgs = {
  labelId: Scalars['ID'];
};


export type QueryMarkdownArgs = {
  input: Scalars['String'];
};


export type QueryFileArgs = {
  projectFullPath: Scalars['String'];
  fileId?: Maybe<Scalars['ID']>;
};


export type QueryTrashArgs = {
  projectFullPath: Scalars['String'];
};


export type QueryMilestoneArgs = {
  milestoneId: Scalars['ID'];
};


export type QueryOutboundMessageArgs = {
  messageId: Scalars['ID'];
};


export type QueryListArgs = {
  listId: Scalars['ID'];
};


export type QueryStatusPageArgs = {
  projectFullPath: Scalars['String'];
};


export type QueryMonitorArgs = {
  monitorId: Scalars['ID'];
};


export type QuerySignedStorageUploadUrlArgs = {
  fileSize: Scalars['Int64'];
};


export type QueryBotArgs = {
  botId: Scalars['ID'];
};


export type QueryBotConnectionArgs = {
  botId: Scalars['ID'];
};


export type QueryHexdumpArgs = {
  file: Scalars['Upload'];
};


export type QueryQrCodeArgs = {
  input: Scalars['String'];
};


export type QueryAdminUserArgs = {
  username: Scalars['String'];
};


export type QueryAdminGroupArgs = {
  groupId: Scalars['ID'];
};


export type QueryAdminProjectArgs = {
  projectId: Scalars['ID'];
};


export type QueryChatboxArgs = {
  projectId: Scalars['ID'];
};


export type QueryContactSubscriptionsArgs = {
  contactID: Scalars['ID'];
};

export type RegisterInput = {
  username: Scalars['String'];
  email: Scalars['String'];
};

export type CompleteRegistrationInput = {
  pendingUserId: Scalars['ID'];
  code: Scalars['String'];
};

export type SignInInput = {
  emailOrUsername: Scalars['String'];
};

export type CompleteSignInInput = {
  pendingSessionId: Scalars['ID'];
  code: Scalars['String'];
};

export type RevokeSessionInput = {
  sessionId: Scalars['ID'];
};

export type CreateGroupInput = {
  name: Scalars['String'];
  path: Scalars['String'];
  description: Scalars['String'];
};

export type CreateProjectInput = {
  namespacePath: Scalars['String'];
  name: Scalars['String'];
  path: Scalars['String'];
  description: Scalars['String'];
};

export type UpdateProjectInput = {
  projectId: Scalars['ID'];
  name?: Maybe<Scalars['String']>;
  path?: Maybe<Scalars['String']>;
  description?: Maybe<Scalars['String']>;
  twitterUrl?: Maybe<Scalars['String']>;
  facebookUrl?: Maybe<Scalars['String']>;
  publicEmail?: Maybe<Scalars['String']>;
  instagramUrl?: Maybe<Scalars['String']>;
  whatsappNumber?: Maybe<Scalars['String']>;
  mastodonUrl?: Maybe<Scalars['String']>;
  homepageUrl?: Maybe<Scalars['String']>;
  avatar?: Maybe<Scalars['Upload']>;
};

export type DeleteProjectInput = {
  fullPath: Scalars['String'];
};

export type DeleteGroupInput = {
  fullPath: Scalars['String'];
};

export type SendChatboxMessageInput = {
  projectId: Scalars['ID'];
  body: Scalars['String'];
};

export type SendMessageToConversationInput = {
  conversationId: Scalars['ID'];
  body: Scalars['String'];
};

export type UpdateChatboxPreferencesInput = {
  projectFullPath: Scalars['String'];
  color?: Maybe<Scalars['String']>;
  name?: Maybe<Scalars['String']>;
  branding?: Maybe<Scalars['Boolean']>;
  welcomeMessage?: Maybe<Scalars['String']>;
};

export type CreateTicketInput = {
  projectFullPath: Scalars['String'];
  title: Scalars['String'];
  body: Scalars['String'];
  labels: Array<Scalars['ID']>;
  milestones: Array<Scalars['ID']>;
};

export type CloseTicketInput = {
  ticketId: Scalars['ID'];
};

export type UpdateTicketInput = {
  ticketId: Scalars['ID'];
  title?: Maybe<Scalars['String']>;
  body?: Maybe<Scalars['String']>;
  labels?: Maybe<Array<Scalars['ID']>>;
  milestones?: Maybe<Array<Scalars['ID']>>;
};

export type DeleteTicketInput = {
  id: Scalars['ID'];
};

export type CreateLabelInput = {
  projectFullPath: Scalars['String'];
  name: Scalars['String'];
  description: Scalars['String'];
  backgroundColor: Scalars['String'];
};

export type UpdateLabelInput = {
  labelId: Scalars['ID'];
  name: Scalars['String'];
  description: Scalars['String'];
  backgroundColor: Scalars['String'];
};

export type DeleteLabelInput = {
  labelId: Scalars['ID'];
};

export type CommentTicketInput = {
  ticketId: Scalars['ID'];
  body: Scalars['String'];
};

export type ReopenTicketInput = {
  ticketId: Scalars['ID'];
};

export type DeleteTicketCommentInput = {
  commentId: Scalars['ID'];
};

export type UpdateTicketCommentInput = {
  commentId: Scalars['ID'];
  body: Scalars['String'];
};

export type MoveFilesToTrashInput = {
  files: Array<Scalars['ID']>;
};

export type RestoreFilesFromTrashInput = {
  files: Array<Scalars['ID']>;
};

export type EmptyTrashInput = {
  projectFullPath: Scalars['String'];
};

export type MoveFilesInput = {
  files: Array<Scalars['ID']>;
  destination: Scalars['ID'];
};

export type CreateFolderInput = {
  parentId: Scalars['ID'];
  name: Scalars['String'];
};

export type RenameFileInput = {
  fileId: Scalars['ID'];
  name: Scalars['String'];
};

export type CompleteFileUploadInput = {
  parentId: Scalars['ID'];
  name: Scalars['String'];
  size: Scalars['Int64'];
  mimeType: Scalars['String'];
  tmpKey: Scalars['String'];
};

export type DeleteContactInput = {
  contactId: Scalars['ID'];
};

export type UpdateContactInput = {
  contactId: Scalars['ID'];
  name: Scalars['String'];
  email: Scalars['String'];
  pgpKey: Scalars['String'];
  phone: Scalars['String'];
  address: Scalars['String'];
  website: Scalars['String'];
  twitter: Scalars['String'];
  instagram: Scalars['String'];
  facebook: Scalars['String'];
  linkedin: Scalars['String'];
  skype: Scalars['String'];
  telegram: Scalars['String'];
  notes: Scalars['String'];
  countryCode: Scalars['String'];
  plan: Scalars['String'];
  userId: Scalars['String'];
  labels: Array<Scalars['ID']>;
  lists: Array<Scalars['ID']>;
};

export type CreateContactInput = {
  projectFullPath: Scalars['String'];
  name: Scalars['String'];
  email: Scalars['String'];
  pgpKey: Scalars['String'];
  phone: Scalars['String'];
  address: Scalars['String'];
  website: Scalars['String'];
  twitter: Scalars['String'];
  instagram: Scalars['String'];
  facebook: Scalars['String'];
  linkedin: Scalars['String'];
  skype: Scalars['String'];
  telegram: Scalars['String'];
  notes: Scalars['String'];
  countryCode: Scalars['String'];
  plan: Scalars['String'];
  userId: Scalars['String'];
  labels: Array<Scalars['ID']>;
  lists: Array<Scalars['ID']>;
};

export type CreateOutboundMessageInput = {
  projectFullPath: Scalars['String'];
  name: Scalars['String'];
  fromName: Scalars['String'];
  fromAddress: Scalars['String'];
  subject: Scalars['String'];
  body: Scalars['String'];
  sendAt?: Maybe<Scalars['Time']>;
  type: OutboundMessageType;
  lists: Array<Scalars['ID']>;
};

export type UpdateOutboundMessageInput = {
  messageId: Scalars['ID'];
  name: Scalars['String'];
  fromName: Scalars['String'];
  fromAddress: Scalars['String'];
  subject: Scalars['String'];
  body: Scalars['String'];
  sendAt?: Maybe<Scalars['Time']>;
  type: OutboundMessageType;
  lists: Array<Scalars['ID']>;
};

export type DeleteOutboundMessageInput = {
  messageId: Scalars['ID'];
};

export type SendTestOutboundMessageInput = {
  messageId: Scalars['ID'];
};

export type SendOutboundMessageInput = {
  messageId: Scalars['ID'];
};

export type CreateMonitorInput = {
  projectFullPath: Scalars['String'];
  name: Scalars['String'];
  endpoint: Scalars['String'];
  type: MonitorType;
  httpMethod: MonitorHttpMethod;
  bodyTextToMatch: Scalars['String'];
  minHTTPStatusCode: Scalars['Int'];
  maxHTTPStatusCode: Scalars['Int'];
  followHTTPRedirects: Scalars['Boolean'];
  showOnStatusPage: Scalars['Boolean'];
};

export type UpdateMonitorInput = {
  monitorId: Scalars['ID'];
  name?: Maybe<Scalars['String']>;
  endpoint?: Maybe<Scalars['String']>;
  type?: Maybe<MonitorType>;
  httpMethod?: Maybe<MonitorHttpMethod>;
  bodyTextToMatch?: Maybe<Scalars['String']>;
  minHTTPStatusCode?: Maybe<Scalars['Int']>;
  maxHTTPStatusCode?: Maybe<Scalars['Int']>;
  followHTTPRedirects?: Maybe<Scalars['Boolean']>;
  isActive?: Maybe<Scalars['Boolean']>;
  showOnStatusPage?: Maybe<Scalars['Boolean']>;
};

export type DeleteMonitorInput = {
  monitorId: Scalars['ID'];
};

export type CreateListInput = {
  projectFullPath: Scalars['String'];
  name: Scalars['String'];
  description: Scalars['String'];
};

export type UpdateListInput = {
  id: Scalars['ID'];
  name?: Maybe<Scalars['String']>;
  description?: Maybe<Scalars['String']>;
};

export type DeleteListInput = {
  id: Scalars['ID'];
};

export type SubscribeToListInput = {
  id: Scalars['ID'];
  name?: Maybe<Scalars['String']>;
  email: Scalars['String'];
};

export type UnsubscribeFromListInput = {
  id: Scalars['ID'];
  contactId: Scalars['ID'];
};

export type ConfirmListSubscriptionInput = {
  pendingSubscriptionID: Scalars['ID'];
};

export type CreateMilestoneInput = {
  projectFullPath: Scalars['String'];
  title: Scalars['String'];
  description: Scalars['String'];
  startDate?: Maybe<Scalars['Time']>;
  dueDate?: Maybe<Scalars['Time']>;
};

export type UpdateMilestoneInput = {
  id: Scalars['ID'];
  title?: Maybe<Scalars['String']>;
  description?: Maybe<Scalars['String']>;
  startDate?: Maybe<Scalars['Time']>;
  dueDate?: Maybe<Scalars['Time']>;
};

export type CloseMilestoneInput = {
  id: Scalars['ID'];
};

export type ReopenMilestoneInput = {
  id: Scalars['ID'];
};

export type DeleteMilestoneInput = {
  id: Scalars['ID'];
};

export type UpdateBillingInformationInput = {
  namespace: Scalars['String'];
  name: Scalars['String'];
  email: Scalars['String'];
  countryCode: Scalars['String'];
  city: Scalars['String'];
  postalCode: Scalars['String'];
  addressLine1: Scalars['String'];
  addressLine2: Scalars['String'];
  state: Scalars['String'];
  taxId?: Maybe<Scalars['String']>;
};

export type CustomerPortalUrlInput = {
  namespace: Scalars['String'];
};

export type CheckoutSessionInput = {
  namespace: Scalars['String'];
  plan: BillingPlan;
};

export type SyncBillingWithProviderInput = {
  namespace: Scalars['String'];
};

export type ImportContactsInput = {
  projectFullPath: Scalars['String'];
  contacts: Scalars['String'];
};

export type UpdateMyProfileInput = {
  username?: Maybe<Scalars['String']>;
  name?: Maybe<Scalars['String']>;
  description?: Maybe<Scalars['String']>;
  email?: Maybe<Scalars['String']>;
  avatar?: Maybe<Scalars['Upload']>;
};

export type VerifyEmailInput = {
  token: Scalars['String'];
};

export type UpdateGroupProfileInput = {
  id: Scalars['ID'];
  name?: Maybe<Scalars['String']>;
  path?: Maybe<Scalars['String']>;
  description?: Maybe<Scalars['String']>;
  avatar?: Maybe<Scalars['Upload']>;
};

export type InvitePeopleInGroupInput = {
  groupId: Scalars['ID'];
  usernames: Array<Scalars['String']>;
};

export type AcceptGroupInvitationInput = {
  invitationId: Scalars['ID'];
};

export type CancelGroupInvitationInput = {
  invitationId: Scalars['ID'];
};

export type DeclineGroupInvitationInput = {
  invitationId: Scalars['ID'];
};

export type RemoveMemberFromGroupInput = {
  groupId: Scalars['ID'];
  username: Scalars['String'];
};

export type QuitGroupInput = {
  groupId: Scalars['ID'];
};

export type EnableTwoFaInput = {
  code: Scalars['String'];
};

export type DisableTwoFaInput = {
  code: Scalars['String'];
};

export type CompleteTwoFaInput = {
  pendingSessionId: Scalars['ID'];
  code: Scalars['String'];
};

export type DeleteMyAccountInput = {
  code?: Maybe<Scalars['String']>;
};

export type CreateBotInput = {
  projectFullPath: Scalars['String'];
  name: Scalars['String'];
  description: Scalars['String'];
  active: Scalars['Boolean'];
};

export type UpdateBotInput = {
  botId: Scalars['ID'];
  name?: Maybe<Scalars['String']>;
  description?: Maybe<Scalars['String']>;
  active?: Maybe<Scalars['Boolean']>;
};

export type DeleteBotInput = {
  botId: Scalars['ID'];
};

export type CreateBotConnectionInput = {
  projectFullPath: Scalars['String'];
  app: Scalars['String'];
  name: Scalars['String'];
  description: Scalars['String'];
  credentials: Scalars['StringMap'];
};

export type UpdateBotConnectionInput = {
  connectionId: Scalars['ID'];
  name?: Maybe<Scalars['String']>;
  description?: Maybe<Scalars['String']>;
};

export type DeleteBotConnectionInput = {
  connectionId: Scalars['ID'];
};

export type Mutation = {
  __typename?: 'Mutation';
  register: RegistrationStarted;
  completeRegistration: SignedIn;
  signIn: SignInStarted;
  completeSignIn: SignedIn;
  revokeSession: Scalars['Boolean'];
  updateMyProfile: User;
  verifyEmail: Scalars['Boolean'];
  deleteMyAccount: Scalars['Boolean'];
  setupTwoFA: Scalars['String'];
  enableTwoFA: Scalars['Boolean'];
  disableTwoFA: Scalars['Boolean'];
  completeTwoFA: SignedIn;
  createGroup: Group;
  deleteGroup: Scalars['Boolean'];
  updateGroupProfile: Group;
  invitePeopleInGroup: Group;
  acceptGroupInvitation: Group;
  declineGroupInvitation: Scalars['Boolean'];
  cancelGroupInvitation: Scalars['Boolean'];
  quitGroup: Scalars['Boolean'];
  removeMemberFromGroup: Group;
  updateBillingInformation: BillingInformation;
  syncBillingWithProvider: Scalars['Boolean'];
  createProject: Project;
  updateProject: Project;
  deleteProject: Scalars['Boolean'];
  sendMessageToConversation: InboxMessage;
  updateChatboxPreferences: ChatboxPreferences;
  createContact: Contact;
  deleteContact: Scalars['Boolean'];
  updateContact: Contact;
  createTicket: Ticket;
  updateTicket: Ticket;
  closeTicket: Ticket;
  reopenTicket: Ticket;
  deleteTicket: Scalars['Boolean'];
  commentTicket: TicketComment;
  deleteTicketComment: Scalars['Boolean'];
  updateTicketComment: TicketComment;
  createLabel: Label;
  updateLabel: Label;
  deleteLabel: Scalars['Boolean'];
  moveFilesToTrash: Scalars['Boolean'];
  restoreFilesFromTrash: Scalars['Boolean'];
  emptyTrash: Scalars['Boolean'];
  moveFiles: Scalars['Boolean'];
  createFolder: File;
  renameFile: File;
  completeFileUpload: File;
  createMilestone: Milestone;
  updateMilestone: Milestone;
  closeMilestone: Milestone;
  reopenMilestone: Milestone;
  deleteMilestone: Scalars['Boolean'];
  createOutboundMessage: OutboundMessage;
  updateOutboundMessage: OutboundMessage;
  deleteOutboundMessage: Scalars['Boolean'];
  sendTestOutboundMessage: Scalars['Boolean'];
  sendOutboundMessage: OutboundMessage;
  createList: List;
  updateList: List;
  deleteList: Scalars['Boolean'];
  importContacts: Array<Contact>;
  createMonitor: Monitor;
  updateMonitor: Monitor;
  deleteMonitor: Scalars['Boolean'];
  createBot: Bot;
  updateBot: Bot;
  deleteBot: Scalars['Boolean'];
  createBotConnection: BotConnection;
  updateBotConnection: BotConnection;
  deleteBotConnection: Scalars['Boolean'];
  adminDisableUser: Scalars['Boolean'];
  adminEnableUser: Scalars['Boolean'];
  sendChatboxMessage: ChatboxMessage;
  subscribeToList: Scalars['Boolean'];
  confirmListSubscription: Scalars['Boolean'];
  unsubscribeFromList: Scalars['Boolean'];
};


export type MutationRegisterArgs = {
  input: RegisterInput;
};


export type MutationCompleteRegistrationArgs = {
  input: CompleteRegistrationInput;
};


export type MutationSignInArgs = {
  input: SignInInput;
};


export type MutationCompleteSignInArgs = {
  input: CompleteSignInInput;
};


export type MutationRevokeSessionArgs = {
  input: RevokeSessionInput;
};


export type MutationUpdateMyProfileArgs = {
  input: UpdateMyProfileInput;
};


export type MutationVerifyEmailArgs = {
  input: VerifyEmailInput;
};


export type MutationDeleteMyAccountArgs = {
  input: DeleteMyAccountInput;
};


export type MutationEnableTwoFaArgs = {
  input: EnableTwoFaInput;
};


export type MutationDisableTwoFaArgs = {
  input: DisableTwoFaInput;
};


export type MutationCompleteTwoFaArgs = {
  input: CompleteTwoFaInput;
};


export type MutationCreateGroupArgs = {
  input: CreateGroupInput;
};


export type MutationDeleteGroupArgs = {
  input: DeleteGroupInput;
};


export type MutationUpdateGroupProfileArgs = {
  input: UpdateGroupProfileInput;
};


export type MutationInvitePeopleInGroupArgs = {
  input: InvitePeopleInGroupInput;
};


export type MutationAcceptGroupInvitationArgs = {
  input: AcceptGroupInvitationInput;
};


export type MutationDeclineGroupInvitationArgs = {
  input: DeclineGroupInvitationInput;
};


export type MutationCancelGroupInvitationArgs = {
  input: CancelGroupInvitationInput;
};


export type MutationQuitGroupArgs = {
  input: QuitGroupInput;
};


export type MutationRemoveMemberFromGroupArgs = {
  input: RemoveMemberFromGroupInput;
};


export type MutationUpdateBillingInformationArgs = {
  input: UpdateBillingInformationInput;
};


export type MutationSyncBillingWithProviderArgs = {
  input: SyncBillingWithProviderInput;
};


export type MutationCreateProjectArgs = {
  input: CreateProjectInput;
};


export type MutationUpdateProjectArgs = {
  input: UpdateProjectInput;
};


export type MutationDeleteProjectArgs = {
  input: DeleteProjectInput;
};


export type MutationSendMessageToConversationArgs = {
  input: SendMessageToConversationInput;
};


export type MutationUpdateChatboxPreferencesArgs = {
  input: UpdateChatboxPreferencesInput;
};


export type MutationCreateContactArgs = {
  input: CreateContactInput;
};


export type MutationDeleteContactArgs = {
  input: DeleteContactInput;
};


export type MutationUpdateContactArgs = {
  input: UpdateContactInput;
};


export type MutationCreateTicketArgs = {
  input: CreateTicketInput;
};


export type MutationUpdateTicketArgs = {
  input: UpdateTicketInput;
};


export type MutationCloseTicketArgs = {
  input: CloseTicketInput;
};


export type MutationReopenTicketArgs = {
  input: ReopenTicketInput;
};


export type MutationDeleteTicketArgs = {
  input: DeleteTicketInput;
};


export type MutationCommentTicketArgs = {
  input: CommentTicketInput;
};


export type MutationDeleteTicketCommentArgs = {
  input: DeleteTicketCommentInput;
};


export type MutationUpdateTicketCommentArgs = {
  input: UpdateTicketCommentInput;
};


export type MutationCreateLabelArgs = {
  input: CreateLabelInput;
};


export type MutationUpdateLabelArgs = {
  input: UpdateLabelInput;
};


export type MutationDeleteLabelArgs = {
  input: DeleteLabelInput;
};


export type MutationMoveFilesToTrashArgs = {
  input: MoveFilesToTrashInput;
};


export type MutationRestoreFilesFromTrashArgs = {
  input: RestoreFilesFromTrashInput;
};


export type MutationEmptyTrashArgs = {
  input: EmptyTrashInput;
};


export type MutationMoveFilesArgs = {
  input: MoveFilesInput;
};


export type MutationCreateFolderArgs = {
  input: CreateFolderInput;
};


export type MutationRenameFileArgs = {
  input: RenameFileInput;
};


export type MutationCompleteFileUploadArgs = {
  input: CompleteFileUploadInput;
};


export type MutationCreateMilestoneArgs = {
  input: CreateMilestoneInput;
};


export type MutationUpdateMilestoneArgs = {
  input: UpdateMilestoneInput;
};


export type MutationCloseMilestoneArgs = {
  input: CloseMilestoneInput;
};


export type MutationReopenMilestoneArgs = {
  input: ReopenMilestoneInput;
};


export type MutationDeleteMilestoneArgs = {
  input: DeleteMilestoneInput;
};


export type MutationCreateOutboundMessageArgs = {
  input: CreateOutboundMessageInput;
};


export type MutationUpdateOutboundMessageArgs = {
  input: UpdateOutboundMessageInput;
};


export type MutationDeleteOutboundMessageArgs = {
  input: DeleteOutboundMessageInput;
};


export type MutationSendTestOutboundMessageArgs = {
  input: SendTestOutboundMessageInput;
};


export type MutationSendOutboundMessageArgs = {
  input: SendOutboundMessageInput;
};


export type MutationCreateListArgs = {
  input: CreateListInput;
};


export type MutationUpdateListArgs = {
  input: UpdateListInput;
};


export type MutationDeleteListArgs = {
  input: DeleteListInput;
};


export type MutationImportContactsArgs = {
  input: ImportContactsInput;
};


export type MutationCreateMonitorArgs = {
  input: CreateMonitorInput;
};


export type MutationUpdateMonitorArgs = {
  input: UpdateMonitorInput;
};


export type MutationDeleteMonitorArgs = {
  input: DeleteMonitorInput;
};


export type MutationCreateBotArgs = {
  input: CreateBotInput;
};


export type MutationUpdateBotArgs = {
  input: UpdateBotInput;
};


export type MutationDeleteBotArgs = {
  input: DeleteBotInput;
};


export type MutationCreateBotConnectionArgs = {
  input: CreateBotConnectionInput;
};


export type MutationUpdateBotConnectionArgs = {
  input: UpdateBotConnectionInput;
};


export type MutationDeleteBotConnectionArgs = {
  input: DeleteBotConnectionInput;
};


export type MutationAdminDisableUserArgs = {
  username: Scalars['String'];
};


export type MutationAdminEnableUserArgs = {
  username: Scalars['String'];
};


export type MutationSendChatboxMessageArgs = {
  input: SendChatboxMessageInput;
};


export type MutationSubscribeToListArgs = {
  input: SubscribeToListInput;
};


export type MutationConfirmListSubscriptionArgs = {
  input: ConfirmListSubscriptionInput;
};


export type MutationUnsubscribeFromListArgs = {
  input: UnsubscribeFromListInput;
};
