/* eslint-disable */
import { Option } from '../kernel/model';


export type GetList = {
  list_id: string;
};

export type GetLists = {
  namespace_id: string;
};

export type List = {
  id: string;
  created_at: string;
  name: string;
  description: string;
};
