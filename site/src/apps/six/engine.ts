import type { Cell, Cell6d, Color } from "@mrlyprod/mrly/math";
import type { Value } from "../../lib/app/spec.ts";
import { num, sheet as paper } from "../../lib/app/svg.ts";
import type { Code, Mrly } from "../../lib/designs/engine.ts";
import { canon, roll, title } from "../../lib/designs/engine.ts";
import type { Design } from "../../lib/designs/store.ts";
import { rng } from "../../lib/rand.ts";

/* TYPES */

export type Look = "cut" | "pro" | "iso";

export type Shade = "VOID" | "FILL" | "UP" | "LEFT" | "RIGHT";

export type Preset = { sheet: "rect"; x: number; y: number } | { sheet: "disc"; radius: number };

export type Knobs = {
  view: Look;
  sheet: string;
  x: number;
  y: number;
  radius: number;
  overhang: number;
  number: number;
  level: number;
  turn: number;
  invert: number;
  tint: string;
};

export type Six = Knobs & Pick<Design, "base" | "code">;

export type Facts = {
  code: string;
  name: string;
  title: string;
  level: number;
  top: number;
  side: number;
  copies: number;
  tile: [number, number];
  sheet: [number, number];
  triangles: number;
  fills: number;
  voids: number;
  exposed: number;
  alone: number;
  pieces: number;
  holes: number;
  corners: number;
  edges: number;
  euler: number;
};

export type Study = { key: string; cube: Cell; one: Cell6d; sheet: Cell6d; drawn: Cell6d; facts: Facts };

export type Box = [number, number, number, number];

export type Group = { name: Shade; points: Float64Array };

export type Built = { groups: Group[]; box: Box; across: boolean };

export const DIM = 3;
export const NUMBERS = [3, 5, 7, 9];
export const VIEWS: [Look, string][] = [
  ["cut", "Middle slice"],
  ["pro", "Three sides"],
  ["iso", "Iso skin"],
];
export const SHEETS: [string, string][] = [
  ["rect", "Rectangle"],
  ["disc", "Disc"],
];
export const PRESETS: Preset[] = [
  { sheet: "rect", x: 5, y: 5 },
  { sheet: "rect", x: 3, y: 9 },
  { sheet: "disc", radius: 2 },
];
export const ORDER: Shade[] = ["VOID", "FILL", "UP", "LEFT", "RIGHT"];
export const BUDGET = 32000;
export const REPEATS = 12;
export const RINGS = 4;
export const RATIO = Math.sqrt(3) / 2;
export const SCALE = 8;

/* SIZE */

export const copies = (math: Mrly, value: Pick<Knobs, "sheet" | "x" | "y" | "radius">) =>
  value.sheet === "disc" ? math.six.copies(value.radius) : value.x * value.y;

export const cap = (math: Mrly, number: number, n: number) =>
  math.bang.level_cap(number, 2, Math.floor(BUDGET / (6 * n)));

export const croppable = (value: Value) =>
  value.sheet !== "disc" && (value.x as number) >= 2 && (value.y as number) >= 2;

export const cropped = (value: Value) => croppable(value) && Number(value.overhang) !== 1;

export const reroll = (math: Mrly, base: number, seed: number) => roll(math, DIM, base, rng(seed));

export const titled = (one: Preset) => (one.sheet === "disc" ? `disc(${one.radius})` : `tile(${one.x},${one.y})`);

export const holds = (value: Value, one: Preset) => Object.entries(one).every(([key, v]) => value[key] === v);

/* STUDY */

const keyed = (...parts: unknown[]) => JSON.stringify(parts);

const solid = (
  math: Mrly,
  code: Code,
  value: Pick<Knobs, "number" | "invert"> & Pick<Design, "base">,
  level: number,
) => {
  const made = math.three.create(code, value.number, level, value.base);
  return value.invert ? math.cell.models.anti(made) : made;
};

const laid = (math: Mrly, one: Cell6d, value: Knobs) =>
  value.sheet === "disc" ? math.six.disc(one, value.radius) : math.six.tile_cell(one, value.x, value.y, cropped(value));

export function sample(math: Mrly, value: Six) {
  const one = math.six[value.view](solid(math, canon(math, DIM, value.base, value.code), value, 1));
  return mesh(math, math.six.framed(laid(math, one, value)), value.turn === 1);
}

export function studio(math: Mrly) {
  const six = math.six;
  const held = new Map<string, { key: string; out: unknown }>();
  const layer = <T>(name: string, key: string, build: () => T): T => {
    const was = held.get(name);
    if (was?.key === key) return was.out as T;
    const out = build();
    held.set(name, { key, out });
    return out;
  };
  return (value: Six): Study => {
    const n = copies(math, value);
    const top = cap(math, value.number, n);
    const level = Math.min(value.level, top);
    const code = layer("code", keyed(value.base, value.code), () => canon(math, DIM, value.base, value.code));
    const grown = keyed(code, value.number, level, value.base, value.invert);
    const cube = layer("cube", grown, () => solid(math, code, value, level));
    const seen = keyed(grown, value.view);
    const one = layer("one", seen, () => six[value.view](cube));
    const key = keyed(seen, value.sheet === "disc" ? ["disc", value.radius] : [value.x, value.y, cropped(value)]);
    const sheet = layer("sheet", key, () => laid(math, one, value));
    const drawn = layer("drawn", key, () => six.framed(sheet));
    const facts = layer("facts", keyed(key, top), (): Facts => {
      const skin = six.skin(sheet);
      const tally = six.census(skin, false);
      return {
        code,
        name: math.name.of(code, DIM, value.base) ?? "",
        title: title(math, DIM, value.base, code),
        level,
        top,
        side: value.number ** level,
        copies: n,
        tile: [six.width(one), six.height(one)],
        sheet: [six.width(sheet), six.height(sheet)],
        triangles: tally.triangles,
        fills: tally.fills,
        voids: tally.voids,
        exposed: six.fills_only(skin).boundary_edges,
        alone: six.fills_only(six.skin(one)).boundary_edges,
        pieces: six.components(skin),
        holes: six.holes(skin),
        corners: tally.vertices,
        edges: tally.edges,
        euler: tally.euler,
      };
    });
    return { key, cube, one, sheet, drawn, facts };
  };
}

/* MESH */

export function kinds(math: Mrly) {
  const six = math.six;
  return { VOID: six.VOID(), FILL: six.FILL(), GRID: six.GRID(), UP: six.UP(), LEFT: six.LEFT(), RIGHT: six.RIGHT() };
}

export function bounds(groups: readonly Group[]): Box {
  let [x0, y0, x1, y1] = [Infinity, Infinity, -Infinity, -Infinity];
  for (const { points } of groups) {
    for (let i = 0; i < points.length; i += 2) {
      x0 = Math.min(x0, points[i]);
      x1 = Math.max(x1, points[i]);
      y0 = Math.min(y0, points[i + 1]);
      y1 = Math.max(y1, points[i + 1]);
    }
  }
  return Number.isFinite(x0) ? [x0, y0, x1, y1] : [0, 0, 1, 1];
}

export function mesh(math: Mrly, sheet: Cell6d, turn: boolean): Built {
  const codes = kinds(math);
  const names = Object.fromEntries(Object.entries(codes).map(([name, type]) => [type, name]));
  const custom = Object.fromEntries(
    Object.entries(codes).map(([name, type]): [string, Color[]] => [
      String(type),
      [[type, 0, 0, name === "GRID" ? 0 : 255]],
    ]),
  );
  const across = sheet.orientation === "Horizontal";
  const runs = new Map<string, number[]>();
  for (const [corners, rgba] of math.six.triangles(math.six.paint(sheet, custom))) {
    const name = names[rgba[0]];
    if (!runs.has(name)) runs.set(name, []);
    const pts = runs.get(name)!;
    for (const [x, y] of corners) {
      const u = across ? Number(x) : Number(x) * RATIO;
      const v = across ? Number(y) * RATIO : Number(y);
      if (turn) pts.push(0 - v, u);
      else pts.push(u, v);
    }
  }
  const groups = ORDER.filter((name) => runs.has(name)).map((name) => ({
    name,
    points: Float64Array.from(runs.get(name)!),
  }));
  return { groups, box: bounds(groups), across };
}

export function fit(box: readonly number[], w: number, h: number, pad: number) {
  const [x0, y0, x1, y1] = box;
  const k = Math.min((w - 2 * pad) / (x1 - x0 || 1), (h - 2 * pad) / (y1 - y0 || 1));
  return { k, ox: w / 2 - ((x0 + x1) / 2) * k, oy: h / 2 - ((y0 + y1) / 2) * k };
}

/* SVG */

export function wrap(text: string, across: boolean, turn: boolean) {
  const found = /^\s*<svg width="([\d.]+)" height="([\d.]+)"[^>]*>([\s\S]*)<\/svg>\s*$/.exec(text);
  if (!found) return text;
  const [, w, h, body] = found;
  const sx = across ? 1 : RATIO;
  const sy = across ? RATIO : 1;
  const W = Number(w) * sx;
  const H = Number(h) * sy;
  const [width, height] = turn ? [H, W] : [W, H];
  const transform = `${turn ? `translate(${num(H, 3)} 0) rotate(90) ` : ""}scale(${num(sx, 3)} ${num(sy, 3)})`;
  return paper(width, height, `<g transform="${transform}">${body.trim()}</g>`);
}
