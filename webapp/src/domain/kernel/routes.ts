/* eslint-disable */
const prefix = 'kernel';
const commands = 'commands';
const queries = 'queries';
const admin = 'admin';

export const Admin = {
  blockUser: `/${prefix}/${admin}/block_user`,
  group: `/${prefix}/${admin}/group`,
  groups: `/${prefix}/${admin}/groups`,
  unblockUser: `/${prefix}/${admin}/unblock_user`,
  user: `/${prefix}/${admin}/user`,
  users: `/${prefix}/${admin}/users`,
};

export const Commands = {
  acceptGroupInvitation: `/${prefix}/${commands}/accept_group_invitation`,
  cancelGroupInvitation: `/${prefix}/${commands}/cancel_group_invitation`,
  completeRegistration: `/${prefix}/${commands}/complete_registration`,
  completeSignIn: `/${prefix}/${commands}/complete_sign_in`,
  completeTwoFaChallenge: `/${prefix}/${commands}/complete_two_fa_challenge`,
  completeTwoFaSetup: `/${prefix}/${commands}/complete_two_fa_setup`,
  createGroup: `/${prefix}/${commands}/create_group`,
  declineGroupInvitation: `/${prefix}/${commands}/decline_group_invitation`,
  deleteGroup: `/${prefix}/${commands}/delete_group`,
  deleteMyAccount: `/${prefix}/${commands}/delete_my_account`,
  disableTwoFa: `/${prefix}/${commands}/disable_two_fa`,
  invitePeopleInGroup: `/${prefix}/${commands}/invite_people_in_group`,
  quitGroup: `/${prefix}/${commands}/quit_group`,
  register: `/${prefix}/${commands}/register`,
  removeMemberFromGroup: `/${prefix}/${commands}/remove_member_from_group`,
  revokeSession: `/${prefix}/${commands}/revoke_session`,
  signIn: `/${prefix}/${commands}/sign_in`,
  setupTwoFa: `/${prefix}/${commands}/setup_two_fa`,
  syncCustomerWithProvider: `/${prefix}/${commands}/sync_customer_with_provider`,
  updateBillingInformation: `/${prefix}/${commands}/update_billing_information`,
  updateGroupAvatar: `/${prefix}/${commands}/update_group_avatar`,
  updateGroupProfile: `/${prefix}/${commands}/update_group_profile`,
  updateMyAvatar: `/${prefix}/${commands}/update_my_avatar`,
  updateMyProfile: `/${prefix}/${commands}/update_my_profile`,
}

export const Queries = {
  billingInformation: `/${prefix}/${queries}/billing_information`,
  checkoutSession: `/${prefix}/${queries}/checkout_session`,
  customerPortal: `/${prefix}/${queries}/customer_portal`,
  generateQrCode: `/${prefix}/${queries}/generate_qr_code`,
  group: `/${prefix}/${queries}/group`,
  groupWithMembersAndInvitations: `/${prefix}/${queries}/group_with_members_and_invitations`,
  myGroupInvitations: `/${prefix}/${queries}/my_group_invitations`,
  markdown: `/${prefix}/${queries}/markdown`,
  me: `/${prefix}/${queries}/me`,
  mySessions: `/${prefix}/${queries}/my_sessions`,
  signedUploadUrl: `/${prefix}/${queries}/signed_upload_url`,
  stripePublicKey: `/${prefix}/${queries}/stripe_public_key`,
}
