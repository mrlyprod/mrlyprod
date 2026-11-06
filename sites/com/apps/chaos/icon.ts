import * as chaos from "@mrlyprod/chaos";
import type { Pen, Point } from "@mrlyprod/draw";
import { plate } from "@mrlyprod/draw/plate.ts";
import * as math from "@mrlyprod/math";
import * as radix from "@mrlyprod/radix";

export const units = { chaos, math, radix };

const BASE = 2;
const CODE = 7;
const LEVEL = 2;
const DIGITS = 3;
const CELLS = DIGITS ** LEVEL;
const TAIL = 1800;
const SEED = 7;
const DOT = 0.012;
const KNOT = 0.03;

let cloud: { digits: Point[]; path: Point[] } | null = null;

const cellOf = ([x, y]: Point) => Math.floor(x * BASE ** LEVEL) + Math.floor(y * BASE ** LEVEL) * BASE ** LEVEL;

function build() {
  if (cloud) return cloud;
  const design = radix.tile(BASE, CODE);
  if (design.size() !== DIGITS) throw new Error(`chaos: ${design.size()} digits, want ${DIGITS}`);
  const digits = design.digits().map(([x, y]): Point => [Number(x), Number(y)]);
  design.free();
  const lap = chaos.de_bruijn(DIGITS, LEVEL);
  if (lap.length !== CELLS) throw new Error(`chaos: a de Bruijn lap of ${lap.length}, want ${CELLS}`);
  const rng = new math.Rng(SEED);
  const draws = [...lap, ...lap.slice(0, LEVEL - 1), ...Array.from({ length: TAIL }, () => rng.below(DIGITS))];
  const path: Point[] = [];
  let at: Point = [0.37, 0.41];
  for (const d of draws) {
    at = [(at[0] + digits[d][0]) / BASE, (at[1] + digits[d][1]) / BASE];
    path.push(at);
  }
  const painted = new Set(path.slice(LEVEL - 1, CELLS + LEVEL - 1).map(cellOf));
  if (painted.size !== CELLS) throw new Error(`chaos: the lap paints ${painted.size} cells, want ${CELLS}`);
  for (const p of path) if (!painted.has(cellOf(p))) throw new Error("chaos: a point left the gasket");
  cloud = { digits, path };
  return cloud;
}

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "indigo", 0.18);
  const { digits, path } = build();
  const at = ([x, y]: Point): Point => [box.x + x * box.w, box.y + (1 - y) * box.h];
  const dot = box.w * DOT;
  for (const p of path) {
    const [x, y] = at(p);
    pen.rect(x - dot / 2, y - dot / 2, dot, dot, paint.white);
  }
  for (const d of digits) {
    const [x, y] = at(d);
    pen.disc(x, y, box.w * KNOT, paint.white);
  }
}
