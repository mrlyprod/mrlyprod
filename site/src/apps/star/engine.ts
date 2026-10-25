import type { six } from "@mrlyprod/mrly/math";
import type { Mrly } from "../../lib/designs/engine.ts";
import type { Held, Look, Styled } from "../../lib/moire/engine.ts";
import { colorizer, odd, paint } from "../../lib/moire/engine.ts";

/* TYPES */

export type Cuts = {
  cut: (n: number, size: number) => Uint8Array;
  mask: () => Uint8Array | null;
  count: () => number;
  free: () => void;
};

export type Stack = Held & { low: number; high: number };

export type Shared = { text: string; value: number };

export type Ribbon = { n: number; arm: string; law: string; exact: boolean };

export type Rung = { count: number; scaled: number; logged: number };

export type Law = {
  layers: number;
  half: number;
  deepest: number;
  exact: number;
  ribbon: Ribbon[];
  chi: number;
  arm: number;
  hexagon: number;
  excess: number;
  scaled: number;
  logged: number;
  constant: number;
  slope: number | null;
  target: number;
  branch: string;
  residual: number;
  predicted: number | null;
  ladder: Rung[];
};

export const CARPET = "23";
export const SIZES = [128, 192, 256, 320, 384];
export const LEAST = 4;
export const MOST = 128;
export const TILE = 64;
export const STACKS: [string, number[]][] = [
  ["Doublings", [4, 8, 16, 32, 64]],
  ["Fours", [12, 20, 28, 48]],
  ["Twos", [6, 10, 18, 30]],
];
export const DEEP = [96, 128];
const RIBBON = 12;
const DIAGONAL = [1, 1, 1];
const MIDDLE = 0.5;

export const deepest = (layers: number) => 2 * layers - 1;

export const more = (layers: number) => (layers + 2 > MOST ? LEAST : layers + 2);

/* CUTS */

export function cutter(math: Mrly): Cuts {
  const spec = math.moire.Spec.new(CARPET, 2, 3);
  const frame = math.moire.frame(DIAGONAL, MIDDLE);
  const held = new Map<number, Uint8Array>();
  let inside: Uint8Array | null = null;
  let side = 0;
  const cut = (n: number, size: number) => {
    if (size !== side) {
      held.clear();
      inside = null;
      side = size;
    }
    if (!held.has(n)) {
      const volume = math.moire.volume(spec, [n], "Sum", 1, n);
      const [ink, mask] = volume.plane(frame, size);
      volume.free();
      held.set(n, Uint8Array.from(ink));
      inside ??= mask;
    }
    return held.get(n)!;
  };
  return { cut, mask: () => inside, count: () => held.size, free: () => held.clear() };
}

export function field(math: Mrly, cuts: Cuts, layers: number, size: number): Stack {
  const data = new Float32Array(size * size);
  for (const n of odd(deepest(layers))) {
    const one = cuts.cut(n, size);
    for (let i = 0; i < data.length; i++) data[i] += one[i];
  }
  const inside = cuts.mask()!;
  let low = Infinity;
  let high = -Infinity;
  for (let i = 0; i < data.length; i++) {
    if (!inside[i]) {
      data[i] = NaN;
      continue;
    }
    data[i] /= layers;
    low = Math.min(low, data[i]);
    high = Math.max(high, data[i]);
  }
  return { field: math.moire.Field.from_data(data, size), data, size, low, high };
}

export function swatch(
  math: Mrly,
  cuts: Cuts,
  value: Pick<Styled, "ramp" | "levels" | "invert">,
  layers: number,
  size: number,
  look: Look,
) {
  const made = field(math, cuts, layers, size);
  const out = paint(math, made, colorizer(value.ramp, value.levels, look), value.levels, value.invert);
  made.field.free();
  return out;
}

/* LAW */

const share = (one: six.star.Share): Shared => {
  const [numer, denom] = one.reduced();
  const out = { text: `${numer}/${denom}`, value: one.value() };
  one.free();
  return out;
};

export function rungs(layers: number) {
  const out = [layers];
  while (out.at(-1)! % 4 === 0 && out.at(-1)! / 2 >= LEAST) out.push(out.at(-1)! / 2);
  return out.reverse();
}

export function law(math: Mrly, star: six.star.Star, layers: number, half: number): Law {
  const s = math.six.star;
  const excesses = star.excesses(layers, half);
  const read = s.decay(excesses, layers);
  const branch = s.Branch.of(layers);
  const ribbon: Ribbon[] = [];
  let exact = 0;
  let arm: Shared | null = null;
  for (const n of odd(deepest(layers))) {
    arm = share(star.arm(n, half));
    const rule = share(s.arm_law(n));
    exact += arm.text === rule.text ? 1 : 0;
    if (ribbon.length < RIBBON) ribbon.push({ n, arm: arm.text, law: rule.text, exact: arm.text === rule.text });
  }
  const n = deepest(layers);
  const hexagon = share(star.hexagon(n));
  return {
    layers,
    half,
    deepest: n,
    exact,
    ribbon,
    chi: Number(s.chi8(n)),
    arm: arm!.value,
    hexagon: hexagon.value,
    excess: excesses[layers - 1],
    scaled: read.scaled,
    logged: read.logged,
    constant: s.constant(),
    slope: read.slope ?? null,
    target: s.width_law(half),
    branch: s.Branch.name(branch),
    residual: read.residual,
    predicted: s.Branch.residual(branch) ?? null,
    ladder: rungs(layers).map((count) => {
      const rung = s.decay(excesses, count);
      return { count, scaled: rung.scaled, logged: rung.logged };
    }),
  };
}
