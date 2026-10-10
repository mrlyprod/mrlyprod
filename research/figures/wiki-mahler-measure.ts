import { field, type Ink, type Pen, type Point } from "mrlyjs/view";
import census from "./census/wiki-mahler-measure.json" with { type: "json" };

type Census = { heights: number[]; on: [number, number][]; on_heights: number[]; off: [number, number][] };

const TAU = 2 * Math.PI;
const SAMPLES = 4096;
const STRETCH = 0.34;
const FLOOR = 2.0;

export const units = {};

// CURVE

function squeeze(v: number) {
  return v >= 0 ? v : -FLOOR * (1 - Math.exp(v / FLOOR));
}

function turn(a: number) {
  const r = a % TAU;
  return r < 0 ? r + TAU : r;
}

// DRAW

export default function draw(pen: Pen, ink: Ink) {
  const { heights, on, on_heights, off } = census as Census;
  if (heights.length !== SAMPLES) throw new Error(`wiki-mahler-measure: ${heights.length} heights, want ${SAMPLES}`);
  if (on_heights.length !== on.length) throw new Error(`wiki-mahler-measure: ${on_heights.length} heights for ${on.length} roots on the circle`);
  if (off.length !== 2) throw new Error(`wiki-mahler-measure: ${off.length} roots off the circle, want 2`);

  const reach = heights.map((v) => 1 + STRETCH * squeeze(v));
  const rows: [number, number][] = heights.map((v, i) => [(i / SAMPLES) * TAU, v]);
  on.forEach(([re, im], k) => rows.push([turn(Math.atan2(im, re)), on_heights[k]]));
  rows.sort((a, b) => a[0] - b[0]);
  if (rows.length !== SAMPLES + 8) throw new Error(`wiki-mahler-measure: ${rows.length} angles, want ${SAMPLES + 8}`);
  const levels = rows.map(([, v]) => v);
  const plane = rows.map(([t, v]): Point => {
    const r = 1 + STRETCH * squeeze(v);
    return [r * Math.cos(t), r * Math.sin(t)];
  });
  let x0 = Number.MAX_VALUE;
  let x1 = -Number.MAX_VALUE;
  let y0 = Number.MAX_VALUE;
  let y1 = -Number.MAX_VALUE;
  for (const [x, y] of plane) {
    x0 = Math.min(x0, x);
    x1 = Math.max(x1, x);
    y0 = Math.min(y0, y);
    y1 = Math.max(y1, y);
  }

  const frame = pen.frame(0.08);
  const scale = frame.w / Math.max(x1 - x0, y1 - y0);
  const [fx, fy] = frame.center();
  const cx = fx - (scale * (x0 + x1)) / 2;
  const cy = fy + (scale * (y0 + y1)) / 2;
  const at = (x: number, y: number): Point => [cx + scale * x, cy - scale * y];

  const out = ink.fade(ink.blue, 0.22);
  const inn = ink.fade(ink.orange, 0.22);
  const [bx0, by0] = at(x0, y1);
  const [bx1, by1] = at(x1, y0);
  const left = Math.max(Math.floor(bx0), 0);
  const top = Math.max(Math.floor(by0), 0);
  const right = Math.min(Math.ceil(bx1), pen.width);
  const bottom = Math.min(Math.ceil(by1), pen.height);
  const bands = field.patch(left, top, right - left, bottom - top);
  for (let py = top; py < bottom; py++) {
    for (let px = left; px < right; px++) {
      const x = (px + 0.5 - cx) / scale;
      const y = (cy - py - 0.5) / scale;
      const rho = Math.hypot(x, y);
      const u = (turn(Math.atan2(y, x)) / TAU) * SAMPLES;
      const i = Math.floor(u) % SAMPLES;
      const w = u - Math.floor(u);
      const r = reach[i] * (1 - w) + reach[(i + 1) % SAMPLES] * w;
      if (rho > 1 && rho < r) bands.blend(px, py, out);
      else if (rho < 1 && rho > r) bands.blend(px, py, inn);
    }
  }
  bands.paint(pen);

  pen.ring(cx, cy, scale, 3, ink.dim);
  let run: Point[] = [];
  let up = levels[0] >= 0;
  for (let i = 0; i <= rows.length; i++) {
    const k = i % rows.length;
    const p = at(plane[k][0], plane[k][1]);
    const side = levels[k] >= 0;
    if (side !== up) {
      run.push(p);
      pen.polyline(run, 5, up ? ink.blue : ink.orange);
      run = [];
      up = side;
    }
    run.push(p);
  }
  pen.polyline(run, 5, up ? ink.blue : ink.orange);

  for (const [re, im] of on) pen.disc(...at(re, im), 8, ink.fg);
  for (const [re, im] of off) pen.disc(...at(re, im), 13, ink.yellow);
}
