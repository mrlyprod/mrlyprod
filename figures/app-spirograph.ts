import * as math from "mrlyjs/math";
import type { Ink, Pen, Point } from "mrlyjs/view";

export const units = { math };

const CODE = 495;
const NUMBER = 3;
const LEVEL = 1;
const BASE = 3;
const RING = 7;
const WHEEL = 3;
const REACH = 0.9;
const SAMPLES = 420;
const MARGIN = 0.08;
const THICK = 5;
const BANDS = 3;
const WANT = { pencils: 8, curves: 8, nodes: 1288n };

type Plan = { track: ReturnType<typeof math.spirograph.track>; pencils: ReturnType<typeof math.spirograph.pencils>; trace: Float32Array; frame: number[]; bands: number[]; cell: number; types: ArrayLike<number>; width: number; height: number };

let plan: Plan | null = null;

function study(): Plan {
  if (plan) return plan;
  const s = math.spirograph;
  const design = math.two.create(CODE, NUMBER, LEVEL, 0, BASE);
  const [height, width] = design.shape;
  const track = s.track("in", RING, WHEEL, 4, 1);
  const pencils = s.pencils(design.types, width, height, "fill", REACH, 0, 1);
  const curves = s.distinct(track, pencils, true);
  const nodes = s.nodes(track, pencils, true);
  if (pencils.length !== WANT.pencils || curves !== WANT.curves || nodes !== WANT.nodes) throw new Error(`app-spirograph: ${pencils.length} pencils, ${curves} curves, ${nodes} nodes, want ${WANT.pencils}, ${WANT.curves}, ${WANT.nodes}`);
  const radii = pencils.map((p) => Math.hypot(p.x, p.y));
  const lo = Math.min(...radii);
  const hi = Math.max(...radii);
  const bands = radii.map((r) => (hi > lo ? Math.min(BANDS - 1, Math.floor(((r - lo) / (hi - lo)) * BANDS)) : BANDS - 1));
  plan = { track, pencils, trace: s.trace(track, pencils, SAMPLES), frame: Array.from(s.frame(track, pencils)), bands, cell: s.cell(width, height, REACH), types: design.types, width, height };
  return plan;
}

export default function draw(pen: Pen, ink: Ink) {
  const { track, pencils, trace, frame, bands, cell, types, width, height } = study();
  const box = pen.frame(MARGIN);
  const [x0, y0, x1, y1] = frame;
  const k = Math.min(box.w / (x1 - x0), box.h / (y1 - y0));
  const ox = box.x + box.w / 2 - ((x0 + x1) / 2) * k;
  const oy = box.y + box.h / 2 + ((y0 + y1) / 2) * k;
  const X = (x: number) => ox + x * k;
  const Y = (y: number) => oy - y * k;
  const shades = Array.from({ length: BANDS }, (_, b) => ink.mix(ink.ground, ink.blue, 0.5 + (0.5 * b) / (BANDS - 1)));
  pen.ring(X(0), Y(0), RING * k, 2, ink.line);
  for (let band = 0; band < BANDS; band++) {
    pencils.forEach((_, p) => {
      if (bands[p] !== band) return;
      const pts: Point[] = [];
      for (let i = 0; i < SAMPLES; i++) pts.push([X(trace[(p * SAMPLES + i) * 2]), Y(trace[(p * SAMPLES + i) * 2 + 1])]);
      pen.polyline(pts, THICK, shades[band]);
    });
  }
  const s = track.total;
  const [cx, cy] = math.spirograph.pose(track, s);
  const phi = math.spirograph.turn(track, s);
  const r = WHEEL * k;
  pen.ring(X(cx), Y(cy), r, 3, ink.dim);
  const unit = cell * r;
  const c = Math.cos(phi);
  const sn = Math.sin(phi);
  const turn = (u: number, v: number): Point => [X(cx) + u * c - v * sn, Y(cy) - (u * sn + v * c)];
  const inset = unit * 0.12;
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      if (!types[row * width + col]) continue;
      const u0 = (col - width / 2) * unit + inset;
      const v0 = (height / 2 - row) * unit - inset;
      const u1 = u0 + unit - 2 * inset;
      const v1 = v0 - unit + 2 * inset;
      pen.polygon([turn(u0, v0), turn(u1, v0), turn(u1, v1), turn(u0, v1)], ink.fg);
    }
  }
  pencils.forEach((_, p) => pen.disc(X(trace[(p * SAMPLES + SAMPLES - 1) * 2]), Y(trace[(p * SAMPLES + SAMPLES - 1) * 2 + 1]), 7, shades[bands[p]]));
  pen.disc(X(cx), Y(cy), 5, ink.fg);
}
