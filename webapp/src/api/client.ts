import { Config } from '@/app/config';
import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios';
import { Store } from 'vuex';
import { AppState, Mutation } from '@/app/store';
import Router from '@/app/router';

type ApiError = {
  message: string;
  extensions: Record<string, unknown>;
}

type ApiResponse = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  data: any;
  errors: ApiError[];
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
      const token = this.store.state.sessionToken;
      if (token) {
        conf.headers.Authorization = `Basic ${token}`;
      }
      return conf;
    });
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  async post(route: string, data?: any): Promise<any> {
    let res: AxiosResponse<ApiResponse> | null = null;
    data = data ?? {};

    try {
      res = await this.http.post(`${this.apiBaseURL}${route}`, data);
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
  async upload(route: string, formData: FormData): Promise<any> {
    let res: AxiosResponse<ApiResponse> | null = null;
    try {
      res = await this.http.post(`${this.apiBaseURL}${route}`, formData, {
        headers: { 'Content-Type': 'multipart/form-data' },
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
  async query(query: string, variables?: any): Promise<any> {
    let res: AxiosResponse<ApiResponse> | null = null;
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
}
