/* eslint-disable */
export namespace routes {
  const prefix = 'kernel';
  const commands = 'commands';
  const queries = 'queries';

  export const Commands = {
    register: `/${prefix}/${commands}/register`,
    complete_registration: `/${prefix}/${commands}/complete_registration`,
  }
}


export namespace model {
  export namespace input {
    export type Register = {
      username: string;
      email: string;
    };
  }

  export type RegistrationStarted = {
    pending_user_id: string;
  }

  export type CompleteRegistration = {
    pending_user_id: string;
    code: string;
  }
}

