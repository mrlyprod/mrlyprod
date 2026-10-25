import type { Anim, Glyph } from "@mrlyprod/mrly/font";
import type { Scalar } from "../../lib/app/spec.ts";
import type { Rng } from "../../lib/rand.ts";
import { pick } from "../../lib/rand.ts";
import type { Unit } from "../../lib/units/index.ts";

/* TYPES */

export type Font = Unit<"font">;

export type Door = "uppers" | "lowers" | "digits" | "extras" | "specials";

export type Kind = { id: string; name: string; door: Door };

export type Written = { text: string; pad: number; write: Anim; merge: Uint32Array[] };

export type Asked = { text: string; pad?: number; hold?: number; loop?: number };

export type Plan = {
  text: string;
  rows: number;
  cols: number;
  fps: number;
  frames: number[][];
  still: number;
  hold: number;
  moves: number[];
};

export type Studied = {
  char: string;
  kind: string;
  label: string;
  name: string;
  rows: number;
  cols: number;
  frames: number[][];
  strokes: number;
  floor: number;
  lifts: number;
  flag: string;
};

export type Shelved = Kind & { glyphs: Studied[] };

export type Facts = { pens: number; strokes: number; flagged: string };

export const LIMIT = 80;

export const LIFTS = 5;

export const SLOT = 7;

export const GAP = 2;

export const SAMPLES = [
  "hello, world!",
  "mrlyprod",
  "simple, beautiful, useful",
  "one desk",
  "the 108 pens",
  "mrly.net",
];

export const KINDS: Kind[] = [
  { id: "upper", name: "Uppers", door: "uppers" },
  { id: "lower", name: "Lowers", door: "lowers" },
  { id: "digit", name: "Digits", door: "digits" },
  { id: "extra", name: "Extras", door: "extras" },
  { id: "special", name: "Specials", door: "specials" },
];

export const sample = (rand: Rng) => pick(rand, SAMPLES);

export const cut = (text?: Scalar | null) => String(text ?? "").slice(0, LIMIT);

export function slug(text?: Scalar | null): string {
  const words = cut(text)
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
  return words.slice(0, 24).replace(/-+$/, "") || "text";
}

/* PLAN */

export function written(font: Font, text?: Scalar | null, pad = 1): Written {
  const words = cut(text);
  return { text: words, pad, write: font.animate(words, pad), merge: font.merge(words, pad) };
}

export function plan(font: Font, { text, pad = 1, hold = 25, loop = 1 }: Asked, made = written(font, text, pad)): Plan {
  const { write, merge } = made;
  const anim = loop ? font.cycle(write, merge, hold) : write;
  const n = write.frames.length;
  return {
    text: made.text,
    rows: anim.rows,
    cols: anim.cols,
    fps: anim.fps,
    frames: anim.frames,
    still: n - 1,
    hold: loop ? hold : 0,
    moves: loop ? [n, merge.length, merge.length, n] : [n],
  };
}

export const index = (k: number, n: number, loop = 1) =>
  n < 2 ? 0 : loop ? ((k % n) + n) % n : Math.min(Math.max(0, k), n - 1);

export const pen = (k: number, n: number, hold: number, loop = 1) =>
  Math.min(loop ? index(k, n + hold) : Math.max(0, k), n - 1);

export function shift(k: number, moves: readonly number[], hold: number, next: number): number {
  let from = 0;
  let to = 0;
  for (const len of moves) {
    const off = k - from;
    if (off < len) return to + off;
    if (off < len + hold) return next ? to + len + Math.min(off - len, next - 1) : to + len - 1;
    from += len + hold;
    to += len + next;
  }
  return Math.max(0, to - 1);
}

export function common(a: string, b: string): string {
  let n = 0;
  while (n < a.length && n < b.length && a[n] === b[n]) n++;
  return a.slice(0, n);
}

export const cells = (font: Font, text: string, pad = 1) => (text ? font.animate(text, pad).frames.length - 1 : 0);

/* PENS */

export const flag = (strokes: number, floor: number, lifts: number) =>
  strokes > floor ? "over floor" : lifts >= LIFTS ? `${lifts} lifts` : "";

export function study(font: Font, glyph: Pick<Glyph, "char" | "rows">, kind = ""): Studied {
  const { char, rows } = glyph;
  const anim = font.animate(char, 0);
  const strokes = font.strokes(char).length;
  const lifts = Math.max(0, strokes - 1);
  const floor = font.floor(font.trim(rows));
  const name = font.name_of(char).toLowerCase();
  return {
    char,
    kind,
    label: char.trim() ? char : name,
    name,
    rows: anim.rows,
    cols: anim.cols,
    frames: anim.frames,
    strokes,
    floor,
    lifts,
    flag: flag(strokes, floor, lifts),
  };
}

export function gallery(font: Font): Shelved[] {
  return KINDS.map((kind) => ({
    ...kind,
    glyphs: font[kind.door]().map((glyph) => {
      const seen = study(font, glyph, kind.id);
      glyph.free?.();
      return seen;
    }),
  }));
}

export function facts(kinds: readonly Pick<Shelved, "glyphs">[]): Facts {
  const pens = kinds.flatMap((kind) => kind.glyphs);
  const flagged = pens.filter((one) => one.flag).map((one) => one.char);
  return {
    pens: pens.length,
    strokes: pens.reduce((n, one) => n + one.strokes, 0),
    flagged: flagged.join(" ") || "none",
  };
}
