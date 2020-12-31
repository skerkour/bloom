import moment from 'moment';

export default function date(text: string, format?: string) {
  const fmt = format ?? 'MMMM Do YYYY, HH:mm';
  return moment(text).format(fmt);
}
