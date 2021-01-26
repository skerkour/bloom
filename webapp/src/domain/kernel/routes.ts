/* eslint-disable */
const prefix = 'kernel';
const commands = 'commands';
const queries = 'queries';

export const Commands = {
  register: `/${prefix}/${commands}/register`,
  completeRegistration: `/${prefix}/${commands}/complete_registration`,
  completeSignIn: `/${prefix}/${commands}/complete_sign_in`,
  completeTwoFaChallenge: `/${prefix}/${commands}/complete_two_fa_challenge`,
  completeTwoFaSetup: `/${prefix}/${commands}/complete_two_fa_setup`,
  disableTwoFa: `/${prefix}/${commands}/disable_two_fa`,
  signIn: `/${prefix}/${commands}/sign_in`,
  setupTwoFa: `/${prefix}/${commands}/setup_two_fa`,
}

export const Queries = {
  me: `/${prefix}/${queries}/me`,
  generateQrCode: `/${prefix}/${queries}/generate_qr_code`,
  signedUploadUrl: `/${prefix}/${queries}/signed_upload_url`,
}
