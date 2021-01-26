/* eslint-disable */
import { Option } from '../kernel/model';

export type Analytics = {
  visits: Visit[],
  pages: Page[],
  referrers: Referrer[],
  devices: Device[],
  events: AnalyticsEvent[];
};

export type AnalyticsEvent = {
  event_name: string;
  views: number;
  visitors: number;
};

export type Device = {
  device_type: string;
  views: number;
  visitors: number;
};

export type GetAnalytics = {
  namespace_id: string;
};

export type Page = {
  url: string;
  path: string;
  views: number;
  visitors: number;
};

export type Referrer = {
  referrer: string;
  views: number;
  visitors: number;
};

export type Visit = {
  date: string;
  views: number;
  visitors: number;
};
