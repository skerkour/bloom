import filesize from '@/libs/filesize';

export default function fileSize(value: number): string {
  return filesize(value, { base: 10 });
}
