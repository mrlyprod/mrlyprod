/* TYPES */

export type Rng = () => number;

export function rng(seed?: number | null): Rng {
  if (seed === undefined || seed === null) return Math.random;
  let s = seed >>> 0;
  s = Math.imul(s ^ (s >>> 16), 0x85ebca6b);
  s = Math.imul(s ^ (s >>> 13), 0xc2b2ae35);
  s = (s ^ (s >>> 16)) >>> 0 || 0x9e3779b9;
  return () => {
    s ^= s << 13;
    s ^= s >>> 17;
    s ^= s << 5;
    s >>>= 0;
    return s / 4294967296;
  };
}

export function pick<T>(rand: Rng, list: readonly T[]): T {
  return list[Math.floor(rand() * list.length)];
}
