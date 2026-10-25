import type { Raw, Spec } from "../app/spec.ts";
import { tidy } from "../app/spec.ts";
import { TINTS } from "../color.ts";
import type { Mrly } from "../designs/engine.ts";
import { canon } from "../designs/engine.ts";
import type { Disc, Src } from "./engine.ts";
import { BASES, CUT, cap, cube, DIMS, LIMIT, NUMBERS, PRESETS, RAMPS, SOURCES, TILES } from "./engine.ts";

/* TYPES */

export type Look = { ramp: string; levels: number; invert: number; tint: string };

export const SOURCE: Spec = [
  { key: "source", label: "Source", kind: "segment", def: "design", options: SOURCES },
  { key: "dim", label: "Dim", kind: "segment", def: 2, options: DIMS },
  { key: "base", label: "Base", kind: "segment", def: 3, options: BASES },
  { key: "code", label: "Code", kind: "text", def: "" },
  { key: "number", label: "Side", kind: "pick", def: 3, options: NUMBERS.map((n) => [n, String(n)]) },
  { key: "level", label: "Level", kind: "slider", def: 4, min: 1, max: (value) => cap(Number(value.number)), step: 1 },
  { key: "preset", label: "Preset", kind: "pick", def: "weave", options: PRESETS },
  { key: "limit", label: "Scales up to", kind: "slider", def: 9, min: 1, max: LIMIT, step: 2 },
  { key: "tile", label: "Tile", kind: "pick", def: 3, options: TILES.map((n) => [n, String(n)]) },
  {
    key: "slevel",
    label: "Level",
    kind: "slider",
    def: 2,
    min: 1,
    max: (value) => cap(Number(value.tile), CUT),
    step: 1,
  },
];

export const LOOK: Spec = [
  { key: "ramp", label: "Ramp", kind: "pick", def: "heat", options: RAMPS, group: "Look" },
  { key: "levels", label: "Levels", kind: "slider", def: 64, min: 2, max: 64, step: 1, group: "Look" },
  { key: "invert", label: "Invert", kind: "toggle", def: 0, group: "Look" },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

export function tag(value: Raw, math?: Mrly | null): string {
  const src = tidy(SOURCE, value) as Src;
  if (src.source === "moire") return `moire-${src.preset}`;
  const dim = cube(src) ? 3 : 2;
  const code = math ? canon(math, dim, src.base, src.code) : src.code || "carpet";
  return `${dim === 3 ? "slice" : "flat"}-${code}`;
}

export const PAD = 16;

export const TAU = Math.PI * 2;

export function rim(ctx: CanvasRenderingContext2D, { x, y, r }: Disc, line: string, dpr: number): void {
  ctx.lineWidth = dpr;
  ctx.strokeStyle = line;
  ctx.beginPath();
  ctx.arc(x, y, r, 0, TAU);
  ctx.stroke();
}

export function place(
  ctx: CanvasRenderingContext2D,
  layer: CanvasImageSource,
  disc: Disc,
  line: string,
  dpr: number,
): void {
  const { x, y, r } = disc;
  ctx.save();
  ctx.beginPath();
  ctx.arc(x, y, r, 0, TAU);
  ctx.clip();
  ctx.drawImage(layer, x - r, y - r, 2 * r, 2 * r);
  ctx.restore();
  rim(ctx, disc, line, dpr);
}

export function sheet(pixels: Uint8ClampedArray<ArrayBuffer>, side: number): OffscreenCanvas {
  const layer = new OffscreenCanvas(side, side);
  layer.getContext("2d")!.putImageData(new ImageData(pixels, side, side), 0, 0);
  return layer;
}
