import * as num from "mrlyjs/num";
import type { Ink, Pen } from "mrlyjs/view";

export const units = { num };

const LEVEL = 10;
const MARGIN = 0.08;
const GAP = 0.12;
const HALF = 0.55;
const WANT = { fill: 1024, distinct: 1024, code: "3", base: "1,1" };

let memo: { pts: [number, number][]; box: [number, number, number, number] } | null = null;

function build() {
  if (memo) return memo;
  const design = num.radix.twindragon();
  const got = { fill: Number(design.fill(LEVEL)), distinct: design.distinct(LEVEL), code: design.code(), base: design.base().value().map(Number).join() };
  if (Object.values(got).join() !== Object.values(WANT).join()) throw new Error(`app-radix: the twindragon reads ${Object.values(got).join()}, want ${Object.values(WANT).join()}`);
  if (LEVEL % 2) throw new Error("app-radix: an odd level turns the cells off the axes");
  const pts = design.plane(LEVEL);
  const box: [number, number, number, number] = [Infinity, Infinity, -Infinity, -Infinity];
  for (const [x, y] of pts) {
    box[0] = Math.min(box[0], x);
    box[1] = Math.min(box[1], y);
    box[2] = Math.max(box[2], x);
    box[3] = Math.max(box[3], y);
  }
  memo = { pts, box };
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const { pts, box } = build();
  const frame = pen.frame(MARGIN);
  const side = 2 ** (-LEVEL / 2);
  const [x0, y0, x1, y1] = box;
  const k = Math.min(frame.w / (x1 - x0 + side), frame.h / (y1 - y0 + side));
  const cx = frame.x + frame.w / 2;
  const cy = frame.y + frame.h / 2;
  const mx = (x0 + x1) / 2;
  const my = (y0 + y1) / 2;
  const s = side * k;
  const h = (s * (1 - GAP)) / 2;
  const tones = [ink.blue, ink.mix(ink.ground, ink.blue, HALF)];
  const half = pts.length / 2;
  pts.forEach(([x, y], i) => {
    const px = cx + (x - mx) * k;
    const py = cy - (y - my) * k;
    pen.rect(px - h, py - h, 2 * h, 2 * h, tones[i < half ? 0 : 1]);
  });
}
