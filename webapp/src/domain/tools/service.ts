/* eslint-disable class-methods-use-this */
import ApiClient from '@/api/client';
import moment from 'moment';

export type Timestamp = {
  unix: number;
  iso: string;
};

export class ToolsService {
  private apiClient: ApiClient;

  constructor(apiClient: ApiClient) {
    this.apiClient = apiClient;
  }

  textToBase64(input: string): string {
    return btoa(input);
  }

  base64ToText(input: string): string {
    return atob(input);
  }

  beautifyJSON(input: string): string {
    const json = JSON.parse(input);
    return JSON.stringify(json, null, 4);
  }

  minifyJSON(input: string): string {
    const json = JSON.parse(input);
    return JSON.stringify(json);
  }

  rot13(s: string) {
    // eslint-disable-next-line
    return s.replace( /[A-Za-z]/g , function(c: string) {
      // eslint-disable-next-line
      return String.fromCharCode( c.charCodeAt(0) + ( c.toUpperCase() <= 'M' ? 13 : -13 ) );
    });
  }

  encodeUri(input: string): string {
    return encodeURI(input);
  }

  decodeUri(input: string): string {
    return decodeURI(input);
  }

  timestamp(input: string): Timestamp {
    let timestamp = moment();

    if (/^([0-9]{5,})$/.test(input)) {
      timestamp = moment.unix(parseInt(input, 10));
    } else {
      timestamp = moment(input);
    }

    return {
      unix: timestamp.unix(),
      iso: timestamp.toISOString(),
    };
  }

  encodeHex(input: string): string {
    let result = '';
    for (let i = 0; i < input.length; i += 1) {
      result += input.charCodeAt(i).toString(16);
      // result += (`000${hex}`).slice(-4);
    }

    return result;
  }

  decodeHex(input: string): string {
    // see here for 4 bytes wide tweak
    // https://stackoverflow.com/questions/21647928/javascript-unicode-string-to-hex
    // const hexes = input.match(/.{1,4}/g) || [];
    let result = '';

    for (let i = 0; i < input.length; i += 2) {
      result += String.fromCharCode(parseInt(input.substr(i, 2), 16));
    }

    return result;
  }

  validateHexdumpFile(file: File) {
    // 2 MB
    if (file.size > 2000000) {
      throw new Error('File size must be less or equal to 2MB');
    }
  }

  // async hexdump(file: File): Promise<string> {
  //   this.validateHexdumpFile(file);

  //   const query = `
  //     query($file: Upload!) {
  //       hexdump(file: $file)
  //     }
  //   `;
  //   const variables = {};
  //   const operations = { query, variables };
  //   const map = {
  //     0: ['variables.file'],
  //   };

  //   const formData = new FormData();
  //   formData.append('operations', JSON.stringify(operations));
  //   formData.append('map', JSON.stringify(map));
  //   formData.append('0', file);

  //   const res: { hexdump: string } = await this.apiClient.upload(formData);
  //   return res.hexdump;
  // }

  async qrCode(input: string): Promise<string> {
    const query = `
      query($input: String!) {
        qrCode(input: $input)
      }
    `;
    const variables = { input };

    const res: { qrCode: string } = await this.apiClient.query(query, variables);
    return res.qrCode;
  }
}

export const ToolsServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: ToolsService) {
    Vue.prototype.$toolsService = service;
  },
};
