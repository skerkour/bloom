/* eslint-disable */
export type Option<T> = T | null;
export type Empty = {};

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

export type Group = {
  id: Option<string>,
  name: string,
  created_at: Option<string>,
  namespace_id: Option<string>,
}

export type Me = {
  // TODO
  user: User,
  session: Session;
  groups: Group[],
}

export type Register = {
  username: string;
  email: string;
};

export type RegistrationStarted = {
  pending_user_id: string;
}

export type Session = {
  id: string,
  created_at: string,
}

export type SignedIn = {
  me: Me;
  token: string;
  two_fa_method: Option<string>,
}

export type SignIn = {
  email_or_username: string;
}

export type SignInStarted = {
  pending_session_id: string;
}

export type User = {
  id: Option<string>;
  username: string;
  name: string;
  avatar_url: string;
  namespace_id: Option<string>;
}
