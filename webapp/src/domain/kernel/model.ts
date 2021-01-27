/* eslint-disable */
export type Option<T> = T | null;
export type Empty = {};

export type AcceptGroupInvitation = {
  invitation_id: string;
}

export type CancelGroupInvitation = {
  invitation_id: string;
}

export type CompleteRegistration = {
  pending_user_id: string;
  code: string;
}

export type CompleteSignIn = {
  pending_session_id: string;
  code: string;
}

export type CompleteTwoFaChallenge = {
  pending_session_id: string;
  code: string;
}

export type CompleteTwoFaSetup = {
  code: string;
}

export type CreateGroup = {
  name: string;
  path: string;
  description: string;
}

export type DeclineGroupInvitation = {
  invitation_id: string;
}

export type DeleteMyAccount = {
  two_fa_code: Option<string>;
}

export type DeleteGroup = {
  group_id: string;
}

export type DisableTwoFa = {
  code: string;
}

export type GenerateQrCode = {
  input: string;
}

export type GetGroup = {
  path: string;
}

export type GetSignedUploadUrl = {
  namespace_id: string;
  filesize: number;
}

export type Group = {
  id: Option<string>;
  avatar_url: string;
  name: string;
  created_at: Option<string>;
  namespace_id: Option<string>;
  path: string;
  description: string;
}

export type GroupInvitation = {
  id: string;
  created_at: string;
  inviter: User;
  invitee: User;
  group: Group;
}

export type GroupMember = {
  user_id: string;
  username: string;
  avatar_url: string;
  name: string;
  role: string;
  joined_at: string;
}

export type GroupWithMembersAndInvitations = {
  group: Group;
  invitations: GroupInvitation[];
  members: GroupMember[];
}

export type InvitePeopleInGroup = {
  group_id: string;
  usernames: string[];
}

export type Markdown = {
  markdown: string;
}

export type MarkdownHtml = {
  html: string;
}

export type Me = {
  user: User;
  session: Session;
  groups: Group[];
}

export type QrCode = {
  base64_jpeg_qr_code: string;
}

export type QuitGroup = {
  group_id: string;
}

export type Register = {
  username: string;
  email: string;
};

export type RegistrationStarted = {
  pending_user_id: string;
}

export type RevokeSession = {
  session_id: string;
}

export type Session = {
  id: string,
  created_at: string,
}

export type SetupTwoFa = {
  base64_qr_code: string,
}

export type SignedIn = {
  me: Me;
  token: string;
  two_fa_method: Option<string>,
}

export type SignedUploadUrl = {
  url: string;
  upload_id: string;
}

export type SignIn = {
  email_or_username: string;
}

export type SignInStarted = {
  pending_session_id: string;
}

export type RemoveMemberFromGroup = {
  group_id: string;
  username: string;
}

export type UpdateGroupProfile = {
  group_id: string;
  name: Option<string>;
  path: Option<string>;
  description: Option<string>;
}

export type UpdateMyProfile = {
  username: Option<string>;
  name: Option<string>;
  description: Option<string>;
  email: Option<string>;
}

export type User = {
  id: Option<string>;
  username: string;
  name: string;
  avatar_url: string;
  namespace_id: Option<string>;
  two_fa_enabled: Option<boolean>;
  is_admin:  Option<boolean>;
  email:Option<string>;
  description: string;
}
