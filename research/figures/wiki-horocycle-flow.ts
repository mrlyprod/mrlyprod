import type { Color, Ink, Pen, Point } from "mrlyjs/view";
import census from "./census/wiki-horocycle-flow.json" with { type: "json" };

type Cusp = [number, number];
type Canvas = { left: number; bottom: number; scale: number; top: number };

const HEIGHT = 0.01;
const ROOF = 1.6;
const THICK = 1.5;
const EDGE = 2.6;
const STEP = 1.5;
const FLOOR = Math.sqrt(3) / 2;
const TAU = 2 * Math.PI;

export const units = {};

// TRACE

function at(canvas: Canvas, x: number, y: number): Point {
  return [canvas.left + x * canvas.scale, canvas.bottom - (y - FLOOR) * canvas.scale];
}

function inside(canvas: Canvas, x: number, y: number) {
  return Math.abs(x) <= 0.5 && x * x + y * y >= 1 && y <= canvas.top;
}

function trace(pen: Pen, canvas: Canvas, [a, c]: Cusp, tone: Color) {
  const centre = a / c;
  const radius = 1 / (2 * (c * c) * HEIGHT);
  const dt = STEP / (radius * canvas.scale);
  const steps = Math.ceil(TAU / dt);
  let run: Point[] = [];
  let runs = 0;
  for (let k = 0; k <= steps; k++) {
    const t = (k * TAU) / steps - Math.PI / 2;
    const x = centre + radius * Math.cos(t);
    const y = radius + radius * Math.sin(t);
    if (inside(canvas, x, y)) run.push(at(canvas, x, y));
    else if (run.length > 0) {
      pen.polyline(run, THICK, tone);
      run = [];
      runs++;
    }
  }
  if (run.length > 0) {
    pen.polyline(run, THICK, tone);
    runs++;
  }
  return runs;
}

function outline(pen: Pen, canvas: Canvas, tone: Color) {
  pen.segment(at(canvas, -0.5, FLOOR), at(canvas, -0.5, canvas.top), EDGE, tone);
  pen.segment(at(canvas, 0.5, FLOOR), at(canvas, 0.5, canvas.top), EDGE, tone);
  const arc: Point[] = [];
  for (let k = 0; k <= 240; k++) {
    const t = Math.PI * (1 / 3 + k / 720);
    arc.push(at(canvas, Math.cos(t), Math.sin(t)));
  }
  pen.polyline(arc, EDGE, tone);
}

// DRAW

export default function draw(pen: Pen, ink: Ink) {
  const cusps = census.cusps as Cusp[];
  if (cusps.length !== 243) throw new Error(`wiki-horocycle-flow: ${cusps.length} cusps, want 243`);
  if (cusps.some((cusp) => cusp.length !== 2 || !Number.isInteger(cusp[0]) || !Number.isInteger(cusp[1]) || cusp[1] < 1)) throw new Error("wiki-horocycle-flow: a cusp is not a pair a, c with c at least one");
  const frame = pen.frame(0.08);
  const canvas: Canvas = { left: frame.x + frame.w / 2, bottom: frame.y + frame.h, scale: frame.h / ROOF, top: FLOOR + ROOF };
  const deepest = Math.max(...cusps.map(([, c]) => c));
  const ramp = ink.Ramp.tone(ink.blue, ink.pink);
  let drawn = 0;
  for (const cusp of cusps) drawn += trace(pen, canvas, cusp, ramp.at((cusp[1] - 1) / Math.max(deepest - 1, 1)));
  outline(pen, canvas, ink.fg);
  if (drawn !== 110) throw new Error(`wiki-horocycle-flow: ${drawn} runs drawn, want 110`);
}
