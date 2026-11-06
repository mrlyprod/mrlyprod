import { xorshift } from "@mrlyprod/ui/kit/rand.ts";
import { HUES } from "@mrlyprod/ui/kit/theme/hues.ts";
import type { Tile } from "../designs/tiles.ts";
import { DESIGNS, ink, NUMBERS, tile } from "../designs/tiles.ts";

export type Sprite = { design: string; number: number; hue: string };

export type Drawn<T> = [T, number];

const MARKS = new Map<string, Tile>();

export const named = ({ design, number }: Pick<Sprite, "design" | "number">) => `${design} ${number}`;

export function below(seed: number, n: number): Drawn<number> {
  const next = xorshift(seed);
  return [next % n, next];
}

export function shuffled<T>(seed: number, list: readonly T[]): Drawn<T[]> {
  const out = list.slice();
  let at = seed;
  for (let i = out.length - 1; i > 0; i--) {
    const [j, next] = below(at, i + 1);
    [out[i], out[j]] = [out[j], out[i]];
    at = next;
  }
  return [out, at];
}

export function sprite(seed: number): Drawn<Sprite> {
  const [d, a] = below(seed, DESIGNS.length);
  const [n, b] = below(a, NUMBERS.length);
  const [h, c] = below(b, HUES.length);
  return [{ design: DESIGNS[d], number: NUMBERS[n], hue: HUES[h] }, c];
}

export function sprites(seed: number, count: number): Drawn<Sprite[]> {
  const [pairs, a] = shuffled(
    seed,
    DESIGNS.flatMap((design) => NUMBERS.map((number) => ({ design, number }))),
  );
  const [hues, b] = shuffled(a, HUES);
  return [pairs.slice(0, count).map((pair, i) => ({ ...pair, hue: hues[i % hues.length] })), b];
}

export function stamp(
  ctx: CanvasRenderingContext2D,
  one: Pick<Sprite, "design" | "number">,
  x: number,
  y: number,
  side: number,
) {
  const key = named(one);
  if (!MARKS.has(key)) MARKS.set(key, tile(one.design, one.number));
  ink(ctx, MARKS.get(key)!, x, y, side);
}
