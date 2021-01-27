/*
  eslint-disable camelcase, prefer-template, operator-linebreak, no-bitwise,
  @typescript-eslint/no-explicit-any
*/
import { LocalStorageService } from '@/app/local_storage';


// Bloom service init values and retrieve data from window.$bloom
export class BloomService {
  localStorage: LocalStorageService;
  anonymousId: string;
  namespaceId: string;
  publicUrl: string;
  apiUrl: string;

  constructor(localStorage: LocalStorageService) {
    this.localStorage = localStorage;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    this.namespaceId = (window as any).$bloom.project;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    this.publicUrl = (window as any).$bloom.url;
    this.apiUrl = `${this.publicUrl}/api`;
    this.anonymousId = (window as any).$bloom.anonymous_id;
  }


  // public methods (to be ported to JavaScript SDK)
  // identify(userId: string, properties?: any) {
  //   console.log('identify', userId, properties);
  // }

  // track(eventName: string, properties?: any) {
  //   console.log('track', eventName, properties);
  // }

  // page(properties?: any) {
  //   console.log('page', properties);
  // }
}

export const BloomServiceProvider = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: BloomService) {
    Vue.prototype.$bloom = service;
  },
};
