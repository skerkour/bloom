import moment from 'moment';

export default function calendar(text: string) {
  return moment(text).calendar();
}
