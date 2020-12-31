/* eslint-disable @typescript-eslint/no-non-null-assertion */
export class Config {
  env: string;
  baseURL: string;
  apiBaseURL: string;
  sentryDsn: string | undefined;

  constructor() {
    this.env = process.env.NODE_ENV!;
    this.baseURL = process.env.BASE_URL ?? '/';
    this.apiBaseURL = process.env.VUE_APP_API_BASE_URL ?? '/api';
    this.sentryDsn = process.env.VUE_APP_SENTRY_DSN;
  }
}


export const ConfigServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, conf: Config) {
    Vue.prototype.$config = conf;
  },
};
