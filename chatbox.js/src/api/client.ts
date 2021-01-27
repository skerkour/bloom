import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios';
import { BloomService } from '@/domain/bloom';


type ApiError = {
  message: string;
  extensions: Record<string, unknown>;
}

type ApiResponse = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  data: any;
  errors: ApiError[];
}


export default class APIClient {
  private apiBaseURL: string;
  private http: AxiosInstance;
  private bloom: BloomService;

  constructor(bloom: BloomService) {
    this.apiBaseURL = bloom.apiUrl;
    this.http = axios;
    this.bloom = bloom;

    this.http.interceptors.request.use((conf: AxiosRequestConfig) => {
      conf.headers.Authorization = `Anonymous ${this.bloom.anonymousId}`;
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
      throw err;
    }
    return res?.data.data;
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  async upload(formData: FormData, options?: AxiosRequestConfig): Promise<any> {
    let res: AxiosResponse<ApiResponse> | null = null;
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
      throw err;
    }
    return res?.data.data;
  }
}

