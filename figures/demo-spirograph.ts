import * as math from "mrlyjs/math";
import type { Ink, Pen, Point } from "mrlyjs/view";

const CODE = 495;
const SIDE = 3;
const LEVEL = 1;
const BASE = 3;
const RING = 13;
const WHEEL = 5;
const REACH = 1.2;
const SAMPLES = 6000;
const PENCILS = 9;
const CURVES = 9;
const ORBITS = 5;
const MARGIN = 0.08;
const THICK = 2.2;
const HAIR = 1.5;

export const units = { math };

type Roulette = { pens: ReturnType<typeof math.spirograph.pencils>; points: Float32Array; box: Float64Array };

let memo: Roulette | undefined;

function roulette() {
  if (memo) return memo;
  const cell = math.two.create(CODE, SIDE, LEVEL, 0, BASE);
  const path = math.spirograph.track("in", RING, WHEEL, 4, 1);
  const pens = math.spirograph.pencils(cell.types, cell.shape[1], cell.shape[0], "both", REACH, 0, 1);
  if (pens.length !== PENCILS) throw new Error(`demo-spirograph: ${pens.length} pencils, want ${PENCILS}`);
  const curves = math.spirograph.distinct(path, pens, true);
  if (curves !== CURVES) throw new Error(`demo-spirograph: ${curves} curves, want ${CURVES}`);
  if (path.orbits !== ORBITS) throw new Error(`demo-spirograph: ${path.orbits} orbits, want ${ORBITS}`);
  const points = math.spirograph.trace(path, pens, SAMPLES);
  if (points.length !== 2 * PENCILS * SAMPLES) throw new Error(`demo-spirograph: ${points.length} coordinates, want ${2 * PENCILS * SAMPLES}`);
  memo = { pens, points, box: math.spirograph.frame(path, pens) };
  return memo;
}

export default function draw(pen: Pen, ink: Ink) {
  const { pens, points, box } = roulette();
  const [x0, y0, x1, y1] = box;
  const area = pen.frame(MARGIN);
  const scale = Math.min(area.w / (x1 - x0), area.h / (y1 - y0));
  const [cx, cy] = area.center();
  const at = (x: number, y: number): Point => [cx + (x - (x0 + x1) / 2) * scale, cy - (y - (y0 + y1) / 2) * scale];
  pen.ring(cx, cy, RING * scale, HAIR, ink.dim);
  pens.forEach((pencil, k) => {
    const pts: Point[] = [];
    for (let i = 0; i < SAMPLES; i++) {
      const j = 2 * (k * SAMPLES + i);
      pts.push(at(points[j], points[j + 1]));
    }
    pen.polyline(pts, THICK, pencil.kind === "Void" ? ink.orange : ink.blue);
  });
}
