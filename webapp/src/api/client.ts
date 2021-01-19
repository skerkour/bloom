import { Config } from '@/app/config';
import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios';
import { Store } from 'vuex';
import { AppState, Mutation } from '@/app/store';
import Router from '@/app/router';

type GraphqlError = {
  message: string;
  extensions: Record<string, unknown>;
}

type GraphqlResponse = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  data: any;
  errors: GraphqlError[];
}

export default class ApiClient {
  private http: AxiosInstance;
  private apiBaseURL: string;
  private store: Store<AppState>;
  private router: Router;

  constructor(config: Config, store: Store<AppState>, router: Router) {
    this.http = axios.create();
    this.apiBaseURL = config.apiBaseURL;
    this.store = store;
    this.router = router;

    this.http.interceptors.request.use((conf: AxiosRequestConfig) => {
      const token = this.store.state.session?.token;
      if (token) {
        conf.headers.Authorization = `Basic ${token}`;
      }
      return conf;
    });
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  async query(query: string, variables?: any): Promise<any> {
    let res: AxiosResponse<GraphqlResponse> | null = null;
    try {
      res = await this.http.post(`${this.apiBaseURL}/graphql`, {
        operationName: null,
        query,
        variables,
      });
    } catch (err) {
      if (err.response) {
        res = err.response;
      } else {
        throw err;
      }
    }
    if (res && res.data && res.data.errors && res.data.errors.length > 0) {
      const err = res.data.errors[0];
      if (err && err.message && err.message.includes('Session is not valid')) {
        this.store.commit(Mutation.SIGN_OUT);
        this.router.push({ path: '/' });
      }
      throw err;
    }
    return res?.data.data;
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  async upload(formData: FormData, options?: AxiosRequestConfig): Promise<any> {
    let res: AxiosResponse<GraphqlResponse> | null = null;
    try {
      res = await this.http.post(`${this.apiBaseURL}/graphql`, formData, options);
    } catch (err) {
      if (err.response) {
        res = err.response;
      } else {
        throw err;
      }
    }
    if (res && res.data && res.data.errors && res.data.errors.length > 0) {
      const err = res.data.errors[0];
      if (err && err.message && err.message.includes('Session is not valid')) {
        this.store.commit(Mutation.SIGN_OUT);
        this.router.push({ path: '/' });
      }
      throw err;
    }
    return res?.data.data;
  }
}
