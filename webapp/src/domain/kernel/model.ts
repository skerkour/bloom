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
  token: string | null,
}


export type Registered = {
  me: Me;
  session: Session;
}


export type SignedIn = {
  me: Me;
  session: Session;
}
