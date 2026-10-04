import type { Ink, Pen } from "mrlyjs/view";
import census from "./census/wiki-random-walk.json" with { type: "json" };

const SIDE = 64;
const STEPS = 1024;
const HOME = SIDE / 2;

export const units = {};

export default function draw(pen: Pen, ink: Ink) {
  const { typical, trace } = census as { typical: number; trace: [number, number][] };
  if (trace.length !== STEPS + 1) throw new Error(`wiki-random-walk: ${trace.length} points, want ${STEPS + 1}`);
  if (trace[0][0] !== HOME || trace[0][1] !== HOME) throw new Error(`wiki-random-walk: starts at ${trace[0]}, want ${HOME}, ${HOME}`);
  for (const [x, y] of trace) {
    if (!Number.isInteger(x) || !Number.isInteger(y) || x < 0 || x >= SIDE || y < 0 || y >= SIDE) throw new Error(`wiki-random-walk: point ${x}, ${y} off the board`);
  }
  const frame = pen.frame(0.08);
  const unit = frame.w / SIDE;
  const spot = (x: number, y: number): [number, number] => [frame.x + (x + 0.5) * unit, frame.y + (y + 0.5) * unit];
  const faint = ink.fade(ink.line, 0.3);
  for (let k = 0; k <= SIDE; k++) {
    const at = frame.x + k * unit;
    const down = frame.y + k * unit;
    pen.segment([at, frame.y], [at, frame.y + frame.h], 1.0, faint);
    pen.segment([frame.x, down], [frame.x + frame.w, down], 1.0, faint);
  }
  const [hx, hy] = spot(HOME, HOME);
  pen.ring(hx, hy, typical * unit, unit * 0.22, ink.fade(ink.dim, 0.9));
  pen.polyline(trace.map(([x, y]) => spot(x, y)), unit * 0.34, ink.fade(ink.blue, 0.85));
  pen.disc(hx, hy, unit * 0.9, ink.yellow);
  const [lx, ly] = trace[trace.length - 1];
  const [ex, ey] = spot(lx, ly);
  pen.disc(ex, ey, unit * 0.9, ink.orange);
}
