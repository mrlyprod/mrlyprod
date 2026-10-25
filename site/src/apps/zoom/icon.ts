import type { Color, Frame, Pen, Point } from "#view";
import { frame, Grid } from "#view";
import { plate } from "../../lib/plate.ts";

export const units = {};

const CODE = 23;
const SIDE = 27;
const DEPTHS = 3;
const DROP = 0.66;
const WANT = [512, 64, 8];

type Shade = { top: Color; side: Color; floor: Color };

const kept = (x: number, y: number) => (CODE >> ((x & 1) * 4 + (y & 1) * 2)) & 1;

function lit(level: number, x: number, y: number) {
  for (let l = 0; l < level; l++, x = Math.floor(x / 3), y = Math.floor(y / 3)) if (!kept(x % 3, y % 3)) return false;
  return true;
}

function shrink(f: Frame, k: number) {
  const side = f.w * k;
  return frame(f.x + (f.w - side) / 2, f.y + (f.h - side) / 2, side, side);
}

function walls(pen: Pen, outer: Frame, inner: Frame, tones: Shade) {
  const a: Point = [outer.x, outer.y];
  const b: Point = [outer.x + outer.w, outer.y];
  const c: Point = [outer.x + outer.w, outer.y + outer.h];
  const d: Point = [outer.x, outer.y + outer.h];
  const p: Point = [inner.x, inner.y];
  const q: Point = [inner.x + inner.w, inner.y];
  const r: Point = [inner.x + inner.w, inner.y + inner.h];
  const s: Point = [inner.x, inner.y + inner.h];
  pen.polygon([a, b, q, p], tones.top);
  pen.polygon([b, c, r, q], tones.side);
  pen.polygon([d, a, p, s], tones.side);
  pen.polygon([c, d, s, r], tones.floor);
}

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "blue", 0.16);
  const tones = [paint.white, paint.light, paint.deep];
  const shade: Shade[] = [
    { top: [0, 0, 0, 120], side: [0, 0, 0, 65], floor: [255, 255, 255, 60] },
    { top: [0, 0, 0, 120], side: [0, 0, 0, 70], floor: [255, 255, 255, 40] },
  ];
  const ends = [box];
  for (let depth = 1; depth < DEPTHS; depth++) {
    const hole = shrink(ends[depth - 1], 1 / 3);
    const end = shrink(hole, DROP);
    walls(pen, hole, end, shade[depth - 1]);
    ends.push(end);
  }
  const grids = ends.map((end, depth) => new Grid(end, SIDE / 3 ** depth, SIDE / 3 ** depth, 0));
  const counts = [0, 0, 0];
  for (let row = 0; row < SIDE; row++) {
    for (let col = 0; col < SIDE; col++) {
      let depth = 0;
      while (depth + 1 < DEPTHS) {
        const side = SIDE / 3 ** (depth + 1);
        const from = (SIDE - side) / 2;
        if (col < from || row < from || col >= from + side || row >= from + side) break;
        depth++;
      }
      const side = SIDE / 3 ** depth;
      const from = (SIDE - side) / 2;
      if (!lit(DEPTHS - depth, col - from, row - from)) continue;
      counts[depth]++;
      grids[depth].fill(pen, col - from, row - from, tones[depth]);
    }
  }
  if (counts.join() !== WANT.join())
    throw new Error(`zoom: the nested cells light ${counts.join(", ")}, want ${WANT.join(", ")}`);
}
