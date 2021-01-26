/* eslint-disable */
import { Option } from '../kernel/model';

export type File = {
  id: string;
  created_at: string;
  updated_at: string;
  name: string;
  size: number;
  type: string;
  explicitly_trashed: boolean;
  trashed_at: Option<string>
  children: Option<File[]>
};

export type GetFile = {
  namespace_id: string;
  file_id: Option<string>;
};

export type GetTrash = {
  namespace_id: string;
};
