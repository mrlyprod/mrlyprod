/* TYPES */

export type Rng = () => number;

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

export function rng(seed?: number | null): Rng {
  if (seed === undefined || seed === null) return Math.random;
  let s = mix(seed);
  return () => {
    s = xorshift(s);
    return s / 4294967296;
  };
}

export function pick<T>(rand: Rng, list: readonly T[]): T {
  return list[Math.floor(rand() * list.length)];
}
