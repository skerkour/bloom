/* eslint-disable @typescript-eslint/no-explicit-any, class-methods-use-this */

export class LocalStorageService {
  set(key: string, value: any) {
    if (typeof value !== 'string') {
      value = JSON.stringify(value);
    }
    window.localStorage.setItem(key, value);
  }

  get(key: string): any {
    let value = window.localStorage.getItem(key);
    if (!value) {
      return value;
    }

    try {
      value = JSON.parse(value);
      return value;
    } catch (err) {
      return value;
    }
  }

  remove(key: string): any {
    return window.localStorage.removeItem(key);
  }

  clear(): any {
    return window.localStorage.clear();
  }
}
