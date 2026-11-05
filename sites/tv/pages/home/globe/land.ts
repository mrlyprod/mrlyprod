import mask from "./land.gen.json";

export const LEVEL = mask.level;

export function land(): Uint8Array {
  const bytes = atob(mask.bits);
  const out = new Uint8Array(mask.cells);
  for (let i = 0; i < out.length; i++) out[i] = (bytes.charCodeAt(i >> 3) >> (i & 7)) & 1;
  return out;
}

export function pack(bits: ArrayLike<number>): string {
  const bytes = new Uint8Array(Math.ceil(bits.length / 8));
  for (let i = 0; i < bits.length; i++) if (bits[i]) bytes[i >> 3] |= 1 << (i & 7);
  return btoa(String.fromCharCode(...bytes));
}
