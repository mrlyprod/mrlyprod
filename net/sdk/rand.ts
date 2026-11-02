import { mix, xorshift } from "../kernel/rand.ts";

export { mix, xorshift };

/* TYPES */

export type Rng = () => number;

export function rng(seed: number | undefined): Rng {
  let s = mix(seed ?? 0);
  return () => {
    s = xorshift(s);
    return s / 4294967296;
  };
}

export function pick<T>(rand: Rng, list: readonly T[]): T {
  return list[Math.floor(rand() * list.length)];
}
