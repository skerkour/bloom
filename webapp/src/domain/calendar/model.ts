/* eslint-disable */
import { Option } from '../kernel/model';

export type CreateEvent = {
  namespace_id: string;
  title: string;
  description: string;
  location: string;
  start_at: string;
  end_at: string;
};

export type DeleteEvent = {
  event_id: string;
};

export type CalendarEvent = {
  id: string;
  created_at: string;
  updated_at: string;
  title: string;
  description: string;
  location: string;
  start_at: string;
  end_at: string;
  namespace_id: string;
};

export type GetEvents = {
  namespace_id: string;
  start_at: Option<string>;
  end_at: Option<string>;
};

export type UpdateEvent = {
  event_id: string;
  title: Option<string>;
  description: Option<string>;
  location: Option<string>;
  start_at: Option<string>;
  end_at: Option<string>;
};
