export type Rng = () => number;

export function mix(seed: number, salt = 0): number {
  let s = (seed ^ Math.imul(salt, 0x9e3779b9)) >>> 0;
  s = Math.imul(s ^ (s >>> 16), 0x85ebca6b);
  s = Math.imul(s ^ (s >>> 13), 0xc2b2ae35);
  return (s ^ (s >>> 16)) >>> 0 || 0x9e3779b9;
}

export function rng(seed: number, salt = 0): Rng {
  let s = mix(seed, salt);
  return () => {
    s ^= s << 13;
    s ^= s >>> 17;
    s ^= s << 5;
    s >>>= 0;
    return s / 4294967296;
  };
}

export const pick = (rand: Rng, count: number) => Math.floor(rand() * count);
