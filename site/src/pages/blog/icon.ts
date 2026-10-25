import type { Color, Pen, Point } from "#view";
import { plate } from "../../lib/plate.ts";

export const units = {};

const LENGTH = 1.2;
const HALF = 0.11;
const TIP = 0.26;
const LEAD = 0.4;
const BAND = 0.07;
const LINE = 0.065;
const LIFT = 0.05;
const SWEEP = 0.66;
const SAG = 0.1;
const STEPS = 24;
const K = Math.SQRT1_2;

export default function draw(pen: Pen) {
  const { box, paint } = plate(pen, "yellow", 0.16);
  const w = box.w;
  const [bx, by] = box.center();
  const cx = bx + LIFT * w * K;
  const cy = by - LIFT * w * K;
  const half = HALF * w;
  const span = (LENGTH / 2) * w;
  const reach = span * K;
  if (reach > w / 2)
    throw new Error(`blog: the pencil reaches ${reach.toFixed(0)} from centre, the box allows ${(w / 2).toFixed(0)}`);
  const at = (t: number, v: number): Point => [cx + (t + v) * K, cy + (v - t) * K];
  const t0 = -span + TIP * w;
  const cap = span - half;
  const facet: Color = [paint.light[0], paint.light[1], paint.light[2], 110];
  pen.polygon([at(t0, -half), at(cap, -half), at(cap, half), at(t0, half)], paint.white);
  pen.disc(...at(cap, 0), half, paint.white);
  pen.polygon([at(t0, half / 3), at(cap, half / 3), at(cap, half), at(t0, half)], facet);
  pen.polygon(
    [at(cap - BAND * w * 2, -half), at(cap - BAND * w, -half), at(cap - BAND * w, half), at(cap - BAND * w * 2, half)],
    paint.light,
  );
  pen.polygon([at(t0, -half), at(t0, half), at(-span, 0)], paint.deep);
  pen.polygon(
    [at(-span + TIP * w * LEAD, -half * LEAD), at(-span + TIP * w * LEAD, half * LEAD), at(-span, 0)],
    paint.ink,
  );
  const [tx, ty] = at(-span, 0);
  const line: Point[] = Array.from({ length: STEPS + 1 }, (_, i) => {
    const s = i / STEPS;
    return [tx + s * SWEEP * w, ty + SAG * w * Math.sin(Math.PI * s)];
  });
  pen.polyline(line, LINE * w, paint.light);
}
