
/* eslint-disable class-methods-use-this */
import ApiClient from '@/api/client';
import { AppState } from '@/app/store';
import { Store } from 'vuex';
import {
  CompleteFileUpload,
  CreateFolder,
  EmptyTrash,
  File, FileDownloadUrl, GetFile, GetFileDownloadUrl, GetTrash, MoveFilesToTrash, RenameFile,
  RestoreFilesFromTrash,
} from './model';
import { Commands, Queries } from './routes';

export type UploadingFile = {
  name: string;
  progress: number;
};

export class FilesService {
  private apiClient: ApiClient;
  private store: Store<AppState>;
  fileTypeFolder: string;
  rootFileName: string;

  constructor(apiClient: ApiClient, store: Store<AppState>) {
    this.apiClient = apiClient;
    this.store = store;
    this.fileTypeFolder = 'application/com.bloom42.files.folder';
    this.rootFileName = '__ROOT__';
  }

  async completeFileUpload(input: CompleteFileUpload): Promise<File> {
    const res: File = await this.apiClient.post(Commands.completeFileUpload, input);

    return res;
  }

  async createFolder(input: CreateFolder): Promise<File> {
    const res: File = await this.apiClient.post(Commands.createFolder, input);

    return res;
  }


  async emptyTrash(): Promise<void> {
    const input: EmptyTrash = {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      namespace_id: this.store.state.currentNamespace!.id!,
    };
    await this.apiClient.post(Commands.emptyTrash, input);
  }

  async fetchFile(fileId: string | null): Promise<File> {
    const input: GetFile = {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      namespace_id: this.store.state.currentNamespace!.id!,
      file_id: fileId,
    };
    const res: File = await this.apiClient.post(Queries.file, input);

    return res;
  }

  async fetchTrash(): Promise<File[]> {
    const input: GetTrash = {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      namespace_id: this.store.state.currentNamespace!.id!,
    };
    const res: File[] = await this.apiClient.post(Queries.trash, input);

    return res;
  }

  async downloadFile(fileId: string): Promise<void> {
    const input: GetFileDownloadUrl = {
      file_id: fileId,
    };
    const res: FileDownloadUrl = await this.apiClient.post(Queries.fileDownloadUrl, input);

    const downloadLink: HTMLAnchorElement = document.createElement('a');
    downloadLink.href = res.url;
    document.body.appendChild(downloadLink);
    downloadLink.click();
    document.body.removeChild(downloadLink);
  }

  async moveFilesToTrash(files: string[]): Promise<void> {
    const input: MoveFilesToTrash = {
      files,
    };
    await this.apiClient.post(Commands.moveFilesToTrash, input);
  }

  async renameFile(input: RenameFile): Promise<File> {
    const res: File = await this.apiClient.post(Commands.renameFile, input);

    return res;
  }

  async restoreFilesFromTrash(files: string[]): Promise<void> {
    const input: RestoreFilesFromTrash = {
      files,
    };
    await this.apiClient.post(Commands.restoreFilesFromTrash, input);
  }
}

export const FilesServiceInjector = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  install(Vue: any, service: FilesService) {
    Vue.prototype.$filesService = service;
  },
};
