/* eslint-disable */
import { Option } from '../kernel/model';

export type CompleteFileUpload = {
  upload_id: string;
  parent_id: string;
  name: string;
  mime_type: string;
};

export type CreateFolder = {
  parent_id: string;
  name: string;
};

export type EmptyTrash = {
  namespace_id: string;
};

export type File = {
  id: string;
  created_at: string;
  updated_at: string;
  name: string;
  size: number;
  type: string;
  explicitly_trashed: boolean;
  trashed_at: Option<string>;
  children: Option<File[]>;
  path: FilePath[];
};

export type FileDownloadUrl = {
  url: string;
};

export type FilePath = {
  id: string;
  name: string;
};

export type GetFile = {
  namespace_id: string;
  file_id: Option<string>;
};

export type GetFileDownloadUrl = {
  file_id: string;
};

export type GetTrash = {
  namespace_id: string;
};

export type MoveFilesToTrash = {
  files: string[];
};

export type RenameFile = {
  file_id: string;
  name: string;
};

export type RestoreFilesFromTrash = {
  files: string[];
};
