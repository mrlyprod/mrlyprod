import { tidy } from "../app/spec.js";
import { TINTS } from "../color.js";
import { canon } from "../designs/engine.js";
import { BASES, CUT, cap, cube, DIMS, LIMIT, NUMBERS, PRESETS, RAMPS, SOURCES, TILES } from "./engine.js";

export const SOURCE = [
  { key: "source", label: "Source", kind: "segment", def: "design", options: SOURCES },
  { key: "dim", label: "Dim", kind: "segment", def: 2, options: DIMS },
  { key: "base", label: "Base", kind: "segment", def: 3, options: BASES },
  { key: "code", label: "Code", kind: "text", def: "" },
  { key: "number", label: "Side", kind: "pick", def: 3, options: NUMBERS.map((n) => [n, String(n)]) },
  { key: "level", label: "Level", kind: "slider", def: 4, min: 1, max: (value) => cap(value.number), step: 1 },
  { key: "preset", label: "Preset", kind: "pick", def: "weave", options: PRESETS },
  { key: "limit", label: "Scales up to", kind: "slider", def: 9, min: 1, max: LIMIT, step: 2 },
  { key: "tile", label: "Tile", kind: "pick", def: 3, options: TILES.map((n) => [n, String(n)]) },
  { key: "slevel", label: "Level", kind: "slider", def: 2, min: 1, max: (value) => cap(value.tile, CUT), step: 1 },
];

export const LOOK = [
  { key: "ramp", label: "Ramp", kind: "pick", def: "heat", options: RAMPS, group: "Look" },
  { key: "levels", label: "Levels", kind: "slider", def: 64, min: 2, max: 64, step: 1, group: "Look" },
  { key: "invert", label: "Invert", kind: "toggle", def: 0, group: "Look" },
  { key: "tint", label: "Tint", kind: "pick", def: "", options: TINTS, group: "Look" },
];

export function tag(value, math) {
  const src = tidy(SOURCE, value);
  if (src.source === "moire") return `moire-${src.preset}`;
  const dim = cube(src) ? 3 : 2;
  const code = math ? canon(math, dim, src.base, src.code) : src.code || "carpet";
  return `${dim === 3 ? "slice" : "flat"}-${code}`;
}

export const PAD = 16;

export const TAU = Math.PI * 2;

export function rim(ctx, { x, y, r }, line, dpr) {
  ctx.lineWidth = dpr;
  ctx.strokeStyle = line;
  ctx.beginPath();
  ctx.arc(x, y, r, 0, TAU);
  ctx.stroke();
}

export function place(ctx, layer, disc, line, dpr) {
  const { x, y, r } = disc;
  ctx.save();
  ctx.beginPath();
  ctx.arc(x, y, r, 0, TAU);
  ctx.clip();
  ctx.drawImage(layer, x - r, y - r, 2 * r, 2 * r);
  ctx.restore();
  rim(ctx, disc, line, dpr);
}

export function sheet(pixels, side) {
  const layer = new OffscreenCanvas(side, side);
  layer.getContext("2d").putImageData(new ImageData(pixels, side, side), 0, 0);
  return layer;
}
