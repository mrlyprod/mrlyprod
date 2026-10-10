import * as math from "mrlyjs/math";
import { plot, type Ink, type Pen, type Point } from "mrlyjs/view";

export const units = { math };

const CODE = 126;
const LEVEL = 4;
const WIDE = 4;

function span(points: Point[]): [Point, Point] {
  const lo: Point = [Number.MAX_VALUE, Number.MAX_VALUE];
  const hi: Point = [-Number.MAX_VALUE, -Number.MAX_VALUE];
  for (const [u, v] of points) {
    lo[0] = Math.min(lo[0], u);
    lo[1] = Math.min(lo[1], v);
    hi[0] = Math.max(hi[0], u);
    hi[1] = Math.max(hi[1], v);
  }
  return [lo, hi];
}

let memo: { low: number; high: number; cuts: Point[][]; flat: Point[] } | undefined;

function cuts() {
  if (memo) return memo;
  const counts = math.three.profile(CODE, 2, LEVEL, 2);
  const support = math.three.support(counts);
  if (!support) throw new Error("demo-cuts: the design fills no height");
  const [low, high] = support;
  const cells = 3 ** LEVEL;
  if (low !== 15 || high !== 30) throw new Error(`demo-cuts: support ${low}..${high}, want 15..30`);
  if (high - low + 1 !== WIDE * WIDE) throw new Error(`demo-cuts: ${high - low + 1} heights, want ${WIDE * WIDE}`);
  const found: Point[][] = [];
  for (let height = low; height <= high; height++) {
    if (BigInt(counts[height]) !== BigInt(cells)) throw new Error(`demo-cuts: ${counts[height]} cells at height ${height}, want ${cells}`);
    const points = math.three.diagonal_slice(CODE, 2, LEVEL, 2, height);
    if (points.length !== cells) throw new Error(`demo-cuts: ${points.length} points at height ${height}, want ${cells}`);
    found.push(points.map((point) => math.three.project(point)));
  }
  const flat = found.flat();
  if (flat.length !== WIDE * WIDE * cells) throw new Error(`demo-cuts: ${flat.length} points, want ${WIDE * WIDE * cells}`);
  memo = { low, high, cuts: found, flat };
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const { low, high, cuts: all, flat } = cuts();
  const frame = pen.frame(0.08);
  const stage = frame.panels(WIDE, WIDE, (frame.w / WIDE) * 0.05);
  const [lo, hi] = span(flat);
  const mid = [(lo[0] + hi[0]) / 2, (lo[1] + hi[1]) / 2];
  const scale = Math.min(stage[0].w / (hi[0] - lo[0]), stage[0].h / (hi[1] - lo[1]));
  const middle = [Math.trunc((low + high) / 2), Math.ceil((low + high) / 2)];
  all.forEach((cut, i) => {
    const [cx, cy] = stage[i].center();
    const dots = cut.map(([u, v]): Point => [cx + (u - mid[0]) * scale, cy - (v - mid[1]) * scale]);
    plot.dots(pen, dots, scale * 0.42, middle.includes(low + i) ? ink.yellow : ink.blue);
  });
}
