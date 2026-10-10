import * as math from "mrlyjs/math";
import { Grid, iso, type Frame, type Ink, type Pen, type Point } from "mrlyjs/view";
import census from "./census/wiki-euler-characteristic.json" with { type: "json" };

export const units = { math };

const SIDE = 9;
const HOLES = 9;
const FACES = [
  [4, 5, 7, 6],
  [1, 3, 7, 5],
  [2, 3, 7, 6],
];

// CUBE

function corner(i: number): [number, number, number] {
  return [i & 1, (i >> 1) & 1, (i >> 2) & 1];
}

function cube(pen: Pen, ink: Ink, frame: Frame): [number, number, number] {
  const flat: Point[] = Array.from({ length: 8 }, (_, i) => iso.project(...corner(i)));
  let lo: Point = [Number.MAX_VALUE, Number.MAX_VALUE];
  let hi: Point = [-Number.MAX_VALUE, -Number.MAX_VALUE];
  for (const p of flat) {
    lo = [Math.min(lo[0], p[0]), Math.min(lo[1], p[1])];
    hi = [Math.max(hi[0], p[0]), Math.max(hi[1], p[1])];
  }
  const span = Math.max(hi[0] - lo[0], hi[1] - lo[1]);
  const scale = (Math.min(frame.w, frame.h) * 0.95) / span;
  const [cx, cy] = frame.center();
  const place = (p: Point): Point => [cx + (p[0] - (lo[0] + hi[0]) / 2) * scale, cy + (p[1] - (lo[1] + hi[1]) / 2) * scale];

  const tones = [ink.blue, ink.mix(ink.blue, ink.ground, 0.3), ink.mix(ink.blue, ink.ground, 0.55)];
  FACES.forEach((quad, i) => pen.polygon(quad.map((v) => place(flat[v])), tones[i]));

  let edges = 0;
  for (let a = 0; a < 8; a++) {
    for (let b = a + 1; b < 8; b++) {
      const d = a ^ b;
      if ((d & (d - 1)) !== 0) continue;
      const hidden = a === 0 || b === 0;
      pen.segment(place(flat[a]), place(flat[b]), hidden ? 4 : 7, hidden ? ink.dim : ink.fg);
      edges++;
    }
  }
  for (const p of flat) pen.disc(...place(p), 15, ink.yellow);
  return [8, edges, FACES.length * 2];
}

// HOLES

function holes(pen: Pen, ink: Ink, frame: Frame) {
  const carpet = math.two.carpet(3, 2);
  if (carpet.shape[0] !== SIDE || carpet.shape[1] !== SIDE) throw new Error(`wiki-euler-characteristic: carpet ${carpet.shape}, want ${SIDE} square`);
  const grid = new Grid(frame, SIDE, SIDE, 0);
  grid.paint(pen, carpet, (kind) => (kind !== 0 ? ink.blue : null));

  const boxes = census.holes;
  if (boxes.length !== HOLES) throw new Error(`wiki-euler-characteristic: ${boxes.length} holes, want ${HOLES}`);
  if (math.two.euler(carpet) !== BigInt(1 - boxes.length)) throw new Error(`wiki-euler-characteristic: euler ${math.two.euler(carpet)}, want ${1 - boxes.length}`);
  for (const box of boxes) {
    if (box.length !== 4 || box.some((v) => !Number.isInteger(v) || v < 0 || v >= SIDE)) throw new Error(`wiki-euler-characteristic: hole box ${box} off the carpet`);
    const [r0, c0, r1, c1] = box;
    const [x0, y0] = grid.cell(c0, r0);
    const [x1, y1, w, h] = grid.cell(c1, r1);
    const cx = (x0 + x1 + w) / 2;
    const cy = (y0 + y1 + h) / 2;
    const radius = Math.min(x1 + w - x0, y1 + h - y0) * 0.62;
    pen.ring(cx, cy, radius, 6, ink.orange);
  }
}

// DRAW

export default function draw(pen: Pen, ink: Ink) {
  const area = pen.frame(0.06);
  const panes = area.cols(2);
  const [vertices, edges, faces] = cube(pen, ink, panes[0].square().inset(18));
  if (vertices !== 8 || edges !== 12 || faces !== 6) throw new Error(`wiki-euler-characteristic: cube ${vertices} ${edges} ${faces}, want 8 12 6`);
  if (vertices - edges + faces !== 2) throw new Error(`wiki-euler-characteristic: cube euler ${vertices - edges + faces}, want 2`);
  holes(pen, ink, panes[1].square().inset(18));
}
