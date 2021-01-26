/* eslint-disable */
const prefix = 'files';
const commands = 'commands';
const queries = 'queries';

export const Commands = {
  completeFileUpload: `/${prefix}/${commands}/complete_file_upload`,
  createFolder: `/${prefix}/${commands}/create_folder`,
  emptyTrash: `/${prefix}/${commands}/empty_trash`,
  moveFilesToTrash: `/${prefix}/${commands}/move_files_to_trash`,
  renameFile: `/${prefix}/${commands}/rename_file`,
  restoreFilesFromTrash: `/${prefix}/${commands}/restore_files_from_trash`,
}

export const Queries = {
  file: `/${prefix}/${queries}/file`,
  trash: `/${prefix}/${queries}/trash`,
  fileDownloadUrl: `/${prefix}/${queries}/file_download_url`,
}
