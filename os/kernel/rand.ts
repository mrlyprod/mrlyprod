import { fnv32 } from "./json.ts";

export function mix(seed: number): number {
  let s = seed >>> 0;
  s = Math.imul(s ^ (s >>> 16), 0x85ebca6b);
  s = Math.imul(s ^ (s >>> 13), 0xc2b2ae35);
  return (s ^ (s >>> 16)) >>> 0 || 0x9e3779b9;
}

export function xorshift(s: number): number {
  s ^= s << 13;
  s ^= s >>> 17;
  s ^= s << 5;
  return s >>> 0;
}

export const seedOf = (seed: number, name: string): number => mix((seed ^ fnv32(name)) >>> 0);

export function below(draw: () => number, n: number): number {
  if (!Number.isInteger(n) || n < 1 || n > 4294967296) throw new Error(`rand: below takes 1..2^32, not ${n}`);
  const limit = 4294967296 - (4294967296 % n);
  for (;;) {
    const u = draw();
    if (u < limit) return u % n;
  }
}

export function stream(state: number): { draw: () => number; state: () => number } {
  let s = state >>> 0 || 0x9e3779b9;
  return {
    draw: () => {
      s = xorshift(s);
      return s;
    },
    state: () => s,
  };
}
