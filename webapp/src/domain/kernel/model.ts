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
