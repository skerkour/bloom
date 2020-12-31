/*
  eslint-disable camelcase, prefer-template, operator-linebreak, no-bitwise,
  @typescript-eslint/no-explicit-any
*/
const byteToHex: string[] = [];

// eslint-disable-next-line no-plusplus
for (let i = 0; i < 256; ++i) {
  byteToHex.push((i + 0x100).toString(16).substr(1));
}

function bytesToUuid(buf: Uint8Array): string {
  const offset = 0;

  // Note: Be careful editing this code!  It's been tuned for performance
  // and works in ways you may not expect. See https://github.com/uuidjs/uuid/pull/434
  return (
    byteToHex[buf[offset + 0]] +
    byteToHex[buf[offset + 1]] +
    byteToHex[buf[offset + 2]] +
    byteToHex[buf[offset + 3]] +
    '-' +
    byteToHex[buf[offset + 4]] +
    byteToHex[buf[offset + 5]] +
    '-' +
    byteToHex[buf[offset + 6]] +
    byteToHex[buf[offset + 7]] +
    '-' +
    byteToHex[buf[offset + 8]] +
    byteToHex[buf[offset + 9]] +
    '-' +
    byteToHex[buf[offset + 10]] +
    byteToHex[buf[offset + 11]] +
    byteToHex[buf[offset + 12]] +
    byteToHex[buf[offset + 13]] +
    byteToHex[buf[offset + 14]] +
    byteToHex[buf[offset + 15]]
  ).toLowerCase();
}

export default function uuidv4(): string {
  const uuidBytes = new Uint8Array(16);
  window.crypto.getRandomValues(uuidBytes);
  uuidBytes[6] = (uuidBytes[6] & 0x0f) | 0x40;
  uuidBytes[8] = (uuidBytes[8] & 0x3f) | 0x80;
  return bytesToUuid(uuidBytes);
}
