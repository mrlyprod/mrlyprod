import * as math from "@mrlyprod/mrly/math";
import { plate } from "../../lib/icon.js";

export const units = { math };

const LEVEL = 10;
const DRAW = 12;
const EDGE = 0.01;
const WANT = { fill: 1024, distinct: 1024, code: "3", base: "1,1" };

let memo = null;

function build() {
  if (memo) return memo;
  const design = math.radix.twindragon();
  const got = { fill: Number(design.fill(LEVEL)), distinct: design.distinct(LEVEL), code: design.code(), base: design.base().value().map(Number).join() };
  if (Object.values(got).join() !== Object.values(WANT).join()) throw new Error(`radix: the twindragon reads ${Object.values(got).join()}, want ${Object.values(WANT).join()}`);
  if (LEVEL % 2 || DRAW % 2) throw new Error("radix: an odd level turns the cells off the axes");
  const pts = design.plane(DRAW);
  if (pts.length !== 2 ** DRAW || design.distinct(DRAW) !== 2 ** DRAW) throw new Error(`radix: level ${DRAW} draws ${pts.length} cells, want ${2 ** DRAW}`);
  const side = 2 ** (-DRAW / 2);
  const box = [Infinity, Infinity, -Infinity, -Infinity];
  for (const [x, y] of pts) {
    box[0] = Math.min(box[0], x);
    box[1] = Math.min(box[1], y);
    box[2] = Math.max(box[2], x);
    box[3] = Math.max(box[3], y);
  }
  const key = ([x, y]) => `${Math.round((x - box[0]) / side)},${Math.round((y - box[1]) / side)}`;
  const near = [-1, 0, 1].flatMap((dx) => [-1, 0, 1].map((dy) => [dx * side, dy * side]));
  const half = pts.length / 2;
  const dragons = [pts.slice(half), pts.slice(0, half)].map((cells) => {
    const lit = new Set(cells.map(key));
    return { cells, rim: cells.filter(([x, y]) => near.some(([dx, dy]) => !lit.has(key([x + dx, y + dy])))) };
  });
  memo = { dragons, box, side };
  return memo;
}

export default function draw(pen) {
  const { box: frame, paint, size } = plate(pen, "orange", 0.16);
  const { dragons, box, side } = build();
  const [x0, y0, x1, y1] = box;
  const k = Math.min(frame.w / (x1 - x0 + side), frame.h / (y1 - y0 + side));
  const [cx, cy] = frame.center();
  const mx = (x0 + x1) / 2;
  const my = (y0 + y1) / 2;
  const h = (side * k) / 2 + 0.5;
  const e = size * EDGE;
  const at = ([x, y]) => [cx + (x - mx) * k, cy - (y - my) * k];
  for (const { cells, rim } of dragons) {
    for (const p of rim) {
      const [px, py] = at(p);
      pen.round_rect(px - h - e, py - h - e, 2 * (h + e), 2 * (h + e), e, paint.light);
    }
    for (const p of cells) {
      const [px, py] = at(p);
      pen.rect(px - h, py - h, 2 * h, 2 * h, paint.white);
    }
  }
}
