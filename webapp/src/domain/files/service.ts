
/* eslint-disable class-methods-use-this */
import ApiClient from '@/api/client';
import { AppState } from '@/app/store';
import { Store } from 'vuex';
import { File, GetFile, GetTrash } from './model';
import { Queries } from './routes';

export class FilesService {
  private apiClient: ApiClient;
  private store: Store<AppState>;
  fileTypeFolder: string;

  constructor(apiClient: ApiClient, store: Store<AppState>) {
    this.apiClient = apiClient;
    this.store = store;
    this.fileTypeFolder = 'application/com.bloom42.files.folder';
  }

  async fetchFile(fileId: string | null): Promise<File> {
    const input: GetFile = {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      namespace_id: this.store.state.currentNamespaceId!,
      file_id: fileId,
    };
    const res: File = await this.apiClient.post(Queries.file, input);

    return res;
  }

  async fetchTrash(): Promise<File[]> {
    const input: GetTrash = {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      namespace_id: this.store.state.currentNamespaceId!,
    };
    const res: File[] = await this.apiClient.post(Queries.trash, input);

    return res;
  }

  async downloadFile(file: File): Promise<void> {
    // TODO
  }
}

export const FilesServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: FilesService) {
    Vue.prototype.$filesService = service;
  },
};
