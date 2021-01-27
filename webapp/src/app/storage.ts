/* eslint-disable @typescript-eslint/no-explicit-any, class-methods-use-this */

import localStorage from 'store';


export class Storage {
  keyToken = 'bloom_token';
  keyDarkMode = 'bloom_dark_mode';
  keyCurrentNamespace = 'current_namespace';

  set(key: string, value: any) {
    localStorage.set(key, value);
  }

  get(key: string): any {
    return localStorage.get(key);
  }

  remove(key: string): any {
    return localStorage.remove(key);
  }

  clear(): any {
    return localStorage.clearAll();
  }
}
