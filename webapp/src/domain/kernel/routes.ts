/* eslint-disable */
const prefix = 'kernel';
const commands = 'commands';
const queries = 'queries';

export const Commands = {
  register: `/${prefix}/${commands}/register`,
  completeRegistration: `/${prefix}/${commands}/complete_registration`,
  signIn: `/${prefix}/${commands}/sign_in`,
  completeSignIn: `/${prefix}/${commands}/complete_sign_in`,
  completeTwoFaChallenge: `/${prefix}/${commands}/complete_two_fa_challenge`,
}
