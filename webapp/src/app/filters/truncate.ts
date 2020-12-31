export default function truncate(text: string, length = 42, suffix = '...') {
  if (text.length > length) {
    return `${text.substring(0, length)}${suffix}`;
  }
  return text;
}
