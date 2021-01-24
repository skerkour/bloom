/* eslint-disable */

export type Register = {
  username: string;
  email: string;
};

export type RegistrationStarted = {
  pending_user_id: string;
}

export type CompleteRegistration = {
  pending_user_id: string;
  code: string;
}

export type Me = {
  // TODO
  user: User,
}

export type User = {
  // TODO
  username: string,
  name: string,
  avatar_url: string,
}

export type Session = {
  // TODO
  id: string,
  created_at: string,
  token: string | null,
}


export type Registered = {
  me: Me;
  session: Session;
  token: string;
}


export type SignedIn = {
  me: Me;
  session: Session;
  token: string;
  two_fa_method: string | null,
}

export type SignIn = {
  email_or_username: string;
}


export type SignInStarted = {
  pending_session_id: string;
}


export type CompleteSignIn = {
  pending_session_id: string;
  code: string;
}

export type CompleteTwoFaChallenge = {
  pending_session_id: string;
  code: string;
}
